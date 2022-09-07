//! Contains data structures and code for handling blockchain signatures.

use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// Represents a blockchain signature.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[derive(PartialEq, Eq, ReadWriteRPC, ReadWriteState, Debug, Clone)]
pub struct Signature {
    /// Id used to recover public key when verifying signature.
    recovery_id: u8,
    /// R value in signature.
    value_r: [u8; 32],
    /// S value in signature.
    value_s: [u8; 32],
}

impl Signature {
    /// Crate private utility for creating testing signatures.
    #[allow(dead_code)]
    pub(crate) const fn new(recovery_id: u8, value_r: [u8; 32], value_s: [u8; 32]) -> Self {
        Self {
            recovery_id,
            value_r,
            value_s,
        }
    }
}
