//! This submodule handles the creation of easy-to-use zk-computation starter methods.

use crate::quote::ToTokens;
use crate::{determine_parameter_type, determine_return_types_from_output, TokenStream2};
use pbc_contract_common::address::Shortname;
use proc_macro::TokenStream;
use syn::Ident;

/// Creates the easy-to-use zk-computation starter function.
pub fn handle_zk_compute_macro(input: TokenStream, shortname: Shortname) -> TokenStream {
    let fn_ast: syn::ItemFn = syn::parse(input).unwrap();

    let fn_identifier: &Ident = &fn_ast.sig.ident;

    let fn_id_computation_starter: Ident = format_ident!("{}_start", fn_identifier);
    let fn_id_insecure_rust_version: Ident = format_ident!("{}", fn_identifier);

    let parameters: Vec<_> = fn_ast
        .sig
        .inputs
        .iter()
        .map(determine_parameter_type)
        .collect();

    let parameter_names: Vec<TokenStream2> = parameters
        .iter()
        .map(|token| token.pat.to_token_stream())
        .collect();
    let parameter_types: Vec<_> = parameters
        .iter()
        .map(|token| token.ty.to_token_stream())
        .collect();

    let docs_computation_starter = format!("Constructor for [`pbc_contract_common::zk::ZkStateChange`] for starting the `{fn_identifier}` computation");
    let docs_rust_version_additional =  format!("Insecure Rust version of the `{fn_identifier}` computation.\nShould be used exclusively for computation testing.\nUse [`{fn_id_computation_starter}`] to initialize the computation!\n\n# Original Documentation");

    let num_return_items = determine_return_types_from_output(&fn_ast.sig.output).len();

    let mut result = wrap_zk_compute_creator(
        &fn_id_computation_starter,
        &shortname,
        &parameter_names,
        &parameter_types,
        &docs_computation_starter,
        Some(num_return_items),
    );

    let mut fn_ast: syn::ItemFn = fn_ast;
    fn_ast.sig.ident = fn_id_insecure_rust_version;
    fn_ast.vis = syn::Visibility::Inherited; // Privatize the Rust version to restrict to tests.

    let hidden_inner_implementation = quote! {
        #[allow(unused)]
        #[doc = #docs_rust_version_additional]
        #fn_ast
    };

    result.extend(hidden_inner_implementation);
    result.into()
}

fn metadata_parameter_and_serialization(
    metadata_type: &TokenStream2,
    num_expected_outputs: Option<usize>,
) -> (TokenStream2, Option<TokenStream2>, TokenStream2) {
    let metadata_generic = quote! { <#metadata_type: pbc_traits::ReadWriteState> };
    match num_expected_outputs {
        // Do not produce output metadata parameters when we expect no outputs.
        Some(0) => (quote! {}, None, quote! {}),
        // Expect single argument.
        Some(1) => (
            quote! { output_metadata: &#metadata_type, },
            Some(quote! { [output_metadata] }),
            metadata_generic,
        ),

        // Expect array of arguments.
        Some(n) => (
            quote! { output_metadata: [&#metadata_type; #n], },
            Some(quote! { output_metadata }),
            metadata_generic,
        ),

        // Expect slice of arguments.
        None => (
            quote! { output_metadata: &[&#metadata_type], },
            Some(quote! { output_metadata }),
            metadata_generic,
        ),
    }
}

fn wrap_zk_compute_creator(
    rust_visible_symbol: &Ident,
    shortname: &Shortname,
    parameter_names: &[TokenStream2],
    parameter_types: &[TokenStream2],
    docs: &str,
    num_return_items: Option<usize>,
) -> TokenStream2 {
    let shortname_u32 = shortname.as_u32();

    let metadata_type = quote! { T };
    let (metadata_parameter, metadata_serialization, metadata_generic) =
        metadata_parameter_and_serialization(&metadata_type, num_return_items);

    let metadata_serialization_complete =
        if let Some(metadata_serialization) = metadata_serialization {
            quote! {
                #metadata_serialization.iter().map(|arg| {
                    let mut buf: Vec<u8> = Vec::new();
                    arg.state_write_to(&mut buf).unwrap();
                    buf
                }).collect()
            }
        } else {
            quote! { vec![] }
        };

    quote! {
        #[doc = #docs]
        #[automatically_derived]
        pub(crate) fn #rust_visible_symbol #metadata_generic(
            #(#parameter_names : #parameter_types,)*
            #metadata_parameter
        ) -> pbc_contract_common::zk::ZkStateChange {
            // Serialize metadata output
            let output_variable_metadata: Vec<Vec<u8>> = #metadata_serialization_complete;

            // Serialize input arguments
            let input_arguments: Vec<Vec<u8>> = vec![
                #(
                    {
                        let mut buf: Vec<u8> = Vec::new();
                        #parameter_names.secret_write_to(&mut buf).unwrap();
                        buf
                    },
                )*
            ];

            // Create StartComputation output struct
            pbc_contract_common::zk::ZkStateChange::StartComputation {
                function_shortname: pbc_contract_common::shortname::ShortnameZkComputation::from_u32(#shortname_u32),
                output_variable_metadata,
                input_arguments,
            }
        }
    }
}
