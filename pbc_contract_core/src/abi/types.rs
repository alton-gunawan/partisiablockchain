use std::io::Write;

use crate::abi::enum_variant::EnumVariant;
use pbc_traits::{WriteInt, WriteRPC};

use super::{abi_serialize_slice, AbiSerialize, NamedEntityAbi};

/// A struct representing the ABI for a Rust type.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Debug, Eq)]
pub struct NamedTypeSpec {
    /// The name of the type.
    pub name: String,
    /// The type key, a unique key for identifying the represented Rust type in the ABI.
    pub type_identifier: String,
    /// The list of bytes comprising the type spec for the type represented by the ABI.
    pub type_spec: Vec<u8>,
    /// The specific kind information, either struct or enum.
    pub kind_information: KindInfo,
}

/// An enum holding the specific kind information for the different named type specifications.
#[derive(PartialEq, Debug, Eq)]
pub enum KindInfo {
    /// The list of the fields that are associated with the struct.
    Struct {
        /// The fields of the struct.
        fields: Vec<NamedEntityAbi>,
    },
    /// The list of variants that are associated with the enum.
    Enum {
        /// The variants of the enum.
        variants: Vec<EnumVariant>,
    },
}

impl NamedTypeSpec {
    /// Construct a new `StructTypeSpec` instance with the specified name
    pub fn new_struct(name: String, type_identifier: String, type_spec: Vec<u8>) -> Self {
        NamedTypeSpec {
            name,
            type_identifier,
            type_spec,
            kind_information: KindInfo::Struct { fields: Vec::new() },
        }
    }

    /// Construct a new `EnumTypeSpec` instance with the specified name
    pub fn new_enum(name: String, type_identifier: String, type_spec: Vec<u8>) -> Self {
        NamedTypeSpec {
            name,
            type_identifier,
            type_spec,
            kind_information: KindInfo::Enum {
                variants: Vec::new(),
            },
        }
    }

    /// Add a field to this `TypeAbi` instance.
    pub fn add_field(&mut self, field: NamedEntityAbi) {
        if let KindInfo::Struct { ref mut fields } = self.kind_information {
            fields.push(field);
        }
    }

    /// Add a variant to this `TypeAbi` instance.
    pub fn add_variant(&mut self, variant: EnumVariant) {
        if let KindInfo::Enum { ref mut variants } = self.kind_information {
            variants.push(variant);
        }
    }
}

impl AbiSerialize for NamedTypeSpec {
    fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        match &self.kind_information {
            KindInfo::Struct { fields } => {
                writer.write_u8(1)?;
                self.name.rpc_write_to(writer)?;
                abi_serialize_slice(fields, writer)
            }
            KindInfo::Enum { variants } => {
                writer.write_u8(2)?;
                self.name.rpc_write_to(writer)?;
                abi_serialize_slice(variants, writer)
            }
        }
    }
}
