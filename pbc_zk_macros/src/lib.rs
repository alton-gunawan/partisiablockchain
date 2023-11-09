#![doc = include_str!("../README.md")]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use crate::secret_binary_derive::implement_secret;
use crate::test_eq::implement_test_eq;
use proc_macro::TokenStream;

mod secret_binary_derive;
mod test_eq;

/// Implements `#[derive(SecretBinary)]`.
#[proc_macro_derive(SecretBinary)]
pub fn derive_secret_binary(input: TokenStream) -> TokenStream {
    implement_secret(input)
}

/// Testing macro for Zero-knowledge computations.
///
/// # Usage example
///
/// ```ignore
/// # use pbc_zk_macros::zk_compute;
/// # use pbc_zk_core::Sbi32;
/// pub fn load_test() -> Sbi32 {
///     load_sbi::<Sbi32>(SecretVarId::new(1))
/// }
///
/// test_eq!(load_test(), 3, [3i32]);
/// test_eq!(load_test(), 55, [55i32]);
/// test_eq!(load_test(), 55, [55i32], []);
/// ```
#[proc_macro]
pub fn test_eq(input: TokenStream) -> TokenStream {
    implement_test_eq(input)
}

/// Marks function as a ZK computation entry point with a given shortname.
///
/// **ZK COMPUTATION HOOK**: This hook must only be annotated on ZK computations, and not on any
/// other type of function.
///
/// Creates a new function named `[variable]_start`, capable of producing an instance of
/// `ZkStateChange::StartComputation` that executes that ZK computation. The original function is
/// made private, as it can be tested using [`test_eq`](crate::test_eq!), but the original is
/// purely for testing purposes, as the `_start` function should be used to actually start the
/// zero-knowledge computation.
///
/// # Usage example
///
/// In `zk_compute.rs` define a computation function like so:
///
/// ```ignore
/// /// Computation that adds two zero-knowledge values together.
/// #[zk_compute(shortname = 0x1)]
/// fn add_variables(id1: SecretVarId, id2: SecretVarId) -> Sbi32 {
///     load_sbi::<Sbi32>(id1) + load_sbi::<Sbi32>(id1)
/// }
///
/// // Testing
/// test_eq!(add_variables(SecretVarId::new(1), SecretVarId::new(2)), 9, [3i32, 6i32]);
/// test_eq!(add_variables(SecretVarId::new(1), SecretVarId::new(1)), 10, [5i32]);
/// ```
///
/// In `contract.rs` we can then start the computation by using the `_start` function:
///
/// ```ignore
/// /// Contract invocation that immediately start summing the input with a state variable.
/// #[zk_on_variable_inputted]
/// fn output_variables(
///     context: ContractContext,
///     state: ContractState,
///     zk_state: ZkState<SecretVarMetadata>,
///     variable_id: SecretVarId,
/// ) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
///     (state, vec![], vec![
///         zk_compute::add_variables_start(state.some_other_variable_id, variable_id),
///     ])
/// }
/// ```
///
#[proc_macro_attribute]
pub fn zk_compute(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let args: syn::AttributeArgs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
    let attributes = pbc_contract_codegen_internal::parse_attributes(
        args,
        vec!["shortname".to_string()],
        vec!["shortname".to_string()],
    );
    let shortname_override =
        pbc_contract_codegen_internal::parse_shortname_override(&attributes).unwrap();

    pbc_contract_codegen_internal::zk_compute_macro::handle_zk_compute_macro(
        input,
        shortname_override,
    )
}
