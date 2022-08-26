//! Serialization for Partisia Blockchain SDK
//!
//! Exposes [the three serialization formats](https://privacyblockchain.gitlab.io/language/rust-contract-sdk/abiv1.html) used in contracts:
//!
//! - [`ReadWriteState`] for State serialization.
//! - [`ReadWriteRPC`] for RPC serialization.
//! - [`create_type_spec::CreateTypeSpec`] for ABI serialization.

#[cfg(feature = "abi")]
#[macro_use]
extern crate quote;

#[cfg(not(feature = "abi"))]
extern crate quote;

#[cfg(feature = "abi")]
pub use create_type_spec::CreateTypeSpec;
pub use read_int::ReadInt;
pub use readwrite_rpc::ReadWriteRPC;
pub use readwrite_state::ReadWriteState;
pub use write_int::WriteInt;

#[cfg(feature = "abi")]
mod create_type_spec;

mod read_int;
mod readwrite_rpc;
mod readwrite_state;
mod write_int;
