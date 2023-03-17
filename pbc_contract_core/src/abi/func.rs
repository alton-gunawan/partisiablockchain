extern crate sha2;

use std::collections::BTreeMap;
use std::io::Write;

use pbc_traits::{CreateTypeSpec, WriteRPC};

use super::{abi_serialize_slice, AbiSerialize, NamedEntityAbi};
use crate::function_name::{FunctionKind, FunctionName};
use crate::shortname::Shortname;

/// A struct representing a function in the ABI.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Eq)]
pub struct FnAbi {
    name: FunctionName,
    fn_kind: FunctionKind,
    args: Vec<NamedEntityAbi>,
}

impl FnAbi {
    /// Create a function abi with the supplied name.
    pub fn new(name: String, shortname: Option<Shortname>, fn_kind: FunctionKind) -> Self {
        Self::from_name(FunctionName::new(name, shortname), fn_kind)
    }

    /// Create a function abi with the given function name
    pub fn from_name(name: FunctionName, fn_kind: FunctionKind) -> Self {
        FnAbi {
            name,
            fn_kind,
            args: Vec::new(),
        }
    }

    /// Add an argument to this instance. Types are inferred.
    ///
    /// * `name` - the name of the type.
    /// * `lut` - the lookup table for the ABI generation. See `pbc-abigen` for details.
    pub fn argument<T: CreateTypeSpec>(&mut self, name: String, lut: &BTreeMap<String, u8>) {
        self.args.push(NamedEntityAbi::new::<T>(name, lut));
    }
}

#[cfg(feature = "abi")]
impl AbiSerialize for FnAbi {
    fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.fn_kind.rpc_write_to(writer)?;
        self.name.serialize_abi(writer)?;
        abi_serialize_slice(&self.args, writer)
    }
}
