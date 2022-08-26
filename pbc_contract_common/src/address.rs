use crate::to_leb128_bytes;
use pbc_traits::ReadWriteRPC;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;

/// Represents the type of a blockchain address.
///
/// Serializable with both RPC format and State format, guaranteed identical representation.
#[repr(u8)]
#[derive(Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, ReadWriteState, ReadWriteRPC)]
pub enum AddressType {
    /// Identifies a user/service account. Identifier is prefixed with `0x00`.
    Account = 0x00,
    /// Identifies a system contract. Identifier is prefixed with `0x01`.
    SystemContract = 0x01,
    /// Identifies a public contract. Identifier is prefixed with `0x02`.
    PublicContract = 0x02,
    /// Identifies a zero knowledge contract. Identifier is prefixed with `0x03`.
    ZkContract = 0x03,
}

/// Represents a blockchain address.
///
/// Serializable with both RPC format and State format, guaranteed identical representation.
#[repr(C)]
#[derive(Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, ReadWriteRPC, ReadWriteState)]
pub struct Address {
    /// The type of the blockchain address
    pub address_type: AddressType,
    /// The embedded identifier of the blockchain address
    pub identifier: Identifier,
}

/// An address identifier is a 20 byte array derived from the hash of the public key of
/// an account.
pub type Identifier = [u8; 20];

#[cfg(feature = "abi")]
impl CreateTypeSpec for Address {
    fn __ty_name() -> String {
        "Address".to_string()
    }

    fn __ty_identifier() -> String {
        Self::__ty_name()
    }

    fn __ty_spec_write(w: &mut Vec<u8>, _lut: &std::collections::BTreeMap<String, u8>) {
        w.push(0x0d)
    }
}

/// Container for a LEB128-encoded shortname.
///
/// Instances of this type is always valid LEB128-encoded.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Shortname {
    /// Value
    value: u32,
}

impl Shortname {
    /// Create Shortname from an u32
    pub const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    /// Create Shortname from a slice of bytes. Slice must be valid LEB128-encoded.
    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, String> {
        // Errors for last byte
        match bytes.last() {
            None => {
                return Result::Err("Shortname must not be empty".to_string());
            }
            Some(&b) if b >= 0x80 => {
                return Result::Err(
                    "Shortname's last byte must not have continuation bit set".to_string(),
                );
            }
            Some(&b) if b == 0x00 && bytes.len() > 1 => {
                return Result::Err(
                    "Shortname must be normalized, with no trailing zeroes".to_string(),
                );
            }
            _ => {} // Good
        }

        // Global validation
        let all_non_last_bytes_possess_continuation_bit =
            bytes.iter().rev().skip(1).all(|&b| b >= 0x80);
        if !all_non_last_bytes_possess_continuation_bit {
            return Result::Err(
                "Shortname's non-last bytes must have their continuation bits set".to_string(),
            );
        }

        let value_bytes: Vec<_> = bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| actual_checked_shl(b as u32 & 0x7F, i as u32 * 7))
            .collect();

        if value_bytes.iter().any(|x| x.is_none()) {
            return Result::Err("Shortname value too large for u32".to_string());
        }

        Result::Ok(Self {
            value: value_bytes.iter().map(|x| x.unwrap()).sum(),
        })
    }

    /// Gets the shortname as it's u32 representation.
    ///
    /// Note invariant:
    ///
    /// ```
    /// # use pbc_contract_common::address::Shortname;
    /// # let i = 1231;
    /// assert_eq!(i, Shortname::from_u32(i).as_u32());
    /// ```
    pub const fn as_u32(&self) -> u32 {
        self.value
    }

    /// Gets the shortname as it's bytes representation.
    ///
    /// Invariants:
    /// - At least one byte long.
    /// - Last byte is less than 0x80.
    /// - Preceding bytes are larger than 0x80.
    pub(crate) fn bytes(&self) -> Vec<u8> {
        to_leb128_bytes(self.value)
    }
}

/// Container for a LEB128-encoded shortname, guarenteed to be a callback.
#[non_exhaustive]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct ShortnameCallback {
    /// Internal shortname
    pub shortname: Shortname,
}

impl ShortnameCallback {
    /// Create Shortname from an u32
    pub const fn from_u32(value: u32) -> Self {
        Self {
            shortname: Shortname::from_u32(value),
        }
    }
}

fn actual_checked_shl(lhs: u32, rhs: u32) -> Option<u32> {
    lhs.checked_shl(rhs).filter(|result| result >> rhs == lhs)
}

impl std::fmt::Display for Shortname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes() {
            write!(f, "{:02x}", byte)?;
        }
        std::fmt::Result::Ok(())
    }
}

impl ReadWriteRPC for Shortname {
    fn rpc_read_from<R: std::io::Read>(_reader: &mut R) -> Self {
        unimplemented!();
    }

    fn rpc_write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for item in &self.bytes() {
            item.rpc_write_to(writer)?;
        }

        Ok(())
    }
}
