use syn::{Ident, ItemFn};

use crate::{read_arguments_names_and_types, TokenStream2};
use pbc_contract_common::FunctionKind;

fn fn_kind_snippet(fn_kind: FunctionKind) -> TokenStream2 {
    let fn_kind_name = format_ident!("{}", format!("{:?}", fn_kind));
    quote! {
        pbc_contract_common::FunctionKind::#fn_kind_name
    }
}

pub fn make_hook_abi_fn(
    fn_ast: &ItemFn,
    abi_fn_name: &Ident,
    fn_kind: FunctionKind,
    rpc_pos: usize,
    shortname_ident: TokenStream2,
) -> TokenStream2 {
    let fn_name = &fn_ast.sig.ident.to_string();
    let (params, types) = read_arguments_names_and_types(fn_ast, rpc_pos).convert_to_tuple();
    let fn_kind_snippet = fn_kind_snippet(fn_kind);
    quote! {
        #[cfg(feature = "abi")]
        #[doc=concat!("ABI: Create ABI for [`", #fn_name, "`]")]
        #[automatically_derived]
        fn #abi_fn_name(lut: &std::collections::BTreeMap<String, u8>) -> pbc_contract_common::abi::FnAbi {
            let mut fn_abi = pbc_contract_common::abi::FnAbi::new(#fn_name.to_string(), #shortname_ident, #fn_kind_snippet);
            #(fn_abi.argument::<#types>(#params.to_string(), &lut);)*
            fn_abi
        }
    }
}

pub fn make_hook_abi_fn_delegator(delegated_function_to_call: &Ident) -> proc_macro2::TokenStream {
    let function_name = format_ident!("__abi_fn_as_fn_ptr_{}", delegated_function_to_call);
    let result = quote! {
        #[cfg(feature = "abi")]
        #[no_mangle]
        #[doc=concat!("ABI: Delegate ABI call to [`", stringify!(#function_name), "`]. Specific name of this function doesn't matter, only the `__abi_fn_as_fn_ptr` prefix.")]
        #[automatically_derived]
        pub unsafe extern "C" fn #function_name() -> *const () {
            #delegated_function_to_call as *const ()
        }
    };
    result
}
