//! Partisia Blockchain SDK Common Crate
//!
//! Defines common types and methods used in PBC smart contracts.
extern crate quote;

//// Internal modules to be reexported

mod address_internal;
mod result_buffer;

#[cfg(feature = "abi")]
mod raw_ptr;

//// Directly exported modules

pub use function_name::{FunctionKind, FunctionName};
use std::io::{Read, Write};
pub mod context;
pub mod events;
pub mod signature;

pub mod sorted_vec_map;

pub mod zk;

//// Reexports

#[cfg(feature = "abi")]
pub use raw_ptr::RawPtr;

#[cfg(feature = "abi")]
pub use pbc_contract_core::abi;
pub use pbc_contract_core::{function_name, shortname};
#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use pbc_traits::{ReadRPC, WriteRPC};
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// Address module.
pub mod address {
    pub use super::address_internal::{Address, AddressType};
    pub use super::shortname::{Shortname, ShortnameCallback};
}

#[cfg(any(test, doc, feature = "test_examples"))]
pub mod test_examples;

pub use result_buffer::ContractResultBuffer;

/// Creates the code for implementing the CreateTypeSpec trait for built-in types.
///
/// The type name and identifier is simply the name of the type.
///
/// The serialization logic writes the identifier byte.
/// ### Parameters:
///
/// * `name`: [&str literal] - The name of the type.
///
/// * `identifier_byte`: byte literal - The identifying byte of the type, as specified by the abi.
///
/// ### Returns:
/// A default implementation of CreateTypeSpec for built-in types.
#[macro_export]
macro_rules! type_spec_default_impl {
    ($name: literal, $identifier_byte: literal) => {
        fn __ty_name() -> String {
            $name.to_string()
        }

        fn __ty_identifier() -> String {
            Self::__ty_name()
        }

        fn __ty_spec_write(w: &mut Vec<u8>, _lut: &std::collections::BTreeMap<String, u8>) {
            w.push($identifier_byte)
        }
    };
}

//// Some actual functionality

/// A hash is the result of a hashing process, yielding a unique identifier for the hashed artifact.
///
/// This identifier always consists of 32 bytes.
#[derive(Eq, PartialEq, Debug, Clone, PartialOrd, Ord, ReadWriteState, ReadWriteRPC)]
pub struct Hash {
    /// The bytes of the hash.
    pub bytes: [u8; 32],
}

#[cfg(feature = "abi")]
impl CreateTypeSpec for Hash {
    type_spec_default_impl!("Hash", 0x13);
}

/// A public key is used to send encrypted transactions on the blockchain.
/// Transactions must be encrypted under a public key registered on the blockchain otherwise they will fail.
///
/// The key consists of a 33 byte array.
#[derive(Eq, PartialEq, Debug, Clone, PartialOrd, Ord, ReadWriteState, ReadWriteRPC)]
pub struct PublicKey {
    /// The bytes of the public key.
    pub bytes: [u8; 33],
}

#[cfg(feature = "abi")]
impl CreateTypeSpec for PublicKey {
    type_spec_default_impl!("PublicKey", 0x14);
}

/// A BLS (Boneh-Lynn-Shacham) is a different type of [public key](PublicKey), that allows for aggregation.
///
/// It is used to authenticate aggregated [BLS signatures](BlsSignature), by aggregating the public keys that was used for signing, into a 'single' key.
///
/// A BLS public key consists of a 96 byte array.
#[derive(Eq, PartialEq, Debug, Clone, PartialOrd, Ord, ReadWriteState, ReadWriteRPC)]
pub struct BlsPublicKey {
    /// The bytes of the BLS public key.
    pub bytes: [u8; 96],
}

#[cfg(feature = "abi")]
impl CreateTypeSpec for BlsPublicKey {
    type_spec_default_impl!("BlsPublicKey", 0x16);
}

/// A BLS (Boneh-Lynn-Shacham) is a different type of [signature](signature::Signature), that allows for aggregation.
///
/// It is used to produce a joint (single) signature on e.g. a block between a group of users, rather than one signature per user as is the case for [signature](signature::Signature).
///
/// A BLS signature consists of a 48 byte array.
#[derive(Eq, PartialEq, Debug, Clone, PartialOrd, Ord, ReadWriteState, ReadWriteRPC)]
pub struct BlsSignature {
    /// The bytes of the BLS signature.
    pub bytes: [u8; 48],
}

#[cfg(feature = "abi")]
impl CreateTypeSpec for BlsSignature {
    type_spec_default_impl!("BlsSignature", 0x17);
}

/// A u256 is a 256-bit unsigned integer.
///
/// It consists of a 32 byte array.
#[derive(Eq, PartialEq, Debug, Clone, PartialOrd, Ord, ReadWriteState)]
pub struct U256 {
    /// The bytes of the u256.
    pub bytes: [u8; 32],
}

#[cfg(feature = "abi")]
impl CreateTypeSpec for U256 {
    type_spec_default_impl!("U256", 0x18);
}

impl ReadRPC for U256 {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        let mut bytes: [u8; 32] = [0; 32];
        reader.read_exact(&mut bytes).unwrap();
        bytes.reverse();
        Self { bytes }
    }
}

impl WriteRPC for U256 {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        let mut bytes: [u8; 32] = self.bytes;
        bytes.reverse();
        writer.write_all(&bytes)
    }
}

/// The shortname for the init method of a contract.
const FN_INIT_SHORTNAME: u32 = 0xFFFFFFFF;

/// The shortname for the init method of a contract.
pub fn fn_init_shortname() -> shortname::Shortname {
    shortname::Shortname::from_u32(FN_INIT_SHORTNAME)
}

/// Write an ABI object to the given pointer.
/// This method uses unsafe code.
#[cfg(feature = "abi")]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn abi_to_ptr<T: pbc_traits::ReadWriteState>(abi: T, pointer: *mut u8) -> u32 {
    let mut raw = RawPtr::new(pointer);
    abi.state_write_to(&mut raw).unwrap();
    raw.get_offset()
}

/// Log a message to the blockchain standard out.
#[deprecated(note = "Was never supported")]
pub fn info(_string: String) {
    unimplemented!("info no longer supported")
}
