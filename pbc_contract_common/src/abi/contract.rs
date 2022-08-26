use std::io::Write;

use crate::abi::{abi_serialize_slice, FnAbi, TypeAbi};

/// The `ContractAbi` describes the ABI for a contract including all the actions
/// in the contract and the contract state + all user-defined structs within the state and actions.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Eq)]
pub struct ContractAbi {
    types: Vec<TypeAbi>,
    actions: Vec<FnAbi>,
    state: Vec<u8>,
}

impl ContractAbi {
    /// Construct a new `ContractAbi` with the specified init function and state type ordinal list.
    pub fn new(state: Vec<u8>) -> Self {
        let actions = Vec::new();
        let types = Vec::new();
        ContractAbi {
            actions,
            state,
            types,
        }
    }

    /// Set the actions of this `ContractAbi` instance to the supplied vector.
    pub fn actions(&mut self, actions: Vec<FnAbi>) {
        self.actions = actions;
    }

    /// Set the types of this `ContractAbi` instance to the supplied vector.
    pub fn types(&mut self, types: Vec<TypeAbi>) {
        self.types = types;
    }

    /// Serialize this struct according to the ABI specification.
    pub fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        abi_serialize_slice(&self.types, writer)?;
        abi_serialize_slice(&self.actions, writer)?;
        writer.write_all(&self.state)
    }
}
