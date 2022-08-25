use proc_macro::TokenStream;

use syn::Item;
use syn::__private::TokenStream2;

/// This handles the actual state struct AST with regards to it being a contract state.
///
/// It adds derives for `CreateTypeSpec` and `ReadWriteState` on the struct and generates
/// a couple of helper methods used by the ABI generation tool.
pub(crate) fn handle_state_macro(input: TokenStream) -> TokenStream {
    let original_state_struct: proc_macro2::TokenStream = input.clone().into();
    let struct_ast: Item = syn::parse(input).unwrap();
    let state_struct_name = match struct_ast {
        Item::Struct(i) => i.ident.to_string(),
        _ => unimplemented!("The state attribute is only valid for structs."),
    };

    let stamped_versions = crate::version::create_version_numbers();
    let version_client_token = crate::version::create_abi_version_client();
    let version_binder_token = crate::version::create_abi_version_binder();

    let result: TokenStream2 = quote! {
        use create_type_spec_derive::CreateTypeSpec as InternalDeriveCreateType;
        use read_write_state_derive::ReadWriteState as InternalDeriveReadWriteState;

        #stamped_versions

        #[repr(C)]
        #[derive(Clone, InternalDeriveCreateType, InternalDeriveReadWriteState)]
        #original_state_struct

        #[cfg(feature = "abi")]
        #[doc = "ABI: Generate the ABI, write it to memory and return a pointer to said memory"]
        #[no_mangle]
        #[automatically_derived]
        pub unsafe extern "C" fn __abi_generate_to_ptr(
            fn_len: u32, fn_list_ptr: *const u32,
            ty_len: u32, ty_list_ptr: *const u32) -> u64 {

            let version_client = #version_client_token;
            let version_binder = #version_binder_token;
            let state_name = #state_struct_name.to_string();

            pbc_contract_common::abi::generate::generate_abi(version_binder, version_client, state_name, fn_len, fn_list_ptr, ty_len, ty_list_ptr)
        }

        #[cfg(feature = "abi")]
        #[doc = "ABI: Expose a vector based malloc to the host"]
        #[no_mangle]
        #[automatically_derived]
        pub unsafe extern "C" fn __abi_malloc(size:u32) -> u32 {
            let allocated: Vec<u8> = Vec::with_capacity(size as usize);
            let ptr = allocated.as_ptr();
            std::mem::forget(allocated);
            ptr as u32
        }
    };

    // Convert the token from quote into the proc macro token stream
    result.into()
}
