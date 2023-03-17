use std::io::Write;

use pbc_traits::{WriteInt, WriteRPC};
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;

use super::AbiSerialize;

/// A struct representing an enum variant.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Eq, Debug, ReadRPC, WriteRPC)]
pub struct EnumVariant {
    /// The discriminant of the variant.
    pub discriminant: u8,
    /// The raw type spec for the variant, should always be a struct.
    pub type_spec: Vec<u8>,
}

impl EnumVariant {
    /// Instantiate an `EnumVariant` with  the specified discriminant.
    ///
    /// * `discriminant` - the discriminant of the variant.
    /// * `type_spec` - the type_spec for the variant.
    pub fn new(discriminant: u8, type_spec: Vec<u8>) -> Self {
        EnumVariant {
            discriminant,
            type_spec,
        }
    }
}

impl AbiSerialize for EnumVariant {
    fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.discriminant.rpc_write_to(writer)?;
        for ord in self.type_spec.iter() {
            writer.write_u8(*ord)?;
        }

        Ok(())
    }
}
