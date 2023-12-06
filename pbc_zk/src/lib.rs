//! Test utility library for testing Zero-Knowledge computations
//!
//! **NOTICE: This library cannot perform MPC; it can only mock MPC.**
//!
//! This library tries to mimic the type system and semantics of the Zero-Knowledge compiler and
//! stack, in order allow contracts to test their contract computations without deploying on the
//! test net. This library is the first testing step for a contract, followed by
//!
//! 1. Integration testing between public contract and computation
//! 2. Full integration testing on test-net.
//!
//! This library also acts as documentation of the various builtin functions available in the
//! Zk-compiler.
//!
//! # Example Computation of variable summation
//!
//! Simple example of a computation summing all input variables:
//!
//! ```rust
//! use pbc_zk::{Sbi32, load_sbi, secret_variable_ids, zk_compute};
//!
//! #[zk_compute(shortname = 0x01)]
//! pub fn sum_all_variables() -> Sbi32 {
//!     let mut sum: Sbi32 = Sbi32::from(0);
//!     for variable_id in secret_variable_ids() {
//!         sum = sum + load_sbi::<Sbi32>(variable_id);
//!     }
//!     sum
//! }
//! ```
//!
//! # Example Computation of loading struct
//!
//! Another example that sums all values of a specific type
//!
//! ```rust
//! use pbc_zk::{Sbi1, Sbi8, Sbi32, secret_variable_ids, load_sbi, SecretBinary, zk_compute};
//!
//! #[derive(Clone, SecretBinary)]
//! struct MyStruct {
//!     variable_type: Sbi8,
//!     value: Sbi32,
//! }
//!
//! #[zk_compute(shortname = 0x02)]
//! pub fn sum_all_variables() -> Sbi32 {
//!     let mut sum: Sbi32 = Sbi32::from(0);
//!     for variable_id in secret_variable_ids() {
//!         let value = load_sbi::<MyStruct>(variable_id);
//!         if value.variable_type == Sbi8::from(0) {
//!             sum = sum + value.value;
//!         }
//!     }
//!     sum
//! }
//! ```
#![allow(dead_code)]

use std::thread;

pub use pbc_zk_core::*;

extern crate pbc_zk_macros;
pub use pbc_contract_common::zk::SecretVarId;
pub use pbc_zk_macros::{test_eq, zk_compute, SecretBinary};

use pbc_traits::ReadWriteState;

pub mod api {
    //! Module for configuring outside environment of test.

    use super::SecretVarId;
    use once_cell::sync::Lazy;
    use pbc_traits::ReadWriteState;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::thread;
    use std::thread::ThreadId;

    /// A secret variable containing a secret-shared value and meta-data information for the value
    pub struct SecretVarWithId {
        /// id of the variable
        pub id: SecretVarId,
        /// the value of the secret
        pub value: Vec<u8>,
        /// the metadata for the value
        pub metadata: Vec<u8>,
    }

    /// A secret variable containing a secret-shared value and meta-data information for the value
    #[derive(Clone)]
    pub struct SecretVar {
        /// the value of the secret
        pub value: Vec<u8>,
        /// the metadata for the value
        pub metadata: Vec<u8>,
    }

    /// A secret variable containing a secret-shared value and meta-data information for the value
    pub struct SecretVarInput<ValueT, MetadataT> {
        /// the value of the secret
        pub value: ValueT,
        /// the metadata for the value
        pub metadata: MetadataT,
    }

    /// The global vector of secrets
    pub(super) static SECRET_INPUTS: Lazy<
        Mutex<HashMap<ThreadId, HashMap<SecretVarId, SecretVar>>>,
    > = Lazy::new(|| Mutex::new(HashMap::new()));

    /// The global vector of secret outputs
    pub(super) static SECRET_OUTPUTS: Lazy<Mutex<HashMap<ThreadId, Vec<SecretVar>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    /// Sets the global secrets, should be called before testing a zk computation.
    ///
    /// # Safety
    ///
    /// Changes global state. Must be called before testing zk computation.
    ///
    /// ### Parameters:
    ///
    /// * `new_secrets`: [`Vec<SecretVar<Sbi32>>`], the new vector of secrets.
    pub unsafe fn set_secrets_with_ids(new_secrets: Vec<SecretVarWithId>) {
        let secrets_map: HashMap<SecretVarId, SecretVar> = new_secrets
            .into_iter()
            .map(|v| {
                (
                    v.id,
                    SecretVar {
                        value: v.value,
                        metadata: v.metadata,
                    },
                )
            })
            .collect();
        SECRET_INPUTS
            .lock()
            .unwrap()
            .insert(thread::current().id(), secrets_map);
    }

    /// Sets the global secrets, should be called before testing a zk computation.
    ///
    /// # Safety
    ///
    /// Changes global state. Must be called before testing zk computation.
    pub unsafe fn set_secrets(new_secrets: Vec<SecretVar>) {
        let secrets_map: HashMap<SecretVarId, SecretVar> = new_secrets
            .into_iter()
            .enumerate()
            .map(|(id, v)| (SecretVarId::new((id as u32) + 1), v))
            .collect();
        SECRET_INPUTS
            .lock()
            .unwrap()
            .insert(thread::current().id(), secrets_map);
    }

    /// Sets the global secrets, should be called before testing a zk computation.
    ///
    /// # Safety
    ///
    /// Changes global state. Must be called before testing zk computation.
    ///
    /// ### Parameters:
    ///
    /// * `ValueT`: Type of variable value.
    /// * `MetadataT`: Type of variable metadata.
    /// * `new_secrets`: The new vector of secret variables.
    pub unsafe fn set_secrets_of_single_type<
        ValueT: crate::Secret + 'static,
        MetadataT: ReadWriteState + 'static,
    >(
        new_secrets: Vec<SecretVarInput<ValueT, MetadataT>>,
    ) {
        let new_secrets2 = new_secrets
            .into_iter()
            .enumerate()
            .map(
                |(id, v): (_, SecretVarInput<ValueT, MetadataT>)| SecretVarWithId {
                    id: SecretVarId::new((id as u32) + 1),
                    value: {
                        let mut buff = vec![];
                        v.value.secret_write_to(&mut buff).unwrap();
                        buff
                    },
                    metadata: {
                        let mut buff = vec![];
                        v.metadata.state_write_to(&mut buff).unwrap();
                        buff
                    },
                },
            )
            .collect();
        set_secrets_with_ids(new_secrets2);
    }

    pub(crate) fn secret_keys() -> Vec<SecretVarId> {
        let map = SECRET_INPUTS.lock().unwrap();
        let secret_vars = map.get(&thread::current().id()).unwrap();
        secret_vars.keys().copied().collect()
    }

    /// Gets the secret outputs belonging to this thread
    pub fn get_secret_outputs() -> Vec<SecretVar> {
        SECRET_OUTPUTS
            .lock()
            .unwrap()
            .get(&thread::current().id())
            .unwrap_or(&vec![])
            .clone()
            .to_vec()
    }

    pub(crate) fn load_secret_input(variable_id: SecretVarId) -> Option<SecretVar> {
        let secret_inputs = SECRET_INPUTS.lock().ok()?;
        let secret_vars = secret_inputs.get(&thread::current().id())?;
        secret_vars.get(&variable_id).cloned()
    }

    pub(crate) fn num_secrets() -> usize {
        secret_keys().len()
    }
}

/// Get the number of secret variables.
///
/// ### Returns:
///
/// The number of secret variables.
#[deprecated(note = "use secret_variable_ids() instead when iterating over secret variables.")]
pub fn num_secret_variables() -> usize {
    api::num_secrets()
}

/// Creates an iterator for secret variable ids.
///
/// ### Returns:
///
/// Iterator over the ids of secret variables.
pub fn secret_variable_ids() -> impl Iterator<Item = SecretVarId> {
    api::secret_keys().into_iter()
}

/// Retrieve the input from `variable_id` as `T`.
///
/// ### Parameters:
///
/// * `variable_id`: [`SecretVarId`], the id to retrieve the value from.
///
/// ### Returns:
///
/// The corresponding secret value.
pub fn load_sbi<T: Secret + 'static>(variable_id: SecretVarId) -> T {
    let secret_var = api::load_secret_input(variable_id as SecretVarId)
        .unwrap_or_else(|| panic!("Could not load value for variable {}", variable_id.raw_id));
    let mut buff = secret_var.value.as_slice();
    let zk_var: T = T::secret_read_from(&mut buff);
    zk_var
}

/// Retrieve the metadata from `variable_id` as `T`.
///
/// ### Parameters:
///
/// * `T`: Type of metadata. Must not be secret. This cannot be enforced in Rust 1.64, but the
/// ZK-compiler can enforce it.
/// * `variable_id`: [`SecretVarId`], the id to retrieve metadata from.
///
/// ### Returns:
///
/// The corresponding metadata.
pub fn load_metadata<T: ReadWriteState + 'static>(variable_id: SecretVarId) -> T {
    let secret_var = api::load_secret_input(variable_id as SecretVarId).unwrap_or_else(|| {
        panic!(
            "Could not load metadata for variable {}",
            variable_id.raw_id
        )
    });
    let mut buff = secret_var.metadata.as_slice();
    let zk_var: T = T::state_read_from(&mut buff);
    zk_var
}

/// Save the secret variable as an output, allowing for variable number of outputs.
///
/// ### Parameters
///
/// * `T`: Type of the output variable. Must be a secret type.
/// * `var`: variable to be saved.
pub fn save_sbi<T: Secret + 'static>(var: T) {
    let mut buff: Vec<u8> = vec![];
    var.secret_write_to(&mut buff).unwrap();
    let secret_var = api::SecretVar {
        value: buff,
        metadata: vec![],
    };

    let mut map = api::SECRET_OUTPUTS.lock().unwrap();
    map.entry(thread::current().id())
        .or_insert(vec![])
        .push(secret_var);
}
