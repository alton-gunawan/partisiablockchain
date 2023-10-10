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
//! # Example of Average
//!
//! Simple example of a computation summing all input variables:
//!
//! ```
//! # use pbc_zk::{Sbi32, load_sbi, secret_variable_ids};
//!
//! pub fn sum_all_variables() -> Sbi32 {
//!     let mut sum: Sbi32 = Sbi32::from(0);
//!     for variable_id in secret_variable_ids() {
//!         sum = sum + load_sbi::<Sbi32>(variable_id);
//!     }
//!     sum
//! }
//! ```
//!
//! # Example of loading struct
//!
//! ```
//! # use pbc_zk::{Sbi1, Sbi32, secret_variable_ids, load_sbi, Secret};
//!
//! # #[derive(Clone)]
//! struct MyStruct {
//!     include_in_sum: Sbi1,
//!     value: Sbi32,
//! }
//!
//! # impl Secret for MyStruct {}
//!
//! pub fn sum_all_variables() -> Sbi32 {
//!     let mut sum: Sbi32 = Sbi32::from(0);
//!     for variable_id in secret_variable_ids() {
//!         let value = load_sbi::<MyStruct>(variable_id);
//!         if value.include_in_sum {
//!             sum = sum + value.value;
//!         }
//!     }
//!     sum
//! }
//! ```
#![allow(dead_code)]

#[cfg(not(doc))]
mod sbi;
#[cfg(doc)]
pub mod sbi;

extern crate pbc_zk_macros;

pub use pbc_zk_macros::{zk_compute, SecretBinary};
pub use sbi::*;
use std::thread;

/// A secret-shared [`bool`] value.
pub type Sbi1 = bool;
/// A secret-shared [`i8`] value. See [`Sbi`].
pub type Sbi8 = Sbi<i8>;
/// A secret-shared [`i16`] value. See [`Sbi`].
pub type Sbi16 = Sbi<i16>;
/// A secret-shared [`i32`] value. See [`Sbi`].
pub type Sbi32 = Sbi<i32>;
/// A secret-shared [`i64`] value. See [`Sbi`].
pub type Sbi64 = Sbi<i64>;
/// A secret-shared [`i128`] value. See [`Sbi`].
pub type Sbi128 = Sbi<i128>;

/// A secret-shared value.
pub trait SecretBinary {}
pub use crate::SecretBinary as Secret;

impl SecretBinary for Sbi1 {}
impl<NT> SecretBinary for Sbi<NT> where NT: Clone {}

pub mod api {
    //! Module for configuring outside environment of test.

    use core::any::Any;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::thread;
    use std::thread::ThreadId;

    /// The global vector of secrets
    pub(super) static mut SECRETS: Lazy<Mutex<HashMap<ThreadId, Vec<SecretVar>>>> =
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
    pub unsafe fn set_secrets(new_secrets: Vec<SecretVar>) {
        SECRETS
            .lock()
            .unwrap()
            .insert(thread::current().id(), new_secrets);
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
        MetadataT: 'static,
    >(
        new_secrets: Vec<SecretVarInput<ValueT, MetadataT>>,
    ) {
        let new_secrets2 = new_secrets
            .into_iter()
            .map(|v| SecretVar {
                value: Box::new(v.value),
                metadata: Box::new(v.metadata),
            })
            .collect();
        set_secrets(new_secrets2);
    }

    /// A secret variable containing a secret-shared value and meta-data information for the value
    ///
    /// ### Fields:
    /// * `value`: the value of the secret
    /// * `metadata`: the metadata for the value
    pub struct SecretVar {
        /// the value of the secret
        pub value: Box<dyn Any>,
        /// the metadata for the value
        pub metadata: Box<dyn Any>,
    }

    /// A secret variable containing a secret-shared value and meta-data information for the value
    ///
    /// ### Fields:
    /// * `value`: the value of the secret
    /// * `metadata`: the metadata for the value
    pub struct SecretVarInput<ValueT, MetadataT> {
        /// the value of the secret
        pub value: ValueT,
        /// the metadata for the value
        pub metadata: MetadataT,
    }

    pub(crate) fn num_secrets() -> i32 {
        unsafe {
            let map = SECRETS.lock().unwrap();
            let secret_vars = map.get(&thread::current().id()).unwrap();
            secret_vars.len() as i32
        }
    }
}

/// Get the number of secret variables.
///
/// ### Returns:
///
/// The number of secret variables.
#[deprecated(note = "use secret_variable_ids() instead when iterating over secret variables.")]
pub fn num_secret_variables() -> i32 {
    api::num_secrets()
}

/// Creates an iterator for secret variable ids.
///
/// ### Returns:
///
/// Iterator over the ids of secret variables.
pub fn secret_variable_ids() -> impl Iterator<Item = i32> {
    1..(api::num_secrets() + 1)
}

/// Conversion from [`i32`] to [`Sbi32`]. Use [`Sbi32::from`] instead.
///
/// ### Parameters:
///
/// * `val`: [`i32`], the value to be converted.
///
/// ### Returns:
///
/// The corresponding Sbi32 value.
#[deprecated(note = "Use [`Sbi32::from`] instead, which is more general")]
pub fn sbi32_from(val: i32) -> Sbi32 {
    Sbi32::from(val)
}

/// Retrieve the input from `variable_id` as [`Sbi32`]. Use [`load_sbi`] instead.
///
/// ### Parameters:
///
/// * `variable_id`: [`i32`], the id to retrieve the value from.
///
/// ### Returns:
///
/// The corresponding [`Sbi32`] value.
#[deprecated(note = "Use [`load_sbi`] instead, which is more general")]
pub fn sbi32_input(variable_id: i32) -> Sbi32 {
    load_sbi(variable_id)
}

/// Retrieve the input from `variable_id` as `T`.
///
/// ### Parameters:
///
/// * `variable_id`: [`i32`], the id to retrieve the value from.
///
/// ### Returns:
///
/// The corresponding secret value.
pub fn load_sbi<T: Secret + Clone + 'static>(variable_id: i32) -> T {
    unsafe {
        let map = api::SECRETS.lock().unwrap();
        let secret_vars = map.get(&thread::current().id()).unwrap();
        let zk_var = &secret_vars[variable_id as usize - 1];
        zk_var
            .value
            .downcast_ref::<T>()
            .expect("Loaded variable value did not have expected type")
            .clone()
    }
}

/// Retrieve the metadata from `variable_id` as [`i32`]. Use [`load_metadata`] instead.
///
/// ### Parameters:
///
/// * `variable_id`: [`i32`], the id to retrieve metadata from.
///
/// ### Returns:
///
/// The corresponding Sbi32 value.
#[deprecated(note = "Use [`load_metadata`] instead, which is more general")]
pub fn sbi32_metadata(variable_id: i32) -> i32 {
    load_metadata(variable_id)
}

/// Retrieve the metadata from `variable_id` as `T`.
///
/// ### Parameters:
///
/// * `T`: Type of metadata. Must not be secret. This cannot be enforced in Rust 1.64, but the
/// ZK-compiler can enforce it.
/// * `variable_id`: [`i32`], the id to retrieve metadata from.
///
/// ### Returns:
///
/// The corresponding metadata.
pub fn load_metadata<T: Clone + 'static>(variable_id: i32) -> T {
    unsafe {
        let map = api::SECRETS.lock().unwrap();
        let secret_vars = map.get(&thread::current().id()).unwrap();
        let zk_var = &secret_vars[variable_id as usize - 1];
        zk_var
            .metadata
            .downcast_ref::<T>()
            .expect("Loaded variable value did not have expected type")
            .clone()
    }
}
