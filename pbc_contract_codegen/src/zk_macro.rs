use proc_macro::TokenStream;

use crate::macro_abi::{make_hook_abi_fn, make_hook_abi_fn_delegator};
use pbc_contract_common::address::Shortname;

use crate::{
    determine_names, variables_for_inner_call, wrap_function_for_export, CallType, TokenStream2,
    WrappedFunctionKind,
};

pub(crate) fn handle_zk_macro(
    input: TokenStream,
    shortname_override: Option<Shortname>,
    export_symbol_base: &str,
    kind: &WrappedFunctionKind,
    shortname_in_export: bool,
) -> TokenStream {
    let fn_ast: syn::ItemFn = syn::parse(input.clone()).unwrap();
    let names = determine_names(
        shortname_override,
        &fn_ast,
        export_symbol_base,
        shortname_in_export,
    );
    let docs = format!(
        "Serialization wrapper for contract zk hook `{}`.",
        names.fn_identifier
    );

    let invocation = variables_for_inner_call(&fn_ast, CallType::Action);
    let mut result = wrap_function_for_export(
        &names.fn_identifier,
        names.export_symbol,
        &docs,
        invocation,
        kind,
    );

    let abi_fn_name = format_ident!("__abi_fn_{}", &names.fn_identifier);
    let abi_fn = {
        let rpc_pos = kind.system_arguments;
        let shortname_u32 = names.function_name.shortname().as_u32();
        let shortname_ident =
            quote! {Some(pbc_contract_common::address::Shortname::from_u32(#shortname_u32))};
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