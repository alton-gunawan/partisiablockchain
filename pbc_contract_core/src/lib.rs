//! Crate containing contract data types, notably ABI generation.

pub mod function_name;
pub mod shortname;

#[cfg(feature = "abi")]
pub mod abi;

mod leb128;
