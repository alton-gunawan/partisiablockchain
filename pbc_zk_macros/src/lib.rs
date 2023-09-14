//! This crate implements macros for use in the `pbc_zk` crate.
//!
//! Currently this includes:
//!
//! - `SecretBinary`

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

fn implement_secret(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let type_name = &ast.ident;
    let supported_kind = derive_commons::get_kind_data(&ast.data);

    let field_types = match supported_kind {
        derive_commons::SupportedKind::StructWithNamedFields { fields, .. } => fields
            .into_iter()
            .map(derive_commons::field_to_type)
            .collect(),
        derive_commons::SupportedKind::DiscriminatedCstyleEnum { .. } => vec![],
        derive_commons::SupportedKind::ItemStructEnum { .. } => unimplemented!("ItemStructEnum"),
    };

    let impl_secret_block = quote! {
        #[automatically_derived]
        impl ::pbc_zk::SecretBinary for #type_name { }
        #[automatically_derived]
        impl #type_name {
            #[automatically_derived]
            #[allow(dead_code)]
            fn __ignore_for_secret_binary() {
                fn ignore<T: ::pbc_zk::SecretBinary>() {}
                #(
                    ignore::<#field_types>();
                )*
            }
        };
    };

    // Return the generated impl
    impl_secret_block.into()
}

/// Implements `#[derive(SecretBinary)]`.
#[proc_macro_derive(SecretBinary)]
pub fn derive_secret_binary(input: TokenStream) -> TokenStream {
    implement_secret(input)
}

/// Marks the given function as a ZK computation entry point, with the given shortname.
#[proc_macro_attribute]
pub fn zk_compute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Currently implemented only for compatibility.
    item
}
