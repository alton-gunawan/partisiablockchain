extern crate sha2;

use std::collections::BTreeMap;
use std::io::Write;

use pbc_traits::{CreateTypeSpec, WriteRPC};
use pbc_zk_core::{Sbi32, SecretBinary};

use super::{abi_serialize_slice, AbiSerialize, NamedEntityAbi};
use crate::function_name::{FunctionKind, FunctionName};
use crate::shortname::Shortname;

/// A struct representing a function in the ABI.
///
/// Serialized with the ABI format.
pub struct FnAbi {
    name: FunctionName,
    fn_kind: FunctionKind,
    args: Vec<NamedEntityAbi>,
    secret_arg: Option<NamedEntityAbi>,
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
            secret_arg: None,
        }
    }

    /// Add an argument to this instance. Types are inferred.
    ///
    /// * `name` - the name of the type.
    /// * `lut` - the lookup table for the ABI generation. See `pbc-abigen` for details.
    pub fn argument<T: CreateTypeSpec>(&mut self, name: String, lut: &BTreeMap<String, u8>) {
        self.args.push(NamedEntityAbi::new::<T>(name, lut));
    }

    /// Add a secret argument to this instance. Argument must implement [`SecretBinary`].
    /// Name is "secret_input"
    ///
    /// * `lut` - the lookup table for the ABI generation. See `pbc-abigen` for details.
    pub fn secret_argument<T: CreateTypeSpec + SecretBinary>(
        &mut self,
        lut: &BTreeMap<String, u8>,
    ) {
        assert_eq!(
            self.fn_kind,
            FunctionKind::ZkSecretInputWithExplicitType,
            "Only function with kind ZkSecretInputWithExplicitType can take a secret argument"
        );
        self.secret_arg = Some(NamedEntityAbi::new::<T>("secret_input".to_string(), lut));
    }

    /// Add a default secret argument of type Sbi32 to this instance.
    /// Name is "secret_input"
    ///
    /// * `lut` - the lookup table for the ABI generation. See `pbc-abigen` for details.
    pub fn default_secret_argument(&mut self, lut: &BTreeMap<String, u8>) {
        self.secret_argument::<Sbi32>(lut);
    }
}

#[cfg(feature = "abi")]
impl AbiSerialize for FnAbi {
    fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.fn_kind.rpc_write_to(writer)?;
        self.name.serialize_abi(writer)?;
        abi_serialize_slice(&self.args, writer)?;
        match self.secret_arg {
            None => Ok(()),
            Some(ref arg) => arg.serialize_abi::<T>(writer),
        }
    }
}
