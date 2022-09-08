//! Library for enabling test of `zk_compute`.
//! Includes mocked functions for handling secrets.
#![allow(dead_code)]

mod sbi1;
mod sbi32;

/// The sbi1 module
pub use sbi1::*;
/// The sbi32 module
pub use sbi32::*;

/// A secret-shared value
pub trait Secret {}

/// The global vector of secrets
static mut SECRETS: Vec<SecretVar<Sbi32>> = vec![];

/// # Safety
///
/// Sets the global secrets, should be called before testing a zk computation.
///
/// ### Parameters:
///
/// * `new_secrets`: [`Vec<SecretVar<Sbi32>>`], the new vector of secrets.
///
pub unsafe fn set_secrets(new_secrets: Vec<SecretVar<Sbi32>>) {
    SECRETS = new_secrets;
}

/// Get the number of secret variables.
///
/// ### Returns:
///
/// The number of secret variables.
pub fn num_secret_variables() -> i32 {
    unsafe { SECRETS.len() as i32 }
}

/// Retrieve the input from `variable_id` as [`Sbi32`].
///
/// ### Parameters:
///
/// * `variable_id`: [`i32`], the id to retrieve the value from.
///
/// ### Returns:
///
/// The corresponding Sbi32 value.
pub fn sbi32_input(variable_id: i32) -> Sbi32 {
    unsafe { SECRETS[variable_id as usize - 1].value }
}

/// Retrieve the metadata from `variable_id` as [`i32`].
///
/// ### Parameters:
///
/// * `variable_id`: [`i32`], the id to retrieve metadata from.
///
/// ### Returns:
///
/// The corresponding Sbi32 value.
pub fn sbi32_metadata(variable_id: i32) -> i32 {
    unsafe { SECRETS[variable_id as usize - 1].metadata }
}

/// A secret variable containing a secret-shared value and meta-data information for the value
///
/// ### Fields:
/// * `value`: [`Secret`], the value of the secret
/// * `metadata`: [`i32`], the metadata for the value
///
pub struct SecretVar<T: Secret> {
    /// the value of the secret
    pub value: T,
    /// the metadata for the value
    pub metadata: i32,
}
