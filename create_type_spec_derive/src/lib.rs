//! Internal Partisia Blockchain SDK crate with derive logic for `CreateTypeSpec` trait.
//!
//! *This module is only used during `"ABI"` construction.*

extern crate derive_commons;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::__private::TokenStream2;
use syn::{Data, Fields, Ident};
use uuid::Uuid;

/// Derive the `CreateTypeSpec` trait for a struct.
#[proc_macro_derive(CreateTypeSpec)]
pub fn create_type_spec(input: TokenStream) -> TokenStream {
    // Parse the AST
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_create_type_spec(&ast);

    // Return the generated impl
    gen.into()
}

fn impl_create_type_spec(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (field_names, field_types) = data_to_field_types(&ast.data);
    let string_name = name.to_string();

    let uuid = Uuid::new_v4();
    let type_id = uuid.to_hyphenated().to_string();

    let lowercase_name = name.to_string().to_lowercase();
    let lowercase_ident = format_ident!("{}", &name.to_string().to_lowercase());
    let mut implementation = quote! {
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

                let type_key = Self::__ty_identifier();
                let type_index = match lut.get(&type_key) {
                    Some(index) => *index,
                    None => 0,
                };

                w.push(type_index);
            }
        }
    };

    let fields_string_names: Vec<String> = field_names
        .iter()
        .map(|name| -> String { name.to_string().to_lowercase() })
        .collect();
    let create_func_ident = format_ident!("__abi_for_type_{}", lowercase_name);

    let create_type = quote! {
        #[cfg(feature = "abi")]
        fn #create_func_ident(lut: &std::collections::BTreeMap< String, u8>) -> pbc_contract_common::abi::TypeAbi {
            let mut type_abi = pbc_contract_common::abi::TypeAbi::new::<#name>(format!("{}", #string_name), lut);
            #(
                let #field_names = pbc_contract_common::abi::NamedEntityAbi::new::<#field_types>(#fields_string_names.to_string(), lut);
                type_abi.field(#field_names);
            )*

            type_abi
        }
    };

    let func_name = format_ident!("__abi_type_as_fn_ptr_{}", lowercase_name);
    let to_ptr_func = quote! {
        #[cfg(feature = "abi")]
        #[no_mangle]
        #[doc = "ABI: Function pointer lookup"]
        pub unsafe extern "C" fn #func_name() -> u32 {
            let function_pointer = #create_func_ident as *const ();
            function_pointer as u32
        }
    };

    implementation.extend(create_type);
    implementation.extend(to_ptr_func);
    implementation
}

fn data_to_field_types(data: &Data) -> (Vec<Ident>, Vec<TokenStream2>) {
    match *data {
        Data::Struct(ref data) => match data.fields {
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
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
