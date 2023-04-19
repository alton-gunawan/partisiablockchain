use proc_macro2::{Ident, TokenStream};

static CLIENT_ABI_VERSION: [u8; 3] = [5, 2, 0];
/// Binder version for private contracts.
#[cfg(feature = "zk")]
static BINDER_ABI_VERSION: [u8; 3] = [9, 2, 0];
/// Binder version for public contract.
#[cfg(not(feature = "zk"))]
static BINDER_ABI_VERSION: [u8; 3] = [9, 0, 0];

pub(crate) fn create_version_numbers() -> TokenStream {
    let mut result = create_static_version_client();
    result.extend(create_static_version_binder());
    result
}

fn create_static_version_client() -> TokenStream {
    let name = version_name("CLIENT", CLIENT_ABI_VERSION);
    quote! {
        #[doc = "PBC Version of the binary format used by blockchain clients."]
        #[doc = "This versions the format of the binary data that smart contract code read/write to the contract state and the binary data received/sent in transactions/events."]
        #[no_mangle]
        pub static #name : () = ();
    }
}

fn create_static_version_binder() -> TokenStream {
    let name = version_name("BINDER", BINDER_ABI_VERSION);
    quote! {
        #[doc = "PBC Version of the binary format used by the PBC WASM binder."]
        #[doc = "This versions the format of the binary data that the PBC WASM binder reads when handling smart contracts."]
        #[no_mangle]
        pub static #name : () = ();
    }
}

fn version_name(version_type: &str, version: [u8; 3]) -> Ident {
    let major = version[0];
    let minor = version[1];
    let patch = version[2];

    format_ident!(
        "__PBC_VERSION_{}_{}_{}_{}",
        version_type,
        major,
        minor,
        patch
    )
}

pub(crate) fn create_abi_version_client() -> TokenStream {
    convert_to_expression(CLIENT_ABI_VERSION)
}

pub(crate) fn create_abi_version_binder() -> TokenStream {
    convert_to_expression(BINDER_ABI_VERSION)
}

/// Convert the given 3-byte slice into an expression that can be assigned to a variable.
fn convert_to_expression(version: [u8; 3]) -> TokenStream {
    let major = version[0];
    let minor = version[1];
    let patch = version[2];

    quote! {
        [#major, #minor, #patch]
    }
}
