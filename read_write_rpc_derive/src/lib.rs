//! A crate for deriving `ReadRPC` and `WriteRPC`.
extern crate derive_commons;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use derive_commons::{impl_read_write, ReadWriteGenType};

/// Implement `ReadRPC` for the annotated struct and enums.
#[proc_macro_derive(ReadRPC, attributes(discriminant))]
pub fn implement_read_rpc(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_read_write(
        &ast,
        format_ident!("ReadRPC"),
        format_ident!("rpc_read_from"),
        format_ident!("rpc_write_to"),
        None,
        ReadWriteGenType::Read,
    );

    // Return the generated impl
    gen.into()
}

/// Implement `WriteRPC` for the annotated struct and enums.
#[proc_macro_derive(WriteRPC, attributes(discriminant))]
pub fn implement_write_rpc(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_read_write(
        &ast,
        format_ident!("WriteRPC"),
        format_ident!("rpc_read_from"),
        format_ident!("rpc_write_to"),
        None,
        ReadWriteGenType::Write,
    );

    // Return the generated impl
    gen.into()
}

/// Implement `ReadRPC` and `WriteRPC` for the annotated struct and enums.
#[proc_macro_derive(ReadWriteRPC, attributes(discriminant))]
pub fn implement_read_write_rpc(input: TokenStream) -> TokenStream {
    let mut gen: TokenStream = implement_read_rpc(input.clone());
    gen.extend(implement_write_rpc(input));
    gen
}
