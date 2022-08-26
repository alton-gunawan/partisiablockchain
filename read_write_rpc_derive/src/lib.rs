//! A crate for deriving `ReadWriteRPC`.
extern crate derive_commons;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use derive_commons::impl_read_write;

/// Implement `ReadWriteRPC` for the annotated struct.
#[proc_macro_derive(ReadWriteRPC)]
pub fn read_write(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_read_write(
        &ast,
        format_ident!("ReadWriteRPC"),
        format_ident!("rpc_read_from"),
        format_ident!("rpc_write_to"),
        None,
    );

    // Return the generated impl
    gen.into()
}
