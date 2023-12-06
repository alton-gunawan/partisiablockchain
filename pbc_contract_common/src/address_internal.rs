#[cfg(feature = "abi")]
use crate::type_spec_default_impl;

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;
use read_write_state_derive::ReadWriteState;

use std::fmt;

/// An address identifier is a 20 byte array derived from the hash of the public key of
/// an account.
pub type Identifier = [u8; 20];

#[cfg(feature = "abi")]
impl CreateTypeSpec for Address {
    type_spec_default_impl!("Address", 0x0d);
}

/// Indicates whether an [`Address`] identifies a user account or a contract.
///
/// ## Serialization
///
/// Serializable with both RPC format and State format, guaranteed identical representation.
#[repr(u8)]
#[derive(
    Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, ReadWriteState, ReadRPC, WriteRPC, Hash,
)]
pub enum AddressType {
    /// Specifies that the [`Address`] identifies an end user or service account.
    ///
    /// Every invocation coming from outside Partisia BlockChain will use such an account address.
    ///
    /// When [`sender`](crate::context::ContractContext::sender) contains this kind of address, it indicates that the
    /// executing contract has been called directly by a user.
    ///
    /// Address starts with `0x00`.
    Account = 0x00,
    /// Specifies that the [`Address`] identifies a system contract.
    ///
    /// [`SystemContract`](Self::SystemContract)s are special contracts with priviledged access to the blockchain state,
    /// most notably modification access to account plugin, which handles BYOC and MPC tokens.
    ///
    /// [`SystemContract`](Self::SystemContract)s can only be created by consensus of block producers, due to the priviledge
    /// access these contracts possess.
    ///
    /// Address starts with `0x01`.
    SystemContract = 0x01,
    /// Specifies that the [`Address`] identifies a standard smart contract.
    ///
    /// These addresses are used for smart contracts that does not use zero knowledge functionality,
    /// including for example token contracts.
    ///
    /// When [`sender`](crate::context::ContractContext::sender) contains this kind of address, it indicates that the
    /// executing contract has been called indirectly through another contract.
    ///
    /// Address starts with `0x02`.
    PublicContract = 0x02,
    /// Specifies that the [`Address`] identifies a zero knowledge smart contract.
    ///
    /// These addresses are used by contracts that use zero knowledge functionality.
    ///
    /// When [`sender`](crate::context::ContractContext::sender) contains this kind of address, it indicates that the
    /// executing contract has been called indirectly through another contract.
    ///
    /// Address starts with `0x03`.
    ZkContract = 0x03,
    /// Specifies that the [`Address`] identifies a governance system contract.
    ///
    /// These are a special variant of [`SystemContract`](Self::SystemContract), and all documentation for [`SystemContract`](Self::SystemContract)s apply
    /// here also.
    ///
    /// [`GoveranceContract`](Self::GoveranceContract)s are much cheaper to call, due to being part of the Partisia
    /// BlockChain backbone and bootstrap process.
    ///
    /// Address starts with `0x04`.
    GoveranceContract = 0x04,
}

impl AddressType {
    fn discriminant(&self) -> u8 {
        match self {
            Self::Account => 0x00,
            Self::SystemContract => 0x01,
            Self::PublicContract => 0x02,
            Self::ZkContract => 0x03,
            Self::GoveranceContract => 0x04,
        }
    }
}

/// A unique number that identifies accounts and contracts on Partisia BlockChain.
///
/// [`Address`]es are used to for most blockchain interactions, including:
///
/// - Specifying a contract to interact with.
/// - Specifying the contract running an interaction.
/// - Specifying an account to deposit/withdraw BYOC to/from.
/// - Identifying the node operators.
/// - Identifying zero-knowledge nodes.
///
/// The address is composed of an [`AddressType`] indicating whether the address identifies
/// a contract or an end user, and a [`Identifier`](Address::identifier), which is the main
/// identifer.
///
/// The [`Identifier`](Address::identifier) deriviation method varies by the [`AddressType`]:
///
/// - [`Account`](AddressType::Account): Derived from the user's public key.
/// - [`PublicContract`](AddressType::PublicContract), [`ZkContract`](AddressType::ZkContract): Derived from the hash of the signed transaction that triggered the deployment of the contract.
/// - [`SystemContract`](AddressType::SystemContract), [`GoveranceContract`](AddressType::GoveranceContract): Hardcoded.
///
/// ## Serialization
///
/// Serializable with both RPC format and State format, guaranteed identical representation.
#[repr(C)]
#[derive(
    Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, ReadRPC, WriteRPC, ReadWriteState, Hash,
)]
pub struct Address {
    /// The type of the [`Address`], indicating whether the address identifies an account or
    /// a contract.
    pub address_type: AddressType,
    /// The embedded identifier of the [`Address`].
    pub identifier: Identifier,
}

impl fmt::UpperHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}", self.address_type.discriminant())?;
        for b in self.identifier {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

impl fmt::LowerHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.address_type.discriminant())?;
        for b in self.identifier {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self, f)
    }
}

#[cfg(test)]
use pbc_traits::{ReadRPC, ReadWriteState};

#[cfg(test)]
const ADDRESS_TYPES: [AddressType; 5] = [
    AddressType::Account,
    AddressType::SystemContract,
    AddressType::PublicContract,
    AddressType::ZkContract,
    AddressType::GoveranceContract,
];

#[test]
pub fn discriminant_test_rpc() {
    for address_type in ADDRESS_TYPES {
        let rpc = [address_type.discriminant(); 1];
        let deserialized: AddressType = ReadRPC::rpc_read_from(&mut rpc.as_slice());
        assert_eq!(deserialized, address_type);
    }
}

#[test]
pub fn discriminant_test_state() {
    for address_type in ADDRESS_TYPES {
        let rpc = [address_type.discriminant(); 1];
        let deserialized: AddressType = ReadWriteState::state_read_from(&mut rpc.as_slice());
        assert_eq!(deserialized, address_type);
    }
}
