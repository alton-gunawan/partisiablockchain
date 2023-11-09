//! Defines logic for handling the `#[init]` attribute.

use proc_macro::TokenStream;

use crate::macro_abi::{make_hook_abi_fn, make_hook_abi_fn_delegator};
use crate::{
    determine_names, variables_for_inner_call, wrap_function_for_export, FnKindCallProtocol,
    SecretInput, TokenStream2, WrappedFunctionKind,
};

/// Defines logic for handling the `#[init]` attribute.
pub fn handle_init_macro(input: TokenStream, zk_argument: bool) -> TokenStream {
    let fn_ast: syn::ItemFn = syn::parse(input.clone()).unwrap();
    let names = determine_names(None, &fn_ast, "init", false);

    let invocation = variables_for_inner_call(&fn_ast, FnKindCallProtocol::Init, zk_argument);

    let docs = format!(
        "Serialization wrapper for contract init `{}`.",
        names.fn_identifier
    );

    let kind = WrappedFunctionKind::public_contract_hook_kind(
        1,
        pbc_contract_common::FunctionKind::Init,
        zk_argument,
    );

    let mut result = wrap_function_for_export(
        &names.fn_identifier,
        names.export_symbol,
        &docs,
        invocation,
        &kind,
        None,
    );

    let abi_fn_name = format_ident!("__abi_fn_{}", &names.fn_identifier);
    let abi_fn = {
        let rpc_pos = kind.system_arguments;
        let shortname_ident = quote! { Some(pbc_contract_common::fn_init_shortname()) };

        make_hook_abi_fn(
            &fn_ast,
            &abi_fn_name,
            kind.fn_kind,
            rpc_pos,
            shortname_ident,
            SecretInput::None,
        )
    };

    let zk_constant = create_zk_constant(zk_argument);
    result.extend(zk_constant);

    let stamped_versions = crate::version::create_version_numbers(zk_argument);

    result.extend(stamped_versions);

    result.extend(TokenStream2::from(input));
    result.extend(abi_fn);
    result.extend(make_hook_abi_fn_delegator(&abi_fn_name));
    result.into()
}

fn create_zk_constant(is_zk: bool) -> TokenStream2 {
    quote! {
        #[doc = "Constant denoting whether the contract is a zk contract or not."]
        #[no_mangle]
        pub const __PBC_IS_ZK_CONTRACT: bool = #is_zk;
    }
}
