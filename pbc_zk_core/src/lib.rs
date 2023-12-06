#![doc = include_str!("../README.md")]
mod sbi;

use std::io::{Read, Write};

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
pub use sbi::FromToBits;
#[cfg(doc)]
pub use sbi::Sbi;
#[cfg(not(doc))]
use sbi::Sbi;

/// A secret-shared [`bool`] value.
pub type Sbi1 = bool;
/// A secret-shared [`i8`] value. See [`Sbi`].
pub type Sbi8 = Sbi<i8>;
/// A secret-shared [`i16`] value. See [`Sbi`].
pub type Sbi16 = Sbi<i16>;
/// A secret-shared [`i32`] value. See [`Sbi`].
pub type Sbi32 = Sbi<i32>;
/// A secret-shared [`i64`] value. See [`Sbi`].
pub type Sbi64 = Sbi<i64>;
/// A secret-shared [`i128`] value. See [`Sbi`].
pub type Sbi128 = Sbi<i128>;

/// Required for secret-shared values.
/// Secret variables are serialized like their public counterparts using the
/// [State serialization format](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html#state-binary-format).
pub trait SecretBinary {
    /// Deserialization method for a secret.
    fn secret_read_from<T: Read>(reader: &mut T) -> Self;
    /// Serialization method for a secret.
    fn secret_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;
}
pub use crate::SecretBinary as Secret;

/// Required for secret-shared values. Used to determine the size of secret-shared inputs.
pub trait SecretBinaryFixedSize {
    /// The bitsize of the type.
    const BITS: u32;
}

impl<T: pbc_traits::ReadWriteState> SecretBinary for T {
    fn secret_read_from<ReadT: Read>(reader: &mut ReadT) -> Self {
        Self::state_read_from(reader)
    }

    fn secret_write_to<WriteT: Write>(&self, writer: &mut WriteT) -> std::io::Result<()> {
        self.state_write_to(writer)
    }
}

/// The output is n implementations of [`CreateTypeSpec`] that simply write the type as a string
/// and fill the ordinal in the [`CreateTypeSpec::__ty_ordinal`] vector output.
#[cfg(feature = "abi")]
macro_rules! impl_for_type {
    ($($type:ty, $val:literal)*) => {
        $(
            #[doc = "Implementation of the [`CreateTypeSpec`] trait for [`"]
            #[doc = stringify!($type)]
            #[doc = "`]."]
            impl CreateTypeSpec for $type {

                #[doc = concat!("Constant string [`", stringify!($type), "`].")]
                fn __ty_name() -> String {
                    format!("{}", stringify!($type).to_string())
                }

                #[doc = concat!("Ordinal is `", stringify!($val), "`, as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).")]
                fn __ty_identifier() -> String {
                    Self::__ty_name()
                }

                fn __ty_spec_write( w: &mut Vec<u8>, _lut: &std::collections::BTreeMap<String, u8>) {
                    w.push($val)
                }
            }
        )*
    }
}

// Sbi types are mapped to their public counterparts.
#[cfg(feature = "abi")]
impl_for_type!(
    Sbi8,   0x06
    Sbi16,  0x07
    Sbi32,  0x08
    Sbi64,  0x09
    Sbi128, 0x0a
);
