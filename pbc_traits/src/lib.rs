//! Serialization for Partisia Blockchain SDK
//!
//! Exposes [the three serialization formats](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html) used in contracts:
//!
//! - [`ReadWriteState`] for State serialization.
//! - [`ReadRPC`] for RPC serialization.
//! - [`WriteRPC`] for RPC serialization.
//! - [`create_type_spec::CreateTypeSpec`] for ABI serialization.

#[cfg(feature = "abi")]
pub use create_type_spec::CreateTypeSpec;
pub use read_int::ReadInt;
pub use readwrite_rpc::ReadRPC;
pub use readwrite_rpc::WriteRPC;
pub use readwrite_state::ReadWriteState;
pub use write_int::WriteInt;

#[cfg(feature = "abi")]
mod create_type_spec;

mod read_int;
mod readwrite_rpc;
mod readwrite_state;
mod write_int;
