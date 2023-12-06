#![doc = include_str!("../README.md")]
#![recursion_limit = "128"]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use std::cmp::max;
use std::collections::HashMap;

use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{
    AttributeArgs, FnArg, Ident, Lit, Meta, NestedMeta, PatType, ReturnType, Type, TypeArray,
    TypePath,
};

use crate::tokenized::{ArgumentList, InstantiableArgument, TokenizedInvocation};
use pbc_contract_common::address::Shortname;
use pbc_contract_common::{FunctionKind, FunctionName};

pub mod action_macro;
pub mod callback_macro;
pub mod init_macro;
mod macro_abi;
pub mod state_macro;
mod tokenized;
mod version;
pub mod zk_compute_macro;
pub mod zk_macro;

/// Parses the attributes of a macro into a map from attribute names to a `Literal`.
///
/// ### Parameters:
///
/// * `args`: [`AttributeArgs`] - the args to be parsed.
///
/// * `valid_names`: [`Vec<String>`] - valid names of attributes. Panics if an attribute name not
/// in valid_names is present in args.
///
/// * `required_names`: [`Vec<String>`] - required names. Panics if any of the attribute names is
/// not present in args.
///
/// ### Returns
///
/// A map from attribute name to their value.
pub fn parse_attributes(
    args: AttributeArgs,
    valid_names: Vec<String>,
    required_names: Vec<String>,
) -> HashMap<String, Lit> {
    let metas = args.iter().map(|nested_meta| match nested_meta {
        NestedMeta::Meta(meta) => meta,
        _ => panic!("Invalid attribute: {}", nested_meta.to_token_stream()),
    });

    let mut result = HashMap::new();

    for meta in metas {
        match meta {
            Meta::NameValue(pair) => {
                let name = pair
                    .path
                    .get_ident()
                    .map(|ident| ident.to_string())
                    .unwrap_or_else(|| "INVALID".to_string());

                if !valid_names.contains(&name) {
                    panic!(
                        "Invalid attribute found, valid attributes are: {}",
                        valid_names.join(", ")
                    );
                }
                result.insert(name, pair.lit.clone());
            }
            _ => panic!("Invalid attribute: {}", meta.to_token_stream()),
        }
    }

    for required_name in required_names {
        assert!(
            result.get(&required_name).is_some(),
            "Required attribute '{required_name}' is missing",
        );
    }

    result
}

/// Gets the shortname attribute of the arguments and if present parses it into a `Shortname`.
/// Panics if the attribute is not a valid shortname literal.
///
/// ### Parameters:
///
/// * `args`: &[HashMap<String, Lit>] - parsed attributes of a macro.
///
/// ### Returns
/// Some of the parsed shortname if present in args, None if it is not present.
pub fn parse_shortname_override(args: &HashMap<String, Lit>) -> Option<Shortname> {
    args.get("shortname").map(|lit: &Lit| match lit {
        Lit::Int(lit_int) if is_hex_literal(lit_int) => {
            let x: u64 = lit_int
                .base10_parse()
                .expect("Invalid shortname, expecting a u32 hex literal");
            select_leb_bytes(x.to_be_bytes()).expect("Invalid shortname, should be LEB128 encoded")
        }
        _ => panic!(
            "Invalid shortname, expecting a u32 hex literal, but got: {}",
            lit.to_token_stream()
        ),
    })
}

/// Gets the zk attribute of the arguments and if present parses it into a `bool`.
/// Panics if the attribute is not a valid boolean literal.
///
/// ### Parameters:
///
/// * `args`: &[HashMap<String, Lit>] - parsed attributes of a macro.
///
/// ### Returns
/// `true` if the zk attribute is present and set to true, `false` otherwise.
pub fn parse_zk_argument(args: &HashMap<String, Lit>) -> bool {
    let zk = args.get("zk").map(|lit: &Lit| match lit {
        Lit::Bool(lit_bool) => lit_bool.value,
        _ => panic!(
            "Invalid zk attribute, expecting a boolean literal, but got: {}",
            lit.to_token_stream()
        ),
    });
    zk.unwrap_or(false)
}

/// Gets the secret_type attribute of and parses it into a `SecretInput` enum containing the
/// secret input type.
/// Panics if the attribute is not a string literal.
///
/// ### Parameters:
///
/// * `args`: [HashMap<String, Lit>] - parsed attributes of a macro.
///
/// ### Returns
/// Some of the secret input type if present in args. Default Sbi32 if not present.
pub fn parse_secret_type_input(args: HashMap<String, Lit>) -> SecretInput {
    match args.get("secret_type") {
        None => SecretInput::Default,
        Some(Lit::Str(lit_str)) => SecretInput::Some(lit_str.value()),
        Some(err) => panic!(
            "Invalid type, expecting a string literal, but got: {}",
            err.to_token_stream()
        ),
    }
}

/// Describes the kind of secret input argument have been defined.
pub enum SecretInput {
    /// The action does not require a secret input.
    None,
    /// The default secret input type have been given.
    Default,
    /// The type given by the string name is given.
    Some(String),
}

impl SecretInput {
    /// Determines the type string, if any was given.
    pub fn to_secret_type_str(&self) -> Option<&str> {
        match self {
            SecretInput::Some(secret_type) => Some(secret_type),
            _ => None,
        }
    }
}

/// Whether the given integer literal is hex formatted or not.
fn is_hex_literal(lit: &syn::LitInt) -> bool {
    let token_text = format!("{}", lit.token());
    return token_text.starts_with("0x")
        && token_text.chars().skip(2).all(|c| c.is_ascii_hexdigit());
}

fn select_leb_bytes<const N: usize>(bytes: [u8; N]) -> Result<Shortname, String> {
    let idx_first_non_zero = bytes.iter().position(|&x| x != 0).unwrap_or(N - 1);
    let vec_bytes: Vec<_> = bytes.iter().skip(idx_first_non_zero).copied().collect();
    Shortname::from_be_bytes(&vec_bytes)
}

/// Describes how to wrap a function as an action.
pub struct WrappedFunctionKind {
    /// Whether the wrapped function will output state and events.
    pub output_state_and_events: bool,
    /// What other kinds of types the function can output.
    pub output_other_types: Vec<(TokenStream2, Ident)>,
    /// The minimum allowed number of results.
    pub min_allowed_num_results: usize,
    /// Number of "system" arguments before RPC arguments occur
    pub system_arguments: usize,
    /// The [`FunctionKind`] of the wrapped function.
    pub fn_kind: FunctionKind,
    /// If `false`, RPC arguments are disallowed, and only system arguments must occur.
    pub allow_rpc_arguments: bool,
}

impl WrappedFunctionKind {
    fn public_contract_hook_kind(
        system_arguments: usize,
        fn_kind: FunctionKind,
        zk_state_allowed: bool,
    ) -> Self {
        let output_other_types = if zk_state_allowed {
            // With Zk we only need state, events optional, and ZkStateChange optional.
            vec![(
                quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
                format_ident!("write_zk_state_change"),
            )]
        } else {
            // Without Zk we only need state, events optional.
            vec![]
        };
        Self {
            output_state_and_events: true,
            output_other_types,
            min_allowed_num_results: 1,
            system_arguments: system_arguments + usize::from(zk_state_allowed),
            fn_kind,
            allow_rpc_arguments: true,
        }
    }

    fn types(&self) -> Vec<TokenStream2> {
        let mut types = vec![];
        if self.output_state_and_events {
            types.push(quote! { _ });
            types.push(quote! { Vec<pbc_contract_common::events::EventGroup> });
        };
        for (typ, _) in &self.output_other_types {
            types.push(typ.clone());
        }
        types
    }

    fn write_methods(&self) -> Vec<Ident> {
        let mut methods = vec![];
        if self.output_state_and_events {
            methods.push(format_ident!("write_state"));
            methods.push(format_ident!("write_events"));
        };
        for (_, method) in &self.output_other_types {
            methods.push(method.clone());
        }
        methods
    }
}

/// Generate an exported function for reading the rpc and call the wrapped function. The result
/// is then written to a [ContractResultBuffer] placed as is expected by the blockchain,
/// and produces a value so the blockchain can locate the buffer result.
///
/// ### Parameters:
/// * `fn_identifier`: &[Ident], The identifier for the function to wrap.
/// * `export_symbol`: [Ident], The identifier for the wrapper function.
/// * `docs`: &[str], The documentation to insert above the wrapper function.
/// * `arguments`: [TokenizedInvocation], The arguments for the wrapped function.
/// * `function_kind`: &[WrappedFunctionKind], The function kind, e.g. action or callback.
/// * `check_zk_contract`: Option<[bool]>, If `Some(true)` asserts that the contract is a zk-contract.
///     if `Some(false)` asserts that the contract is a public contract, otherwise no check is performed.
///
/// ### Returns:
/// The [TokenStream2] for the wrapper function.
#[allow(clippy::too_many_arguments)]
fn wrap_function_for_export(
    fn_identifier: &Ident,
    export_symbol: Ident,
    docs: &str,
    arguments: TokenizedInvocation,
    function_kind: &WrappedFunctionKind,
    check_zk_contract: Option<bool>,
) -> TokenStream2 {
    // Check that function is well-formed
    if arguments.num_params() > function_kind.system_arguments && !function_kind.allow_rpc_arguments
    {
        panic!(
            "Functions annotated with this macro must have at most {} arguments",
            function_kind.system_arguments,
        );
    }
    let fn_ident = fn_identifier.to_string();
    let kind = format!("{:?}", function_kind.fn_kind).to_lowercase();
    let zk_check_stream = if let Some(zk_argument) = check_zk_contract {
        let error_message = if zk_argument {
            format!("{fn_ident} cannot be zk if the init function is not zk. Consider using #[init(zk = true)]")
        } else {
            format!("{fn_ident} cannot be non-zk if the init function is zk. Consider using #[{kind}(zk = true)]")
        };
        check_valid_zk_contract(zk_argument, error_message)
    } else {
        TokenStream2::new()
    };

    let reader = format_ident!("input_reader");
    let rpc_read = &arguments.param_instantiation_expr();
    let rpc_param_names = &arguments.param_names();
    let ctx_expression = arguments.context.expression;

    let mut invoke_read_expr: Vec<TokenStream2> = Vec::new();
    let mut invoke_vars: Vec<Ident> = Vec::new();
    if let Some(callback_context) = arguments.callback_context {
        invoke_vars.push(callback_context.variable_name());
        invoke_read_expr.push(callback_context.expression);
    }

    // Create identifier that is difficult to accidentically collide with, and extremely obvious
    // when deliberately colliding.
    let rust_visible_symbol = format_ident!("__pbc_autogen__{}_wrapped", fn_identifier);

    let mut result_types = function_kind.types();
    result_types.truncate(max(
        arguments.result_types.len(),
        function_kind.min_allowed_num_results,
    ));

    if arguments.result_types.len() < function_kind.min_allowed_num_results {
        panic!(
            "Functions annotated with this macro must have at least {} return values, but had only {}",
            function_kind.min_allowed_num_results, arguments.result_types.len()
        );
    }

    let result_tuple_indice = (0..result_types.len()).map(syn::Index::from);

    if let Some(state) = arguments.state {
        invoke_vars.push(state.variable_name());
        invoke_read_expr.push(state.expression);
    }

    let write_methods: Vec<_> = function_kind.write_methods();
    let indices_with_methods: Vec<_> = write_methods.iter().zip(result_tuple_indice).collect();

    let is_single = indices_with_methods.len() == 1;

    let write_statements: Vec<_> = indices_with_methods
        .iter()
        .map(|(write_method, result_idx)| {
            let path = if is_single {
                quote! { result }
            } else {
                quote! { result.#result_idx }
            };
            quote! { result_buffer.#write_method(#path); }
        })
        .collect();

    let stream: TokenStream2 = quote! {
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        #[doc = #docs]
        #[no_mangle]
        #[automatically_derived]
        #[export_name = stringify!(#export_symbol)]
        pub extern "C" fn #rust_visible_symbol(
            input_buf_ptr: *mut u8, input_buf_len: usize,
        ) -> u64 {
            #[cfg(all(not(feature = "abi"), any(target_arch = "wasm32", doc)))]
            pbc_lib::exit::override_panic();
            #zk_check_stream
            let mut #reader = unsafe { std::slice::from_raw_parts(input_buf_ptr, input_buf_len) };
            let context = #ctx_expression;
            #(let #invoke_vars = #invoke_read_expr;)*
            #rpc_read
            assert!(#reader.is_empty(), "Input data too long; {} bytes remaining", #reader.len());

            let result: (#(#result_types),*) = #fn_identifier(context, #(#invoke_vars,)* #(#rpc_param_names,)*);
            let mut result_buffer = pbc_contract_common::ContractResultBuffer::new();
            #(#write_statements)*

            unsafe { result_buffer.finalize_result_buffer() }
        }
    };
    stream
}

/// The
/// [FnKind](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html)'s
/// call protocol, dictating how the annotated function must format it's function singature.
pub(crate) enum FnKindCallProtocol {
    /// Calling protocol for contract initialization, identical to [`Action`] with no existing contract state.
    Init,
    /// Calling protocol for normal invocations, with a context, state and rpc arguments.
    Action,
    /// Calling protocol for callback invocations, with a context, callback context, state and rpc arguments.
    Callback,
}

/// Various names for a function.
pub(crate) struct Names {
    fn_identifier: Ident,
    function_name: FunctionName,
    export_symbol: Ident,
}

/// Determines various names to be used to address the given [`syn::ItemFn`].
pub(crate) fn determine_names(
    shortname_def: Option<Shortname>,
    fn_ast: &syn::ItemFn,
    export_symbol_base: &str,
    shortname_in_symbol: bool,
) -> Names {
    let fn_identifier: &Ident = &fn_ast.sig.ident;

    let function_name = FunctionName::new(fn_identifier.to_string(), shortname_def);
    let shortname = function_name.shortname();
    let export_symbol = if shortname_in_symbol {
        format_ident!("{}_{}", export_symbol_base, shortname.to_string())
    } else {
        format_ident!("{}", export_symbol_base)
    };

    Names {
        fn_identifier: fn_identifier.clone(),
        function_name,
        export_symbol,
    }
}

/// Determines the vector of returned types from the given type. Flattens tuples.
fn determine_return_types(t: &Type) -> Vec<Type> {
    match t {
        Type::Tuple(syn::TypeTuple { elems, .. }) => elems.iter().cloned().collect(),
        Type::Paren(syn::TypeParen { elem, .. }) => determine_return_types(elem),
        some_type => {
            vec![some_type.clone()]
        }
    }
}

/// Determines the vector of returned types from the given function. Flattens tuples.
fn determine_return_types_from_output(return_type: &ReturnType) -> Vec<Type> {
    match return_type {
        syn::ReturnType::Default => vec![],
        syn::ReturnType::Type(_, t) => determine_return_types(t),
    }
}

/// Determines the variable data to be used with [`wrap_function_for_export`].
fn variables_for_inner_call(
    item: &syn::ItemFn,
    call_protocol: FnKindCallProtocol,
    require_zk_state: bool,
) -> TokenizedInvocation {
    // Constants by FnKindCallProtocol
    let expected_min_arguments = match call_protocol {
        FnKindCallProtocol::Init => 1,
        FnKindCallProtocol::Action => 2,
        FnKindCallProtocol::Callback => 3,
    } + usize::from(require_zk_state);

    // Parse
    let mut item_iterator = item.sig.inputs.iter();
    assert!(
        item_iterator.len() >= expected_min_arguments,
        "Functions annotated with this macro must have at least {} arguments, but had only {}",
        expected_min_arguments,
        item_iterator.len()
    );

    let ctx = read_arguments_for_instantiation(item_iterator.next().unwrap(), false);
    let callback_context = match call_protocol {
        FnKindCallProtocol::Callback => {
            let token = item_iterator
                .next()
                .expect("Callbacks must possess a CallbackContext argument");
            let callback = read_arguments_for_instantiation(token, false);
            Some(callback)
        }
        _ => None,
    };

    let state = match call_protocol {
        FnKindCallProtocol::Action | FnKindCallProtocol::Callback => {
            let token = item_iterator
                .next()
                .expect("Action and callbacks must possess a State argument");
            let state_tmp = read_arguments_for_instantiation(token, true);
            Some(state_tmp)
        }
        _ => None,
    };

    let zk_state = if require_zk_state {
        let token = item_iterator
            .next()
            .expect("Hooks in Zk contracts must possess a ZkState argument");
        let state_tmp = read_arguments_for_instantiation(token, false);
        Some(state_tmp)
    } else {
        None
    };

    let result_types: Vec<Type> = determine_return_types_from_output(&item.sig.output);

    // Parse RPC params
    let rpc_params = item_iterator
        .map(|token| read_arguments_for_instantiation(token, false))
        .collect();
    TokenizedInvocation::new(
        ctx,
        callback_context,
        state,
        zk_state,
        rpc_params,
        result_types,
    )
}

/// Read the arguments from the given function AST.
///
/// * `item` - the parsed function
/// * `skip` - number of leading items to skip
fn read_arguments_names_and_types(item: &syn::ItemFn, skip: usize) -> ArgumentList {
    let mut arguments = ArgumentList::new();
    for token in item.sig.inputs.iter() {
        let pat = determine_parameter_type(token);
        let identifier = pat.pat.to_token_stream();
        let ty = pat.ty.to_token_stream();
        arguments.push(identifier, ty);
    }

    arguments.split_off(skip)
}

/// Determines the parameter type for the given function argument.
fn determine_parameter_type(token: &FnArg) -> &PatType {
    match token {
        FnArg::Receiver(_) => {
            panic!("Contract functions must be bare functions.")
        }
        FnArg::Typed(pat) => pat,
    }
}

fn read_arguments_for_instantiation(token: &FnArg, is_state: bool) -> InstantiableArgument {
    let pat = determine_parameter_type(token);
    let var_name = match &*pat.pat {
        syn::Pat::Ident(x) => x.ident.to_string(),
        pat => panic!("Unsupported argument pattern: {}", pat.to_token_stream()),
    };

    let ty = *(pat.ty.clone());
    match ty {
        Type::Path(path) => {
            let expr = generate_read_from_path_expression(path, is_state);
            InstantiableArgument::new(&var_name, expr)
        }

        Type::Tuple(_) => {
            panic!("Unsupported tuple type");
        }

        Type::Array(array) => {
            let expr = generate_read_from_array_expression(array, is_state);
            InstantiableArgument::new(&var_name, expr)
        }

        Type::ImplTrait(_) => {
            panic!("Unsupported impl trait type");
        }

        Type::Reference(_) => {
            panic!("Unsupported reference type");
        }

        Type::Slice(_) => {
            panic!("Unsupported slice type");
        }

        _ => {
            panic!("Unsupported argument type.")
        }
    }
}

/// Generate instantiating expressions for the given type.
///
/// This is a part of a macro and assumes that `input_buf` is in scope where the macro is called
/// and that said ident represents an instance of std::io::Read.
///
/// * `path` - the AST type to generate an instantiating expression for
/// * `is_state` - whether we are using [`pbc_traits::ReadWriteState`] or [`pbc_traits::ReadRPC`]
fn generate_read_from_path_expression(path: TypePath, is_state: bool) -> TokenStream2 {
    let (trait_type, read_from) = if is_state {
        (quote!(pbc_traits::ReadWriteState), quote!(state_read_from))
    } else {
        (quote!(pbc_traits::ReadRPC), quote!(rpc_read_from))
    };
    let type_name = match path.path.get_ident() {
        Some(ident) => quote! {#ident},
        None => path.into_token_stream(),
    };
    quote! {<#type_name as #trait_type>::#read_from(&mut input_reader);}
}

/// Generate instantiating expressions for the given array.
///
/// This is a part of a macro and assumes that `reader_ident` is in scope where the macro is called
/// and that said ident represents an instance of std::io::Read.
///
/// * `reader_ident` - the reader variable/expression
/// * `path` - the AST type to generate an instantiating expression for
/// * `is_state` - whether we are using `pbc_traits::ReadWriteState` or `pbc_traits::ReadRPC`
fn generate_read_from_array_expression(array: TypeArray, is_state: bool) -> TokenStream2 {
    let (trait_type, read_from) = if is_state {
        (quote!(pbc_traits::ReadWriteState), quote!(state_read_from))
    } else {
        (quote!(pbc_traits::ReadRPC), quote!(rpc_read_from))
    };

    let array_tokens = array.to_token_stream();
    quote! { <#array_tokens as #trait_type>::#read_from(&mut input_reader); }
}

fn check_valid_zk_contract(zk_argument: bool, error_message: String) -> TokenStream2 {
    quote! {
        const _ : () = assert!(#zk_argument == __PBC_IS_ZK_CONTRACT, #error_message);
    }
}
