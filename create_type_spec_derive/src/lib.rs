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
#[proc_macro_derive(CreateTypeSpec, attributes(discriminant))]
pub fn create_type_spec(input: TokenStream) -> TokenStream {
    // Parse the AST
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_create_type_spec(&ast, quote! { pbc_contract_common::abi });

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
    let gen = impl_create_type_spec(&ast, quote! { pbc_contract_core::abi });

    // Return the generated impl
    gen.into()
}

fn impl_create_type_spec(
    ast: &syn::DeriveInput,
    abi_module_prefix: TokenStream2,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match ast.data {
        Data::Struct(ref data_struct) => {
            let (field_names, field_types) = data_to_field_types(data_struct);
            let field_names_ts = field_names.iter().map(|x| x.to_token_stream()).collect();
            create_struct_type_spec(name, &field_names_ts, &field_types, &abi_module_prefix)
        }
        Data::Enum(ref data_enum) => create_enum_type_spec(name, data_enum, &abi_module_prefix),
        _ => panic!(
            "CreateTypeSpec derive does not support Union, currently only structs with named \
        fields and explicitly discriminated enums consisting of struct variants"
        ),
    }
}

fn create_enum_type_spec(
    name: &Ident,
    data_enum: &DataEnum,
    abi_module_prefix: &TokenStream2,
) -> proc_macro2::TokenStream {
    let string_name = name.to_string();

    let uuid = Uuid::new_v4();
    let enum_type_id = uuid.to_hyphenated().to_string();

    let lowercase_name = name.to_string().to_lowercase();
    let lowercase_ident = format_ident!("{}", &lowercase_name);

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

    let mut variant_type_ids = vec![];
    for _ in 0..variant_names.len() {
        let identifier = Uuid::new_v4().to_hyphenated().to_string();
        variant_type_ids.push(identifier);
    }
    let variant_type_specs: Vec<Ident> = variant_names_lowercase
        .iter()
        .map(|n| format_ident!("type_spec_{}", n))
        .collect();
    let calculate_variant_type_specs = quote! {
        #(
            let mut #variant_type_specs = vec![];
            #variant_type_specs.push(0x00);
            let type_index = match lut.get(#variant_type_ids) {
                Some(index) => *index,
                None => 0,
            };
            #variant_type_specs.push(type_index);
        )*
    };

    let add_fields_to_variants: Vec<TokenStream2> = variant_fields.iter().zip(struct_variant_names.iter()).map(|(variant_field, struct_name)| {
        let variant_field_names = &variant_field.1;
        let variant_field_string_names: Vec<String> = variant_field_names.iter().map(|name| name.to_string().to_lowercase()).collect();
        let variant_field_types = &variant_field.0;
        quote! {
        #(
                let #variant_field_names = #abi_module_prefix::NamedEntityAbi::new::<#variant_field_types>(#variant_field_string_names.to_string(), lut);
                #struct_name.add_field(#variant_field_names);
            )*
    }
    }).collect();

    let abi_for_type_function_name = format_ident!("__abi_for_type_{}", lowercase_name);
    let abi_type_as_fn_ptr_function_name = format_ident!("__abi_type_as_fn_ptr_{}", lowercase_name);

    quote! {
        #[cfg(feature = "abi")]
        #[automatically_derived]
        impl pbc_traits::CreateTypeSpec for #name {
            fn __ty_name() -> String {
                let #lowercase_ident =  format!("{}", #string_name);
                #lowercase_ident
            }

            fn __ty_identifier() -> String {
                #enum_type_id.to_string()
            }

            fn __ty_spec_write(w: &mut Vec<u8>, lut: &std::collections::BTreeMap<String, u8>) {
                w.push(0x00);

                let type_index = match lut.get(#enum_type_id) {
                    Some(index) => *index,
                    None => 0,
                };

                w.push(type_index);
            }
        }

        #[cfg(feature = "abi")]
        fn #abi_for_type_function_name(lut: &std::collections::BTreeMap< String, u8>) -> Vec<#abi_module_prefix::NamedTypeSpec> {
            let mut named_types = vec![];

            let mut enum_type_spec: Vec<u8> = vec![];
            enum_type_spec.push(0x00);
            let type_index = match lut.get(#enum_type_id) {
                Some(index) => *index,
                None => 0,
            };
            enum_type_spec.push(type_index);

            #calculate_variant_type_specs

            let mut enum_type_spec = #abi_module_prefix::NamedTypeSpec::new_enum(#string_name.to_string(), #enum_type_id.to_string(), enum_type_spec);

            #(
                let #variant_names_lowercase = #abi_module_prefix::EnumVariant::new(#variant_discriminants, #variant_type_specs.clone());
                enum_type_spec.add_variant(#variant_names_lowercase);
            )*
            named_types.push(enum_type_spec);

            #(

              let mut #struct_variant_names = #abi_module_prefix::NamedTypeSpec::new_struct(#variant_names_string.to_string(), #variant_type_ids.to_string(), #variant_type_specs);
              #add_fields_to_variants
              named_types.push(#struct_variant_names);
            )*
            named_types

        }

        #[cfg(feature = "abi")]
        #[no_mangle]
        #[doc = "ABI: Function pointer lookup"]
        pub unsafe extern "C" fn #abi_type_as_fn_ptr_function_name() -> u32 {
            let function_pointer = #abi_for_type_function_name as *const ();
            function_pointer as u32
        }
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

fn create_struct_type_spec(
    name: &Ident,
    field_names: &Vec<TokenStream2>,
    field_types: &Vec<TokenStream2>,
    abi_module_prefix: &TokenStream2,
) -> proc_macro2::TokenStream {
    let string_name = name.to_string();

    let uuid = Uuid::new_v4();
    let type_id = uuid.to_hyphenated().to_string();

    let lowercase_name = name.to_string().to_lowercase();
    let lowercase_ident = format_ident!("{}", &name.to_string().to_lowercase());

    let fields_string_names: Vec<String> = field_names
        .iter()
        .map(|name| -> String { name.to_string().to_lowercase() })
        .collect();

    let abi_for_type_function_name = format_ident!("__abi_for_type_{}", lowercase_name);
    let abi_type_as_fn_ptr_function_name = format_ident!("__abi_type_as_fn_ptr_{}", lowercase_name);

    let implementation = quote! {
        #[cfg(feature = "abi")]
        #[automatically_derived]
        impl pbc_traits::CreateTypeSpec for #name {
            fn __ty_name() -> String {
                let #lowercase_ident =  format!("{}", #string_name);
                #(
                    <#field_types as pbc_traits::CreateTypeSpec>::__ty_name();
                )*
                #lowercase_ident
            }

            fn __ty_identifier() -> String {
                #type_id.to_owned()
            }

            fn __ty_spec_write(w: &mut Vec<u8>, lut: &std::collections::BTreeMap<String, u8>) {
                w.push(0x00);

                let type_index = match lut.get(#type_id) {
                    Some(index) => *index,
                    None => 0,
                };

                w.push(type_index);
            }
        }

        #[cfg(feature = "abi")]
        fn #abi_for_type_function_name(named_types: &std::collections::BTreeMap< String, u8>) -> Vec<#abi_module_prefix::NamedTypeSpec> {
            let mut type_spec: Vec<u8> = vec![];
            type_spec.push(0x00);
            let type_index = match named_types.get(#type_id) {
                Some(index) => *index,
                None => 0,
            };
            type_spec.push(type_index);

            let mut type_abi = #abi_module_prefix::NamedTypeSpec::new_struct(#string_name.to_string(), #type_id.to_string(), type_spec);
            #(
                let #field_names = #abi_module_prefix::NamedEntityAbi::new::<#field_types>(#fields_string_names.to_string(), named_types);
                type_abi.add_field(#field_names);
            )*

            vec![type_abi]
        }

        #[cfg(feature = "abi")]
        #[no_mangle]
        #[doc = "ABI: Function pointer lookup"]
        pub unsafe extern "C" fn #abi_type_as_fn_ptr_function_name() -> u32 {
            let function_pointer = #abi_for_type_function_name as *const ();
            function_pointer as u32
        }
    };

    implementation
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
