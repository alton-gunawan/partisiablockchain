//! Definitions for contract invocation [`Shortname`]s and similar identifiers.

use pbc_traits::ReadRPC;
use pbc_traits::WriteRPC;

use super::leb128;

/// Unique [LEB128](https://en.wikipedia.org/wiki/LEB128) identifier for a contract's invocations.
///
/// Automatically created for each of the contract's own invocations. Shortnames for invocations in other contracts must be created when
/// calling those contracts.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Shortname {
    /// Raw ID value
    value: u32,
}

impl Shortname {
    /// Create [`Shortname`] from an [`u32`].
    pub const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    /// Create [`Shortname`] from a slice of bytes. Slice must be valid [LEB128](https://en.wikipedia.org/wiki/LEB128)-encoded.
    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, String> {
        // Errors for last byte
        match bytes.last() {
            None => {
                return Err("Shortname must not be empty".to_string());
            }
            Some(&b) if b >= 0x80 => {
                return Err("Shortname's last byte must not have continuation bit set".to_string());
            }
            Some(&b) if b == 0x00 && bytes.len() > 1 => {
                return Err("Shortname must be normalized, with no trailing zeroes".to_string());
            }
            _ => {} // Good
        }

        // Global validation
        let all_non_last_bytes_possess_continuation_bit =
            bytes.iter().rev().skip(1).all(|&b| b >= 0x80);
        if !all_non_last_bytes_possess_continuation_bit {
            return Err(
                "Shortname's non-last bytes must have their continuation bits set".to_string(),
            );
        }

        let value_bytes: Vec<_> = bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| actual_checked_shl(b as u32 & 0x7F, i as u32 * 7))
            .collect();

        if value_bytes.iter().any(|x| x.is_none()) {
            return Err("Shortname value too large for u32".to_string());
        }

        Ok(Self {
            value: value_bytes.iter().map(|x| x.unwrap()).sum(),
        })
    }

    /// Gets the [`Shortname`] as its [`u32`] representation.
    ///
    /// Note invariant:
    ///
    /// ```
    /// # use pbc_contract_core::shortname::Shortname;
    /// # let i = 1231;
    /// assert_eq!(i, Shortname::from_u32(i).as_u32());
    /// ```
    pub const fn as_u32(&self) -> u32 {
        self.value
    }

    /// Gets the [`Shortname`] as its bytes representation.
    ///
    /// Invariants:
    /// - At least one byte long.
    /// - Last byte is less than 0x80.
    /// - Preceding bytes are larger than 0x80.
    pub fn bytes(&self) -> Vec<u8> {
        leb128::to_leb128_bytes(self.value)
    }
}

/// Special [`Shortname`] variant for `#[callback]` invocations.
#[non_exhaustive]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct ShortnameCallback {
    /// Internal [`Shortname`].
    pub shortname: Shortname,
}

impl ShortnameCallback {
    /// Create [`ShortnameCallback`] from an [`u32`].
    pub const fn from_u32(value: u32) -> Self {
        Self {
            shortname: Shortname::from_u32(value),
        }
    }

    /// Create new [`ShortnameCallback`] from [`Shortname`]
    pub fn new(shortname: Shortname) -> Self {
        ShortnameCallback { shortname }
    }
}

/// Special [`Shortname`] variant for `#[zk_compute]` Zero-knowledge computation invocations.
#[non_exhaustive]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct ShortnameZkComputation {
    /// Internal shortname
    pub shortname: Shortname,
}

impl ShortnameZkComputation {
    /// Create [`ShortnameZkComputation`] from an [`u32`].
    pub const fn from_u32(value: u32) -> Self {
        Self {
            shortname: Shortname::from_u32(value),
        }
    }

    /// Create [`ShortnameZkComputation`] from an [`Shortname`].
    pub fn new(shortname: Shortname) -> Self {
        ShortnameZkComputation { shortname }
    }
}

fn actual_checked_shl(lhs: u32, rhs: u32) -> Option<u32> {
    lhs.checked_shl(rhs).filter(|result| result >> rhs == lhs)
}

impl std::fmt::Display for Shortname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes() {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl ReadRPC for Shortname {
    fn rpc_read_from<R: std::io::Read>(_reader: &mut R) -> Self {
        unimplemented!();
    }
}

impl WriteRPC for Shortname {
    fn rpc_write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for item in &self.bytes() {
            item.rpc_write_to(writer)?;
        }

        Ok(())
    }
}
