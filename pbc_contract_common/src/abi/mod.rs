use std::collections::BTreeMap;
use std::io::Write;

pub use contract::ContractAbi;
pub use func::FnAbi;
pub use named_entity::NamedEntityAbi;
use pbc_traits::{CreateTypeSpec, WriteInt};
pub use types::TypeAbi;

mod contract;
mod func;
/// ABI generation goes through this module.
pub mod generate;
mod named_entity;
mod types;

/// Serialize this struct according to the ABI specification.
///
/// * `slice` - the slice of T's to write to the stream
/// * `writer` - the writer
///
pub fn abi_serialize_slice<T: AbiSerialize, W: Write>(
    slice: &[T],
    writer: &mut W,
) -> std::io::Result<()> {
    writer.write_u32_be(slice.len() as u32)?;
    for t in slice.iter() {
        AbiSerialize::serialize_abi(t, writer)?;
    }
    Ok(())
}

/// A trait for serializing ABI objects.
/// This is different from `ReadWriteState` and `ReadWriteRPC` since it is intended
/// for serializing across the FFI layer between contracts and `pbc-abigen`.
/// It does not serialize struct fields directly, but according to the ABI specification.
pub trait AbiSerialize {
    /// Serialize the ABI to the given writer.
    fn serialize_abi<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;
}

/// A helper function to extract a type spec vector from the given T
pub(crate) fn type_spec_from_type<T: CreateTypeSpec>(lut: &BTreeMap<String, u8>) -> Vec<u8> {
    let mut spec = Vec::new();
    T::__ty_spec_write(&mut spec, lut);
    spec
}
