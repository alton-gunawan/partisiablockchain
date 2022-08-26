use std::collections::BTreeMap;
use std::io::Write;

use pbc_traits::{CreateTypeSpec, ReadWriteRPC};

use crate::abi::{abi_serialize_slice, type_spec_from_type, AbiSerialize, NamedEntityAbi};

/// A struct representing the ABI for a Rust type.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Debug, Eq)]
pub struct TypeAbi {
    /// The name of the type.
    pub name: String,
    /// The type key, a unique key for identifying the represented Rust type in the ABI.
    pub type_identifier: String,
    /// The list of the fields that are associated with this type.
    pub fields: Vec<NamedEntityAbi>,

    /// The list of bytes comprising the type spec for the type represented by the ABI.
    pub type_spec: Vec<u8>,
}

impl TypeAbi {
    /// Construct a new `TypeAbi` instance with the specified name.
    pub fn new<T: CreateTypeSpec>(name: String, lut: &BTreeMap<String, u8>) -> Self {
        TypeAbi {
            name,
            type_identifier: T::__ty_identifier(),
            fields: Vec::new(),
            type_spec: type_spec_from_type::<T>(lut),
        }
    }

    /// Add a field to this `TypeAbi` instance.
    pub fn field(&mut self, field: NamedEntityAbi) {
        self.fields.push(field);
    }
}

impl AbiSerialize for TypeAbi {
    fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.name.rpc_write_to(writer)?;
        abi_serialize_slice(&self.fields, writer)
    }
}
