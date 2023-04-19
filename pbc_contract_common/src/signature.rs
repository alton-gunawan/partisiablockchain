//! Contains data structures and code for handling blockchain signatures.

#[cfg(feature = "abi")]
use crate::type_spec_default_impl;

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;
use read_write_state_derive::ReadWriteState;

/// A signature is used to authenticate the sender of a transaction on the blockchain.
///
/// It consists of a 65 byte array.
#[derive(PartialEq, Eq, ReadRPC, WriteRPC, ReadWriteState, Debug, Clone, PartialOrd, Ord)]
pub struct Signature {
    /// Id used to recover public key when verifying signature.
    pub recovery_id: u8,
    /// R value in signature.
    pub value_r: [u8; 32],
    /// S value in signature.
    pub value_s: [u8; 32],
}

#[cfg(feature = "abi")]
impl CreateTypeSpec for Signature {
    type_spec_default_impl!("Signature", 0x15);
}
