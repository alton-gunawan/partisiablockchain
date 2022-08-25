use proc_macro::TokenStream;

use crate::macro_abi::{make_hook_abi_fn, make_hook_abi_fn_delegator};
use crate::{
    determine_names, variables_for_inner_call, wrap_function_for_export, CallType, TokenStream2,
    WrappedFunctionKind,
};

pub fn handle_init_macro(input: TokenStream) -> TokenStream {
    let fn_ast: syn::ItemFn = syn::parse(input.clone()).unwrap();
    let names = determine_names(None, &fn_ast, "init", false);

    let invocation = variables_for_inner_call(&fn_ast, CallType::Init);

    let docs = format!(
        "Serialization wrapper for contract init `{}`.",
        names.fn_identifier
    );

    let kind =
        WrappedFunctionKind::public_contract_hook_kind(1, pbc_contract_common::FunctionKind::Init);

    let mut result = wrap_function_for_export(
        &names.fn_identifier,
        names.export_symbol,
        &docs,
        invocation,
        &kind,
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
        )
    };

    result.extend(TokenStream2::from(input));
    result.extend(abi_fn);
    result.extend(make_hook_abi_fn_delegator(&abi_fn_name));
    result.into()
}
