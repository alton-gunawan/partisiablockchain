use std::collections::BTreeMap;
use std::io::Write;

use pbc_traits::{CreateTypeSpec, ReadWriteRPC, WriteInt};
use read_write_rpc_derive::ReadWriteRPC;

use crate::abi::{type_spec_from_type, AbiSerialize};

/// A struct representing any (name, type) entity.
/// In this case it is function arguments and struct fields.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Eq, Debug, ReadWriteRPC)]
pub struct NamedEntityAbi {
    /// The name of the field or argument.
    pub name: String,
    /// The raw type spec for the type of the argument.
    pub type_spec: Vec<u8>,
    /// The type index should one exist.
    pub type_index: Option<u8>,
}

impl NamedEntityAbi {
    /// Instantiate a `NamedEntityAbi` with  the specified name.
    ///
    /// * `name` - the name of the type.
    /// * `lut` - the lookup table for the ABI generation. See `pbc-abigen` for details.
    pub fn new<T: CreateTypeSpec>(name: String, lut: &BTreeMap<String, u8>) -> Self {
        let type_key = T::__ty_identifier();
        let type_index = lut.get(&type_key).copied();

        NamedEntityAbi {
            name,
            type_spec: type_spec_from_type::<T>(lut),
            type_index,
        }
    }
}

impl AbiSerialize for NamedEntityAbi {
    fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.name.rpc_write_to(writer)?;
        for ord in self.type_spec.iter() {
            writer.write_u8(*ord)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use pbc_traits::ReadWriteRPC;

    use crate::abi::NamedEntityAbi;
    use std::collections::BTreeMap;

    #[test]
    fn read_write_rpc_smoke_test() {
        let lut = BTreeMap::new();
        let abi = NamedEntityAbi::new::<String>("my_name".to_string(), &lut);

        let mut output: Vec<u8> = Vec::new();
        abi.rpc_write_to(&mut output).unwrap();

        assert_eq!(
            output,
            vec![0, 0, 0, 7, 109, 121, 95, 110, 97, 109, 101, 0, 0, 0, 1, 11, 0]
        )
    }
}
