#[cfg(feature = "abi")]
use crate::type_spec_default_impl;

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;
use read_write_state_derive::ReadWriteState;

/// An address identifier is a 20 byte array derived from the hash of the public key of
/// an account.
pub type Identifier = [u8; 20];

#[cfg(feature = "abi")]
impl CreateTypeSpec for Address {
    type_spec_default_impl!("Address", 0x0d);
}

/// Represents the type of a blockchain address.
///
/// Serializable with both RPC format and State format, guaranteed identical representation.
#[repr(u8)]
#[derive(Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, ReadWriteState, ReadRPC, WriteRPC)]
pub enum AddressType {
    /// Identifies a user/service account. Identifier is prefixed with `0x00`.
    Account = 0x00,
    /// Identifies a system contract. Identifier is prefixed with `0x01`.
    SystemContract = 0x01,
    /// Identifies a public contract. Identifier is prefixed with `0x02`.
    PublicContract = 0x02,
    /// Identifies a zero knowledge contract. Identifier is prefixed with `0x03`.
    ZkContract = 0x03,
}

/// Represents a blockchain address.
///
/// Serializable with both RPC format and State format, guaranteed identical representation.
#[repr(C)]
#[derive(Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, ReadRPC, WriteRPC, ReadWriteState)]
pub struct Address {
    /// The type of the blockchain address
    pub address_type: AddressType,
    /// The embedded identifier of the blockchain address
    pub identifier: Identifier,
}
