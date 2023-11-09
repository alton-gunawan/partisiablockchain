//! Provides a [`AvlTreeMap`] which allows for partial deserialization of state.
//!
//! [`AvlTreeMap`] should only be used for field variables in the smart contract state and should
//! not be initialized for intermediate computations
mod tree;

pub use tree::AvlTreeMap;
