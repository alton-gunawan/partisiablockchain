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
pub mod context;
pub mod events;
pub mod signature;

#[cfg(any(feature = "zk", doc))]
pub mod zk;

//// Reexports

#[cfg(feature = "abi")]
pub use raw_ptr::RawPtr;

#[cfg(feature = "abi")]
pub use pbc_contract_core::abi;
pub use pbc_contract_core::{function_name, shortname};

/// Address module.
pub mod address {
    pub use super::address_internal::{Address, AddressType};
    pub use super::shortname::{Shortname, ShortnameCallback};
}

#[cfg(any(test, doc, feature = "test_examples"))]
pub mod test_examples;

pub use result_buffer::ContractResultBuffer;

//// Some actual functionality

/// The hash type is simply a 32 byte array
pub type Hash = [u8; 32];

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
