#![doc = include_str!("../README.md")]
#![recursion_limit = "128"]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::{parse_macro_input, AttributeArgs};

use pbc_contract_codegen_internal::{
    action_macro, callback_macro, init_macro, parse_attributes, parse_secret_type_input,
    parse_shortname_override, parse_zk_argument, state_macro, zk_macro, SecretInput,
    WrappedFunctionKind,
};
use pbc_contract_common::FunctionKind;

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
pub fn init(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    let attributes = parse_attributes(args, vec!["zk".to_string()], vec![]);
    let zk = parse_zk_argument(&attributes);
    init_macro::handle_init_macro(input, zk)
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
    let attributes = parse_attributes(
        args,
        vec!["shortname".to_string(), "zk".to_string()],
        vec![],
    );
    let shortname_override = parse_shortname_override(&attributes);
    let zk = parse_zk_argument(&attributes);
    action_macro::handle_action_macro(input, shortname_override, zk)
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
        vec!["shortname".to_string(), "zk".to_string()],
        vec!["shortname".to_string()],
    );
    let shortname_override = parse_shortname_override(&attributes);
    let zk = parse_zk_argument(&attributes);
    callback_macro::handle_callback_macro(input, shortname_override, zk)
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
/// ```ignore
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
/// ```ignore
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
#[proc_macro_attribute]
pub fn zk_on_secret_input(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    let attributes = parse_attributes(
        args,
        vec!["shortname".to_string(), "secret_type".to_string()],
        vec!["shortname".to_string()],
    );
    let shortname_override = parse_shortname_override(&attributes);
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
/// ```ignore
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
/// ```ignore
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
///         vec![ZkStateChange::start_computation(0, vec![1, 2, 3])]
///     } else {
///         vec![]
///     };
///     (state, vec![], zk_state_changes)
/// }
/// ```
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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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
