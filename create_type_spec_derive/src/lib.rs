//! Internal Partisia Blockchain SDK crate with derive logic for `CreateTypeSpec` trait.
//!
//! *This module is only used during `"ABI"` construction.*

extern crate derive_commons;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::ToTokens;

use derive_commons::{extract_enum_variant_data, has_unique_elements};
use syn::__private::TokenStream2;
use syn::{Data, DataEnum, DataStruct, Fields, Ident};
use uuid::Uuid;

/// Derive the `CreateTypeSpec` trait for structs and enums.
///
/// Warning: When deriving `CreateTypeSpec` for generic arguments you must use
/// `create_type_spec_for_generic` to mark all instantiations of the generic data structure. This
/// is due to to some inconsistencies in the ABI generation. If you forget to do this, your
/// contract ABI might become corrupted, making it difficult for the browser and other blockchain
/// explorers to display and interact with your contract.
#[proc_macro_derive(CreateTypeSpec, attributes(discriminant))]
pub fn create_type_spec(input: TokenStream) -> TokenStream {
    // Parse the AST
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_create_type_spec(ast, syn::parse_quote! { pbc_contract_common::abi });

    // Return the generated impl
    gen.into()
}

/// Derive the `CreateTypeSpec` trait for structs and enum, with an internal view; for use in
/// `pbc_contract_common`.
#[doc(hidden)]
#[proc_macro_derive(CreateTypeSpecInternal)]
pub fn create_type_spec_internal(input: TokenStream) -> TokenStream {
    // Parse the AST
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_create_type_spec(ast, syn::parse_quote! { pbc_contract_core::abi });

    // Return the generated impl
    gen.into()
}

/// Macro required for using generics with `CreateTypeSpec`.
///
/// The ABI generator is currently incapable of performing monomorphization automatically for the
/// relevant types in the state, and needs some help with detecting which types are used. For all
/// instantiations of a generic type, you need to use this macro for that instantiation.
///
/// Usage:
///
/// ```ignore
/// #[derive(ReadWriteState, ReadWriteRPC, CreateTypeSpec, Debug)]
/// pub enum MyOption<T> {
///     #[discriminant(0)]
///     Some { value: T },
///     #[discriminant(1)]
///     None {},
/// }
///
/// #[state]
/// pub struct ContractState {
///     pub some_value: MyOption<u32>,
/// }
///
/// create_type_spec_for_generic! { MyOption<u32> }
/// ```
#[proc_macro]
pub fn create_type_spec_for_generic(input: TokenStream) -> TokenStream {
    // Parse the AST
    let ast_type: syn::TypePath = syn::parse(input).unwrap();
    let derp = create_type_spec_extern_c(&ast_type);
    derp.into()
}

fn impl_create_type_spec(
    ast: syn::DeriveInput,
    abi_module_prefix: syn::Path,
) -> proc_macro2::TokenStream {
    let type_name = &ast.ident;
    let trait_name: syn::Path = syn::parse_quote! { pbc_traits::CreateTypeSpec };

    // Generics
    let mut generics = ast.generics;
    derive_commons::extend_generic_bounds_with_trait(&mut generics, &trait_name);

    let uuid = Uuid::new_v4();
    let type_id: String = uuid.hyphenated().to_string();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let ast_type: syn::TypePath = syn::parse(quote! { #type_name #ty_generics }.into()).unwrap();
    let abi_for_type_function_name = abi_for_type_fn_name(&ast_type);
    let create_type_spec_extern_c = if generics.params.is_empty() {
        create_type_spec_extern_c(&ast_type)
    } else {
        quote! {}
    };

    let create_type_spec_impl = create_type_spec_impl(type_name, type_id, &trait_name, &generics);

    let abi_for_type_function_name_body = match ast.data {
        Data::Struct(ref data_struct) => {
            let (field_names, field_types) = data_to_field_types(data_struct);
            let field_names_ts = field_names.iter().map(|x| x.to_token_stream()).collect();
            create_struct_type_spec_body(&field_names_ts, &field_types, &abi_module_prefix)
        }
        Data::Enum(ref data_enum) => create_enum_type_spec_body(data_enum, &abi_module_prefix),
        _ => panic!(
            "CreateTypeSpec derive does not support Union, currently only structs with named \
        fields and explicitly discriminated enums consisting of struct variants"
        ),
    };

    quote! {
        #create_type_spec_impl

        #[cfg(feature = "abi")]
        #[automatically_derived]
        #[doc = "PBC ABI gen internal method. Ensures that [`"]
        #[doc = stringify!(#type_name #ty_generics)]
        #[doc = "`] is visible to the ABI generator."]
        #[doc = "This is a method used by the PBC ABI-gen system for generating ABI (machine-readable descriptions of smart contracts.)"]
        pub fn #abi_for_type_function_name #impl_generics (named_types: &std::collections::BTreeMap<String, u8>) -> Vec<#abi_module_prefix::NamedTypeSpec> #where_clause {
            let type_id: String = <#type_name #ty_generics as #trait_name> :: __ty_identifier();
            let type_name: String = <#type_name #ty_generics as #trait_name> :: __ty_name();
            let mut named_types_in_fn = vec![];
            let mut type_spec: Vec<u8> = vec![];
            <#type_name #ty_generics as #trait_name> :: __ty_spec_write(&mut type_spec, named_types);

            #abi_for_type_function_name_body
            named_types_in_fn
        }

        #create_type_spec_extern_c
    }
}

fn create_enum_type_spec_body(
    data_enum: &DataEnum,
    abi_module_prefix: &syn::Path,
) -> proc_macro2::TokenStream {
    let (variant_discriminants, variant_names, variant_fields) = data_to_variants(data_enum);
    let variant_names_string: Vec<String> = variant_names.iter().map(|n| n.to_string()).collect();
    let variant_names_lowercase: Vec<Ident> = variant_names_string
        .iter()
        .map(|v| format_ident!("{}", v.to_lowercase()))
        .collect();
    let struct_variant_names: Vec<Ident> = variant_names_lowercase
        .iter()
        .map(|n| format_ident!("struct_{}", n))
        .collect();

    let variant_type_uuids: Vec<String> = variant_names
        .iter()
        .map(|_| Uuid::new_v4().hyphenated().to_string())
        .collect();
    let variant_type_ids: Vec<Ident> = variant_names_lowercase
        .iter()
        .map(|n| format_ident!("type_id_{}", n))
        .collect();
    let variant_type_specs: Vec<Ident> = variant_names_lowercase
        .iter()
        .map(|n| format_ident!("type_spec_{}", n))
        .collect();
    let calculate_variant_type_specs = quote! {
        #(
            let mut #variant_type_specs = vec![];
            let #variant_type_ids: String = format!("{}{}", type_id, #variant_type_uuids);
            let type_index: u8 = *named_types.get(&#variant_type_ids).unwrap_or(&0xFF) ;

            #variant_type_specs.push(0x00);
            #variant_type_specs.push(type_index);
        )*
    };

    let add_fields_to_variants: Vec<TokenStream2> = variant_fields.iter().zip(struct_variant_names.iter()).map(|(variant_field, struct_name)| {
        let variant_field_names = &variant_field.1;
        let variant_field_string_names: Vec<String> = variant_field_names
            .iter().map(|name| name.to_string().to_lowercase()).collect();
        let variant_field_types = &variant_field.0;
        quote! {
            #(
                let #variant_field_names = #abi_module_prefix::NamedEntityAbi::new::<#variant_field_types>(
                    #variant_field_string_names.to_string(), named_types,
                );
                #struct_name.add_field(#variant_field_names);
            )*
        }
    }).collect();

    quote! {

        #calculate_variant_type_specs

        let mut type_spec = #abi_module_prefix::NamedTypeSpec::new_enum(
            type_name,
            type_id, type_spec,
        );

        #(
            let #variant_names_lowercase = #abi_module_prefix::EnumVariant::new(
                #variant_discriminants, #variant_type_specs.clone(),
            );
            type_spec.add_variant(#variant_names_lowercase);
        )*
        named_types_in_fn.push(type_spec);

        #(
            let mut #struct_variant_names = #abi_module_prefix::NamedTypeSpec::new_struct(
                #variant_names_string.to_string(),
                #variant_type_ids,
                #variant_type_specs,
            );
            #add_fields_to_variants
            named_types_in_fn.push(#struct_variant_names);
        )*
    }
}

fn create_struct_type_spec_body(
    field_names: &Vec<TokenStream2>,
    field_types: &Vec<TokenStream2>,
    abi_module_prefix: &syn::Path,
) -> proc_macro2::TokenStream {
    let fields_string_names: Vec<String> = field_names
        .iter()
        .map(|name| -> String { name.to_string().to_lowercase() })
        .collect();

    quote! {

        let mut type_abi = #abi_module_prefix::NamedTypeSpec::new_struct(
            type_name,
            type_id,
            type_spec,
        );
        #(
            let #field_names = #abi_module_prefix::NamedEntityAbi::new::<#field_types>(
                #fields_string_names.to_string(),
                named_types,
            );
            type_abi.add_field(#field_names);
        )*

        named_types_in_fn.push(type_abi);
    }
}

type VariantInfo<'a> = (
    Vec<Literal>,
    Vec<&'a Ident>,
    Vec<(Vec<TokenStream2>, Vec<TokenStream2>)>,
);

fn data_to_variants(data: &DataEnum) -> VariantInfo {
    let mut discriminants: Vec<Literal> = vec![];
    let mut idents: Vec<&Ident> = vec![];
    let mut variant_fields: Vec<(Vec<TokenStream2>, Vec<TokenStream2>)> = vec![];

    for variant in &data.variants {
        match variant.fields {
            Fields::Named(ref fields) => {
                let (variant_discriminator, variant_fields_types, variant_fields_names) =
                    extract_enum_variant_data(fields, &variant.attrs);
                discriminants.push(variant_discriminator);
                idents.push(&variant.ident);
                variant_fields.push((variant_fields_types, variant_fields_names));
            }
            _ => panic!("Derive CreateTypeSpec only supports explicitly discriminated enums consisting of struct variants"),
        }
    }
    if has_unique_elements(discriminants.iter().map(|d| d.to_string())) {
        (discriminants, idents, variant_fields)
    } else {
        panic!("Duplicate discriminant values")
    }
}

fn identifier_creator(
    type_id: &str,
    called_method: syn::Ident,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    let type_params: Vec<syn::Ident> = generics.type_params().map(|x| x.ident.clone()).collect();
    let generics_str = if type_params.is_empty() {
        quote! {}
    } else {
        quote! {
            #( + &pbc_contract_common::abi::capitalize(&#type_params::#called_method()))*
        }
    };
    quote! { #type_id.to_owned() #generics_str }
}

fn create_type_spec_impl(
    type_name: &Ident,
    type_id: String,
    trait_name: &syn::Path,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name_creator =
        identifier_creator(&type_name.to_string(), format_ident!("__ty_name"), generics);
    let identifier_creator =
        identifier_creator(&type_id, format_ident!("__ty_identifier"), generics);

    quote! {
        #[cfg(feature = "abi")]
        #[automatically_derived]
        impl #impl_generics #trait_name for #type_name #ty_generics #where_clause {
            fn __ty_name() -> String {
                #name_creator
            }

            fn __ty_identifier() -> String {
                #identifier_creator
            }

            fn __ty_spec_write(w: &mut Vec<u8>, named_types: &std::collections::BTreeMap<String, u8>) {
                let type_id: String = Self::__ty_identifier();
                let type_index: u8 = *named_types.get(&type_id).unwrap_or(&0xFF);

                w.push(0x00);
                w.push(type_index);
            }
        }
    }
}

fn tokens_to_ident<T: ToTokens>(v: &T) -> String {
    format!("{}", v.to_token_stream())
        .to_lowercase()
        .replace(char::is_whitespace, "_")
        .matches(|c: char| c.is_ascii_alphanumeric() || c == '_')
        .collect()
}

fn abi_for_type_fn_name(ast_type: &syn::TypePath) -> syn::Ident {
    let mut non_argument_path = ast_type.path.clone();
    non_argument_path.segments.last_mut().unwrap().arguments = syn::PathArguments::None;
    format_ident!("__abi_for_type_{}", tokens_to_ident(&non_argument_path))
}

fn create_type_spec_extern_c(ast_type: &syn::TypePath) -> proc_macro2::TokenStream {
    let abi_for_type_function_name = abi_for_type_fn_name(ast_type);

    let abi_type_as_fn_ptr_function_name =
        format_ident!("__abi_type_as_fn_ptr_{}", tokens_to_ident(ast_type));

    let ty_generics_turbofish = match &ast_type.path.segments.last().unwrap().arguments {
        syn::PathArguments::AngleBracketed(args) => {
            let mut args = args.clone();
            args.colon2_token = None;
            quote! { :: #args }
        }
        _ => quote!(),
    };

    quote! {
        #[cfg(feature = "abi")]
        #[no_mangle]
        #[doc = "PBC ABI gen internal method. Ensures that [`"]
        #[doc = stringify!(#ast_type)]
        #[doc = "`] is visible to the ABI generator."]
        #[doc = "This is a method used by the PBC ABI-gen system for generating ABI (machine-readable descriptions of smart contracts.)"]
        #[automatically_derived]
        pub unsafe extern "C" fn #abi_type_as_fn_ptr_function_name () -> u32 {
            let function_pointer = #abi_for_type_function_name #ty_generics_turbofish as *const ();
            function_pointer as u32
        }
    }
}

fn data_to_field_types(data: &DataStruct) -> (Vec<Ident>, Vec<TokenStream2>) {
    match data.fields {
        Fields::Named(ref fields) => {
            let names: Vec<Ident> = fields
                .named
                .iter()
                .map(derive_commons::field_to_name)
                .collect();
            let types: Vec<TokenStream2> = fields
                .named
                .iter()
                .map(derive_commons::field_to_type)
                .collect();
            (names, types)
        }
        _ => {
            panic!("Derive CreateTypeSpec only supports named fields for structs")
        }
    }
}
