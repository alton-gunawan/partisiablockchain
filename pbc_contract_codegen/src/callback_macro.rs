use proc_macro::TokenStream;

use crate::macro_abi::{make_hook_abi_fn, make_hook_abi_fn_delegator};
use pbc_contract_common::address::Shortname;

use crate::{
    determine_names, variables_for_inner_call, wrap_function_for_export, CallType, SecretInput,
    TokenStream2, WrappedFunctionKind,
};

pub fn handle_callback_macro(
    input: TokenStream,
    shortname_override: Option<Shortname>,
) -> TokenStream {
    let fn_ast: syn::ItemFn = syn::parse(input.clone()).unwrap();

    let names = determine_names(shortname_override, &fn_ast, "callback", true);

    let docs = format!(
        "Serialization wrapper for contract callback `{}`.",
        names.fn_identifier
    );

    let kind = WrappedFunctionKind::public_contract_hook_kind(
        3,
        pbc_contract_common::FunctionKind::Callback,
    );

    let invocation = variables_for_inner_call(&fn_ast, CallType::Callback);
    let mut result = wrap_function_for_export(
        &names.fn_identifier,
        names.export_symbol,
        &docs,
        invocation,
        &kind,
    );

    let abi_fn_name = format_ident!("__abi_fn_{}", &names.fn_identifier);
    let shortname_u32 = names.function_name.shortname().as_u32();
    let abi_fn = {
        let rpc_pos = kind.system_arguments;
        let shortname_ident =
            quote! {Some(pbc_contract_common::address::Shortname::from_u32(#shortname_u32))};
        make_hook_abi_fn(
            &fn_ast,
            &abi_fn_name,
            kind.fn_kind,
            rpc_pos,
            shortname_ident,
            SecretInput::None,
        )
    };

    let shortname_fn: TokenStream2 = {
        let shortname = format_ident!(
            "SHORTNAME_{}",
            names.fn_identifier.to_string().to_uppercase()
        );
        quote! {const #shortname: pbc_contract_common::address::ShortnameCallback = pbc_contract_common::address::ShortnameCallback::from_u32(#shortname_u32);}
    };

    result.extend(TokenStream2::from(input));
    result.extend(abi_fn);
    result.extend(make_hook_abi_fn_delegator(&abi_fn_name));
    result.extend(shortname_fn);
    result.into()
}
