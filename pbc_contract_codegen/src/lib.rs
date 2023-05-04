//! Partisia Blockchain SDK Macro Crate
//!
//! Defines the ABI attribute macros:
//!
//! - [`macro@state`] declares how the contract represents its state.
//! - [`macro@init`] declares the code run when the contract is initialized.
//! - [`macro@action`] declares an endpoint that the contract can be interacted with by.
//! - [`macro@callback`] declares a callback hook.
//!
//! Additionally defines the zero-knowledge lifetime attribute macros:
//!
//! - [`macro@zk_on_secret_input`] declares an endpoint that the contract can be interacted with to add secret variables.
//! - [`macro@zk_on_variable_inputted`] declares an automatic hook for when a variable is confirmed inputted.
//! - [`macro@zk_on_variable_rejected`] declares an automatic hook for when a variable is rejected.
//! - [`macro@zk_on_compute_complete`] declares an automatic hook for when the zero-knowledge computation is finished.
//! - [`macro@zk_on_variables_opened`] declares an automatic hook for when one of the contract's own secret variables is ready to be read.
//! - [`macro@zk_on_user_variables_opened`] declares an automatic hook for when a user opens one of their variables and is ready to be read.
//! - [`macro@zk_on_attestation_complete`] declares an automatic hook for when the contract have
//! asked nodes to attest a piece of data, and this process have completed.
//!
//! This crate can automatically produce [ABI files](https://privacyblockchain.gitlab.io/language/rust-contract-sdk/abiv1.html),
//! and serialization boilerplate for actions. Additionally, the crate will type check the
//! function signatures of the annotated functions, to guarantee that the contract can interact
//! correctly with the blockchain.

#![recursion_limit = "128"]
extern crate pbc_external;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate sha2;
extern crate syn;

use proc_macro::TokenStream;
use std::cmp::max;
use std::collections::HashMap;

use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{
    parse_macro_input, AttributeArgs, FnArg, Ident, Lit, Meta, NestedMeta, Type, TypeArray,
    TypePath,
};

use crate::tokenized::{ArgumentList, InstantiableArgument, TokenizedInvocation};
use pbc_contract_common::address::Shortname;
use pbc_contract_common::{FunctionKind, FunctionName};

mod action_macro;
mod callback_macro;
mod init_macro;
mod macro_abi;
mod state_macro;
mod tokenized;
mod version;
#[cfg(feature = "zk")]
mod zk_macro;

/// Parses the attributes of a macro into a map from attribute names to a `Literal`.
///
/// ### Parameters:
///
/// * `args`: [`AttributeArgs`] - the args to be parsed.
///
/// * `valid_names`: [Vec<String>] - valid names of attributes. Panics if an attribute name not
/// in valid_names is present in args.
///
/// * `required_names`: [Vec<String>] - required names. Panics if any of the attribute names is
/// not present in args.
///
///  ### Returns
/// A map from attribute name to their value.
fn parse_attributes(
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

/// State contract annotation
///
/// **REQUIRED ANNOTATION**: This is a required annotated. A contract cannot be created without
/// a state.
///
/// Declares that the annotated struct is the top level of the contract state. This
/// macro must occur exactly once in any given contract.
///
/// # Example
///
/// ```ignore
/// # use pbc_contract_common::address::Address;
/// # use pbc_contract_codegen::state;
/// # use pbc_contract_common::sorted_vec_map::SortedVecMap;
/// #[state]
/// pub struct VotingContractState {
///     proposal_id: u64,
///     mp_addresses: Vec<Address>,
///     votes: SortedVecMap<Address, u8>,
///     closed: u8,
/// }
/// ```
///
/// This macro implicitly derives [`ReadWriteState`](pbc_traits::ReadWriteState) for the struct.
/// The [`ReadWriteState`](pbc_traits::ReadWriteState) derive may fail if any of the state struct's
/// fields aren't impl [`ReadWriteState`](pbc_traits::ReadWriteState).
///
/// Furthermore, note that state serialization speeds are heavily affected by the types contained
/// in the state struct. Types with dynamic sizes ([`Option<T>`], [`String`]) and/or global
/// invariants ([`BTreeSet<T>`](std::collections::BTreeSet))
/// are especially slow. For more background, see
/// [`ReadWriteState::SERIALIZABLE_BY_COPY`](pbc_traits::ReadWriteState::SERIALIZABLE_BY_COPY)
#[proc_macro_attribute]
pub fn state(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    state_macro::handle_state_macro(input)
}

/// Initializer contract annotation
///
/// **REQUIRED HOOK**: This is a required hook. A contract cannot be created without an
/// initializer.
///
/// Similar to [`macro@action`], but declares how the contract can be initialized. Must occur exactly once in any given contract.
///
/// Annotated function must have a signature of following format:
///
/// ```ignore
/// # use pbc_contract_codegen::init;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
///
/// #[init]
/// pub fn initialize(
///     context: ContractContext,
///     // ... Initialization/RPC arguments
/// ) -> (ContractState, Vec<EventGroup>)
/// # { (0, vec![]) }
/// ```
///
/// with following constraints:
///
/// - `ContractState` must be the type annotated with [`macro@state`], and must have an
///   [`pbc_traits::ReadWriteState`] implementation.
/// - All initialization arguments must have a [`pbc_traits::ReadRPC`]
///   and a [`pbc_traits::WriteRPC`] implementation.
///
/// Note that there are no previous state when initializing, in contrast to the
/// [`macro@action`] macro. If the initializer fails the contract will not be created.
#[proc_macro_attribute]
pub fn init(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    init_macro::handle_init_macro(input)
}

/// Public action contract annotation
///
/// **OPTIONAL HOOK?**: This is technically an optional hook, but a contract without a action hooks
/// is of limited use.
///
/// Annotated function is a contract action that can be called from other contracts and dashboard.
///
/// Must have a signature of the following format:
///
/// ```ignore
/// # use pbc_contract_codegen::action;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[action]
/// pub fn action_internal_name(
///   context: ContractContext,
///   state: ContractState,
///   // ... RPC arguments
/// ) -> (ContractState, Vec<EventGroup>)
/// # { (0, vec![]) }
/// ```
///
/// with the following constraints:
///
/// - `ContractState` must be the type annotated with [`macro@state`], and must have an
///   [`pbc_traits::ReadWriteState`] implementation.
/// - All initialization arguments must have a [`pbc_traits::ReadRPC`]
///   and a [`pbc_traits::WriteRPC`] implementation.
///
/// The action receives the previous state, along with a context, and the declared
/// arguments, and must return the new state, along with a vector of
/// [`pbc_contract_common::events::EventGroup`]; a list of interactions with other contracts.
///
/// # Example
///
/// ```ignore
/// # use pbc_contract_codegen::action;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::events::*;
/// # use pbc_contract_common::address::*;
/// # use pbc_contract_common::map::sorted_vec_map::SortedVecMap;
/// type VotingContractState = SortedVecMap<Address, bool>;
/// # type Metadata = u32;
///
/// #[action]
/// pub fn vote(
///     context: ContractContext,
///     mut state: VotingContractState,
///     vote: bool,
/// ) -> VotingContractState {
///     state.insert(context.sender, vote);
///     state
/// }
/// ```
///
/// # Shortname
///
/// In addition to the readable name, each action needs a shortname, a small unique identifier.
/// This shortname is automatically generated by default, but for cases where a specific shortname
/// is desirable, it can be set using the `shortname = <shortname>` attribute.
/// This has to be a [`u32`] and gets encoded as LEB128 (up to 5 bytes). These bytes are then
/// encoded as lowercase zero-padded hex.
///
/// For example:
///
/// ```ignore
/// # use pbc_contract_codegen::action;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// type Metadata = u32;
///
/// #[action(shortname = 53)]
/// pub fn some_action(
///     context: ContractContext,
///     mut state: ContractState,
/// ) -> (ContractState, Vec<EventGroup>) {
///   // Do things
///   (state, vec![])
/// }
/// ```
#[proc_macro_attribute]
pub fn action(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    let attributes = parse_attributes(args, vec!["shortname".to_string()], vec![]);
    let shortname_override = parse_shortname_override(attributes);
    action_macro::handle_action_macro(input, shortname_override)
}

/// Public callback contract annotation
///
/// **OPTIONAL HOOK**: This is an optional hook, only required if the contract needs callback
/// functionality.
///
/// Annotated function is a callback from an event sent by this contract.  Unlike actions,
/// callbacks must specify their shortname explicitly.
///
/// Must have a signature of the following format:
///
/// ```ignore
/// # use pbc_contract_codegen::callback;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState  = u32;
/// # type Metadata = u32;
/// #[callback(shortname = 13)]
/// pub fn callback_internal_name(
///   contract_context: ContractContext,
///   callback_context: CallbackContext,
///   state: ContractState,
///   // ... RPC arguments
/// ) -> (ContractState, Vec<EventGroup>)
/// # { (0, vec![]) }
/// ```
///
/// with following constraints:
///
/// - `ContractState` must be the type annotated with [`macro@state`], and must have an
///   [`pbc_traits::ReadWriteState`] implementation.
/// - All initialization arguments must have a [`pbc_traits::ReadRPC`]
///   and a [`pbc_traits::WriteRPC`] implementation.
///
/// The callback receives the previous state, along with two context objects, and the declared
/// arguments. The [`CallbackContext`](pbc_contract_common::context::CallbackContext) object contains the execution status of all the events
/// sent by the original transaction.
/// Just like actions, callbacks must return the new state, along with a vector of
/// [`EventGroup`](pbc_contract_common::events::EventGroup); a list of interactions with other contracts.
///
/// # Shortname
///
/// In addition to the readable name the callback needs a shortname, a small unique identifier.
/// This shortname must be set using the `shortname = <shortname>` attribute.
/// This has to be a [`u32`] and gets encoded as LEB128 (up to 5 bytes). These bytes are then
/// encoded as lowercase zero-padded hex.
#[proc_macro_attribute]
pub fn callback(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    let attributes = parse_attributes(
        args,
        vec!["shortname".to_string()],
        vec!["shortname".to_string()],
    );
    let shortname_override = parse_shortname_override(attributes);
    callback_macro::handle_callback_macro(input, shortname_override)
}

/// Secret input/action contract annotation
///
/// **OPTIONAL HOOK?**: This is technically an optional hook, but a zero-knowledge contract without
/// a secret input hook is of limited use.
///
/// Annotated function is a contract action that allows a user to deliver secret input to the
/// contract. Can be thought of as the Zk variant of [`macro@action`]. The notable change is the
/// introduction of a required return value, of type
/// [`ZkInputDef`](pbc_contract_common::zk::ZkInputDef), that contains contract-supplied metadata,
/// along with some other configuration for the secret variable.
///
/// The input variable will be rejected if the annotated function panics, so it might be
/// appropriate to deliberately panic.
///
/// Must have a signature of the following format:
///
/// ```
/// # use pbc_contract_codegen::zk_on_secret_input;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[zk_on_secret_input(shortname = 0xDEADB00F)]
/// pub fn function_name(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   // ... RPC arguments.
/// ) -> (ContractState, Vec<EventGroup>, ZkInputDef<Metadata>)
/// # { (state, vec![], ZkInputDef { expected_bit_lengths: vec![], seal: false, metadata: 0 } ) }
/// ```
///
/// with following constraints:
///
/// - `ContractState` must be the type annotated with [`macro@state`], and must have an
///   [`pbc_traits::ReadWriteState`] implementation.
/// - All initialization arguments must have a [`pbc_traits::ReadRPC`]
///   and a [`pbc_traits::WriteRPC`] implementation.
/// - The `Metadata` type given to `ZkState` and `ZkInputDef` must be identical both for individual
///   functions, and across the entire contract.
/// - This function is only available with the `zk` feature enabled.
///
/// The function receives the previous states `ContractState` and
/// [`ZkState<Metadata>`](pbc_contract_common::zk::ZkState), along with the
/// [`ContractContext`](pbc_contract_common::context::ContractContext), and the declared RPC
/// arguments.
///
/// The function must return a tuple containing:
///
/// - New public state.
/// - Vector of [`EventGroup`](pbc_contract_common::events::EventGroup); a list of interactions with other contracts.
/// - Instance of [`ZkInputDef<Metadata>`](pbc_contract_common::zk::ZkInputDef) for declaring the
///   layout and metadata of a secret variable.
///
/// # Example
///
/// ```
/// # use pbc_contract_codegen::zk_on_secret_input;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// type ContractState = u32;
/// type Metadata = u32;
///
/// #[zk_on_secret_input(shortname = 0x13)]
/// pub fn receive_bitlengths_10_10(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<u32>,
/// ) -> (ContractState, Vec<EventGroup>, ZkInputDef<u32>) {
///     let def = ZkInputDef {
///         seal: false,
///         expected_bit_lengths: vec![10, 10],
///         metadata: 23u32,
///     };
///     (state, vec![], def)
/// }
/// ```
#[cfg(feature = "zk")]
#[proc_macro_attribute]
pub fn zk_on_secret_input(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    let attributes = parse_attributes(
        args,
        vec!["shortname".to_string(), "secret_type".to_string()],
        vec!["shortname".to_string()],
    );
    let shortname_override = parse_shortname_override(attributes.clone());
    let secret_type_input = parse_secret_type_input(attributes);

    let function_kind = WrappedFunctionKind {
        output_state_and_events: true,
        min_allowed_num_results: 3,
        output_other_types: vec![(
            quote! { pbc_contract_common::zk::ZkInputDef<_> },
            format_ident!("write_zk_input_def_result"),
        )],
        system_arguments: 3,
        fn_kind: FunctionKind::ZkSecretInputWithExplicitType,
        allow_rpc_arguments: true,
    };
    zk_macro::handle_zk_macro(
        input,
        shortname_override,
        "zk_on_secret_input",
        &function_kind,
        true,
        secret_type_input,
    )
}

/// Secret variable input zero-knowledge contract annotation
///
/// **OPTIONAL HOOK**: This is an optional hook, and is not required for a well-formed
/// zero-knowledge contract. The default behaviour is to do nothing.
///
/// Annotated function is automatically called when a Zk variable is confirmed and fully inputted.
/// This hook is exclusively called by the blockchain itself, and cannot be called manually from
/// the dashboard, nor from another contract.
/// Allows the contract to automatically choose some action to take.
///
/// Must have a signature of the following format:
///
/// ```
/// # use pbc_contract_codegen::zk_on_variable_inputted;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[zk_on_variable_inputted]
/// pub fn zk_on_variable_inputted(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   variable_id: SecretVarId,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>)
/// # { (state, vec![], vec![]) }
/// ```
///
/// with following constraints:
///
/// - `ContractState` must be the type annotated with [`macro@state`], and must have an
///   [`pbc_traits::ReadWriteState`] implementation.
/// - The `Metadata` type given to `ZkState` and `ZkInputDef` must be identical both for individual
///   functions, and across the entire contract.
/// - This function is only available with the `zk` feature enabled.
///
/// The function receives:
/// - `ContractState`: The previous states.
/// - [`ZkState<Metadata>`](pbc_contract_common::zk::ZkState): The current state of the zk computation.
/// - [`ContractContext`](pbc_contract_common::context::ContractContext): The current contract context.
/// - [`SecretVarId`](pbc_contract_common::zk::SecretVarId): Id of the variable.
///
/// The function must return a tuple containing:
///
/// - New public state.
/// - Vector of [`EventGroup`](pbc_contract_common::events::EventGroup); a list of interactions with other contracts.
/// - [`Vec<ZkStateChange>`](pbc_contract_common::zk::ZkStateChange) declaring how to change the zk contract state.
///
/// # Example
///
/// This hook is commonly used to start the computation when enough inputs have been given, as
/// demonstrated in the following example:
///
/// ```
/// # use pbc_contract_codegen::zk_on_variable_inputted;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// type ContractState = u32;
/// type Metadata = u32;
///
/// #[zk_on_variable_inputted]
/// pub fn zk_on_variable_inputted(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   variable_id: SecretVarId,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
///     let zk_state_changes = if (zk_state.secret_variables.len() > 5) {
///         vec![ZkStateChange::start_computation(vec![1, 2, 3])]
///     } else {
///         vec![]
///     };
///     (state, vec![], zk_state_changes)
/// }
/// ```
#[cfg(feature = "zk")]
#[proc_macro_attribute]
pub fn zk_on_variable_inputted(attrs: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        attrs.is_empty(),
        "No attributes are supported for zk_on_variable_inputted"
    );
    let function_kind = WrappedFunctionKind {
        output_state_and_events: true,
        min_allowed_num_results: 1,
        output_other_types: vec![(
            quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
            format_ident!("write_zk_state_change"),
        )],
        system_arguments: 4,
        fn_kind: FunctionKind::ZkVarInputted,
        allow_rpc_arguments: false,
    };
    zk_macro::handle_zk_macro(
        input,
        None,
        "zk_on_variable_inputted",
        &function_kind,
        false,
        SecretInput::None,
    )
}

/// Secret variable rejection zero-knowledge contract annotation
///
/// **OPTIONAL HOOK**: This is an optional hook, and is not required for a well-formed
/// zero-knowledge contract. The default behaviour is to do nothing.
///
/// Annotated function is automatically called when a Zk variable is rejected for any reason.
/// This hook is exclusively called by the blockchain itself, and cannot be called manually from
/// the dashboard, nor from another contract.
/// Allows the contract to automatically choose some action to take.
///
/// Must have a signature of the following format:
///
/// ```
/// # use pbc_contract_codegen::zk_on_variable_rejected;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[zk_on_variable_rejected]
/// pub fn zk_on_variable_rejected(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   variable_id: SecretVarId,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>)
/// # { (state, vec![], vec![]) }
/// ```
#[cfg(feature = "zk")]
#[proc_macro_attribute]
pub fn zk_on_variable_rejected(attrs: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        attrs.is_empty(),
        "No attributes are supported for zk_on_variable_rejected"
    );
    let function_kind = WrappedFunctionKind {
        output_state_and_events: true,
        min_allowed_num_results: 1,
        output_other_types: vec![(
            quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
            format_ident!("write_zk_state_change"),
        )],
        system_arguments: 4,
        fn_kind: FunctionKind::ZkVarRejected,
        allow_rpc_arguments: false,
    };
    zk_macro::handle_zk_macro(
        input,
        None,
        "zk_on_variable_rejected",
        &function_kind,
        false,
        SecretInput::None,
    )
}

/// Computation complete zero-knowledge contract annotation
///
/// **OPTIONAL HOOK**: This is an optional hook, and is not required for a well-formed
/// zero-knowledge contract. The default behaviour is to do nothing.
///
/// Annotated function is automatically called when a zero-knowledge computation is finished; this
/// can only happen after the use of
/// [`ZkStateChange::StartComputation`](pbc_contract_common::zk::ZkStateChange::StartComputation).
/// This hook is exclusively called by the blockchain itself, and cannot be called manually from
/// the dashboard, nor from another contract.
/// Allows the contract to automatically choose some action to take.
///
/// Must have a signature of the following format:
///
/// ```
/// # use pbc_contract_codegen::zk_on_compute_complete;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[zk_on_compute_complete]
/// pub fn function_name(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   created_variables: Vec<SecretVarId>,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>)
/// # { (state, vec![], vec![]) }
/// ```
///
/// with following constraints:
///
/// - `ContractState` must be the type annotated with [`macro@state`], and must have an
///   [`pbc_traits::ReadWriteState`] implementation.
/// - The `Metadata` type given to `ZkState` and `ZkInputDef` must be identical both for individual
///   functions, and across the entire contract.
/// - This function is only available with the `zk` feature enabled.
///
/// The function receives:
/// - `ContractState`: The previous states.
/// - [`ZkState<Metadata>`](pbc_contract_common::zk::ZkState): The current state of the zk computation.
/// - [`ContractContext`](pbc_contract_common::context::ContractContext): The current contract context.
/// - [`Vec<SecretVarId>`](pbc_contract_common::zk::SecretVarId): Ids of the computation's output variables.
///
/// The function must return a tuple containing:
///
/// - New public state.
/// - Vector of [`EventGroup`](pbc_contract_common::events::EventGroup); a list of interactions with other contracts.
/// - [`Vec<ZkStateChange>`](pbc_contract_common::zk::ZkStateChange) declaring how to change the zk contract state.
///
/// # Example
///
/// A commonly used pattern is to open the output variables given to `zk_on_compute_complete`, as
/// demonstrated in the following example:
///
/// ```
/// # use pbc_contract_codegen::zk_on_compute_complete;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// type ContractState = u32;
/// type Metadata = u32;
///
/// #[zk_on_compute_complete]
/// pub fn zk_on_compute_complete(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   created_variables: Vec<SecretVarId>,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
///     (state, vec![], vec![ZkStateChange::OpenVariables { variables: created_variables }])
/// }
/// ```
#[cfg(feature = "zk")]
#[proc_macro_attribute]
pub fn zk_on_compute_complete(attrs: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        attrs.is_empty(),
        "No attributes are supported for zk_on_compute_complete"
    );
    let function_kind = WrappedFunctionKind {
        output_state_and_events: true,
        min_allowed_num_results: 1,
        output_other_types: vec![(
            quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
            format_ident!("write_zk_state_change"),
        )],
        system_arguments: 4,
        fn_kind: FunctionKind::ZkComputeComplete,
        allow_rpc_arguments: false,
    };
    zk_macro::handle_zk_macro(
        input,
        None,
        "zk_on_compute_complete",
        &function_kind,
        false,
        SecretInput::None,
    )
}

/// Secret user variable opened zero-knowledge contract annotation
///
/// **OPTIONAL HOOK**: This is an optional hook, and is not required for a well-formed
/// zero-knowledge contract. The default behaviour is to do nothing.
///
/// Annotated function is automatically called when a user opens one or more of their secret
/// variables.
/// This hook is exclusively called by the blockchain itself, and cannot be called manually from
/// the dashboard, nor from another contract.
/// Allows the contract to automatically choose some action to take.
///
/// Annotated function must have a signature of following format:
///
/// ```
/// # use pbc_contract_codegen::zk_on_user_variables_opened;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[zk_on_user_variables_opened]
/// pub fn zk_on_user_variables_opened(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   opened_variables: Vec<SecretVarId>,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>)
/// # { (state, vec![], vec![]) }
/// ```
///
/// Where `opened_variables` is a [`Vec`] of the opened variables.
#[cfg(feature = "zk")]
#[proc_macro_attribute]
pub fn zk_on_user_variables_opened(attrs: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        attrs.is_empty(),
        "No attributes are supported for zk_on_user_variables_opened"
    );
    let function_kind = WrappedFunctionKind {
        output_state_and_events: true,
        min_allowed_num_results: 1,
        output_other_types: vec![(
            quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
            format_ident!("write_zk_state_change"),
        )],
        system_arguments: 4,
        fn_kind: FunctionKind::ZkUserVarOpened,
        allow_rpc_arguments: true,
    };
    zk_macro::handle_zk_macro(
        input,
        None,
        "zk_on_user_variables_opened",
        &function_kind,
        false,
        SecretInput::None,
    )
}

/// Secret variable opened zero-knowledge contract annotation
///
/// **OPTIONAL HOOK**: This is an optional hook, and is not required for a well-formed
/// zero-knowledge contract. The default behaviour is to do nothing.
///
/// Annotated function is automatically called when a contract opens one or more secret
/// variables; this can only happen after the use of
/// [`ZkStateChange::OpenVariables`](pbc_contract_common::zk::ZkStateChange::OpenVariables).
/// This hook is exclusively called by the blockchain itself, and cannot be called manually from
/// the dashboard, nor from another contract.
/// Allows the contract to automatically choose some action to take.
///
/// Annotated function must have a signature of following format:
///
/// ```
/// # use pbc_contract_codegen::zk_on_variables_opened;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[zk_on_variables_opened]
/// pub fn zk_on_variables_opened(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   opened_variables: Vec<SecretVarId>,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>)
/// # { (state, vec![], vec![]) }
/// ```
///
/// Where `opened_variables` is a [`Vec`] of the opened variables.
///
/// # Example
///
/// Common usages include post-processing of computation results; for example
///
/// ```
/// # use pbc_contract_codegen::zk_on_variables_opened;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = Vec<Vec<u8>>;
/// # type Metadata = u32;
/// #[zk_on_variables_opened]
/// pub fn zk_on_sum_variable_opened(
///   context: ContractContext,
///   mut state: ContractState,
///   zk_state: ZkState<Metadata>,
///   opened_variables: Vec<SecretVarId>,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
///     let result_var_id: SecretVarId = *opened_variables.get(0).unwrap();
///     let result_var: &ZkClosed<Metadata> = zk_state.get_variable(result_var_id).unwrap();
///     let result: Vec<u8> = result_var.data.as_ref().unwrap().clone();
///     state.push(result);
///     (state, vec![], vec![])
/// }
/// ```
#[cfg(feature = "zk")]
#[proc_macro_attribute]
pub fn zk_on_variables_opened(attrs: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        attrs.is_empty(),
        "No attributes are supported for zk_on_variables_opened"
    );
    let function_kind = WrappedFunctionKind {
        output_state_and_events: true,
        min_allowed_num_results: 1,
        output_other_types: vec![(
            quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
            format_ident!("write_zk_state_change"),
        )],
        system_arguments: 4,
        fn_kind: FunctionKind::ZkVarOpened,
        allow_rpc_arguments: false,
    };
    zk_macro::handle_zk_macro(
        input,
        None,
        "zk_on_variables_opened",
        &function_kind,
        false,
        SecretInput::None,
    )
}

/// Data-attestation complete zero-knowledge contract annotation
///
/// **OPTIONAL HOOK**: This is an optional hook, and is not required for a well-formed
/// zero-knowledge contract. The default behaviour is to do nothing.
///
/// Annotated function is automatically called when the contract is informed of the availability of
/// signatures for attested data.  This can only happen after the use of
/// [`ZkStateChange::Attest`](pbc_contract_common::zk::ZkStateChange::Attest).  This hook is
/// exclusively called by the blockchain itself, and cannot be called manually from the dashboard,
/// nor from another contract.
/// Allows the contract to automatically choose some action to take.
///
/// Annotated function must have a signature of following format:
///
/// ```
/// # use pbc_contract_codegen::zk_on_attestation_complete;
/// # use pbc_contract_common::context::*;
/// # use pbc_contract_common::zk::*;
/// # use pbc_contract_common::events::*;
/// # type ContractState = u32;
/// # type Metadata = u32;
/// #[zk_on_attestation_complete]
/// pub fn zk_on_attestation_complete(
///   context: ContractContext,
///   state: ContractState,
///   zk_state: ZkState<Metadata>,
///   attestation_id: AttestationId,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>)
/// # { (state, vec![], vec![]) }
/// ```
///
/// Where `ZkState` can be further accessed to determine signatures, etc.
#[cfg(feature = "zk")]
#[proc_macro_attribute]
pub fn zk_on_attestation_complete(attrs: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        attrs.is_empty(),
        "No attributes are supported for zk_on_attestation_complete"
    );
    let function_kind = WrappedFunctionKind {
        output_state_and_events: true,
        min_allowed_num_results: 1,
        output_other_types: vec![(
            quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
            format_ident!("write_zk_state_change"),
        )],
        system_arguments: 4,
        fn_kind: FunctionKind::ZkAttestationComplete,
        allow_rpc_arguments: false,
    };
    zk_macro::handle_zk_macro(
        input,
        None,
        "zk_on_attestation_complete",
        &function_kind,
        false,
        SecretInput::None,
    )
}

/// Gets the shortname attribute of the arguments and if present parses it into a `Shortname`.
/// Panics if the attribute is not a valid shortname literal.
///
/// ### Parameters:
///
/// * `args`: [HashMap<String, Lit>] - parsed attributes of a macro.
///
/// ### Returns
/// Some of the parsed shortname if present in args, None if it is not present.
fn parse_shortname_override(args: HashMap<String, Lit>) -> Option<Shortname> {
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
#[cfg(feature = "zk")]
fn parse_secret_type_input(args: HashMap<String, Lit>) -> SecretInput {
    match args.get("secret_type") {
        None => SecretInput::Default,
        Some(Lit::Str(lit_str)) => SecretInput::Some(lit_str.value()),
        Some(err) => panic!(
            "Invalid type, expecting a string literal, but got: {}",
            err.to_token_stream()
        ),
    }
}

pub(crate) enum SecretInput {
    None,
    #[cfg(feature = "zk")]
    Default,
    #[cfg(feature = "zk")]
    Some(String),
}

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

struct WrappedFunctionKind {
    output_state_and_events: bool,
    output_other_types: Vec<(TokenStream2, Ident)>,
    min_allowed_num_results: usize,
    /// Number of "system" arguments before RPC arguments occur
    pub(crate) system_arguments: usize,
    pub(crate) fn_kind: FunctionKind,
    /// If `false`, RPC arguments are disallowed, and only system arguments must occur.
    allow_rpc_arguments: bool,
}

impl WrappedFunctionKind {
    fn public_contract_hook_kind(system_arguments: usize, fn_kind: FunctionKind) -> Self {
        // Without Zk we only need state, events optional.
        #[cfg(not(feature = "zk"))]
        let output_other_types = vec![];

        // With Zk we only need state, events optional, and ZkStateChange optional.
        #[cfg(feature = "zk")]
        let output_other_types = vec![(
            quote! { Vec<pbc_contract_common::zk::ZkStateChange> },
            format_ident!("write_zk_state_change"),
        )];

        Self {
            output_state_and_events: true,
            output_other_types,
            min_allowed_num_results: 1,
            system_arguments: system_arguments + usize::from(cfg!(feature = "zk")),
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

#[allow(clippy::too_many_arguments)]
fn wrap_function_for_export(
    fn_identifier: &Ident,
    export_symbol: Ident,
    docs: &str,
    arguments: TokenizedInvocation,
    function_kind: &WrappedFunctionKind,
) -> TokenStream2 {
    // Check that function is well-formed
    if arguments.num_params() > function_kind.system_arguments && !function_kind.allow_rpc_arguments
    {
        panic!(
            "Functions annotated with this macro must have at most {} arguments",
            function_kind.system_arguments,
        );
    }

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

pub(crate) enum CallType {
    Init,
    Action,
    Callback,
}

pub(crate) struct Names {
    fn_identifier: Ident,
    function_name: FunctionName,
    export_symbol: Ident,
}

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

fn variables_for_inner_call(item: &syn::ItemFn, call_type: CallType) -> TokenizedInvocation {
    let require_zk_state = cfg!(feature = "zk");

    // Constants by CallType
    let expected_min_arguments = match call_type {
        CallType::Init => 1,
        CallType::Action => 2,
        CallType::Callback => 3,
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
    let callback_context = match call_type {
        CallType::Callback => {
            let token = item_iterator
                .next()
                .expect("Callbacks must possess a CallbackContext argument");
            let callback = read_arguments_for_instantiation(token, false);
            Some(callback)
        }
        _ => None,
    };

    let state = match call_type {
        CallType::Action | CallType::Callback => {
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

    fn determine_result_types(t: &Type) -> Vec<Type> {
        match t {
            Type::Tuple(syn::TypeTuple { elems, .. }) => elems.iter().cloned().collect(),
            Type::Paren(syn::TypeParen { elem, .. }) => determine_result_types(elem),
            some_type => {
                vec![some_type.clone()]
            }
        }
    }

    let result_types: Vec<Type> = match &item.sig.output {
        syn::ReturnType::Default => vec![],
        syn::ReturnType::Type(_, t) => determine_result_types(t),
    };

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
        match token {
            FnArg::Receiver(_) => {
                panic!("Contract functions must be bare functions.")
            }

            FnArg::Typed(pat) => {
                let identifier = pat.pat.to_token_stream();
                let ty = pat.ty.to_token_stream();
                arguments.push(identifier, ty);
            }
        }
    }

    arguments.split_off(skip)
}

fn read_arguments_for_instantiation(token: &FnArg, is_state: bool) -> InstantiableArgument {
    match token {
        FnArg::Receiver(_) => {
            panic!("Contract functions must be bare functions.")
        }
        FnArg::Typed(pat) => {
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

#[cfg(test)]
mod test {
    #[test]
    fn failing_actions() {
        let t = trybuild::TestCases::new();
        #[cfg(not(feature = "zk"))]
        t.compile_fail("tests/action/fail/*.rs");
        #[cfg(feature = "zk")]
        t.compile_fail("tests/action/fail/zk/*.rs");
        #[cfg(not(feature = "zk"))]
        t.pass("tests/action/success/*.rs");
        #[cfg(feature = "zk")]
        t.pass("tests/action/success/zk/*.rs");
    }
}
