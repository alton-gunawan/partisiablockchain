//! Contains implementation of [`Sbi`].

use crate::{Sbi1, SecretBinary, SecretBinaryFixedSize};
use pbc_traits::{ReadInt, WriteInt};
use std::cmp::Ordering;
use std::io::{Read, Write};
use std::num::Wrapping;
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Shl, Shr, Sub};

/// A secret-shared integer value.
///
/// ### Fields:
/// * `NT`: Public type of secret.
/// * `secret`: `NT`, the value of the secret
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Sbi<NT> {
    secret: NT,
}

impl<NT> From<NT> for Sbi<NT> {
    fn from(secret: NT) -> Self {
        Sbi { secret }
    }
}

impl<NT> Mul for Sbi<NT>
where
    NT: Mul<Output = NT>,
{
    type Output = Sbi<NT>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret * rhs.secret,
        }
    }
}

impl<NT> Add for Sbi<NT>
where
    Wrapping<NT>: Add<Output = Wrapping<NT>>,
{
    type Output = Sbi<NT>;

    fn add(self, rhs: Self) -> Self::Output {
        let wrapped = Wrapping(self.secret) + Wrapping(rhs.secret);
        Self { secret: wrapped.0 }
    }
}

impl<NT> Sub for Sbi<NT>
where
    Wrapping<NT>: Sub<Output = Wrapping<NT>>,
{
    type Output = Sbi<NT>;

    fn sub(self, rhs: Self) -> Self::Output {
        let wrapped = Wrapping(self.secret) - Wrapping(rhs.secret);
        Self { secret: wrapped.0 }
    }
}

impl<NT, Rhs> Shl<Rhs> for Sbi<NT>
where
    Wrapping<NT>: Shl<Rhs, Output = Wrapping<NT>>,
{
    type Output = Sbi<NT>;

    fn shl(self, rhs: Rhs) -> Self::Output {
        let wrapped = Wrapping(self.secret) << rhs;
        Self { secret: wrapped.0 }
    }
}

impl<NT, Rhs> Shr<Rhs> for Sbi<NT>
where
    Wrapping<NT>: Shr<Rhs, Output = Wrapping<NT>>,
{
    type Output = Sbi<NT>;

    fn shr(self, rhs: Rhs) -> Self::Output {
        let wrapped = Wrapping(self.secret) >> rhs;
        Self { secret: wrapped.0 }
    }
}

impl<NT> BitAnd for Sbi<NT>
where
    NT: BitAnd<Output = NT>,
{
    type Output = Sbi<NT>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret & rhs.secret,
        }
    }
}

impl<NT> BitXor for Sbi<NT>
where
    NT: BitXor<Output = NT>,
{
    type Output = Sbi<NT>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret ^ rhs.secret,
        }
    }
}

impl<NT> BitOr for Sbi<NT>
where
    NT: BitOr<Output = NT>,
{
    type Output = Sbi<NT>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret | rhs.secret,
        }
    }
}

impl<NT> PartialOrd for Sbi<NT>
where
    NT: PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.secret.partial_cmp(&rhs.secret)
    }

    fn lt(&self, rhs: &Self) -> Sbi1 {
        self.secret < rhs.secret
    }

    fn le(&self, rhs: &Self) -> Sbi1 {
        self.secret <= rhs.secret
    }

    fn gt(&self, rhs: &Self) -> Sbi1 {
        self.secret > rhs.secret
    }

    fn ge(&self, rhs: &Self) -> Sbi1 {
        self.secret >= rhs.secret
    }
}

/// Exposes interface for splitting elements into an array of its composite bits.
pub trait FromToBits {
    /// Bit-array type.
    type BitsType;

    /// Converts [`Self`] to the bits array type.
    fn to_le_bits(self) -> Self::BitsType;

    /// Converts from an array of bits to [`Self`].
    fn from_le_bits(bits: Self::BitsType) -> Self;
}

/// Expands byte arrays to bit arrays.
fn bytes_to_bits<const NBITS: usize, const NBYTES: usize>(bytes: [u8; NBYTES]) -> [Sbi1; NBITS] {
    let mut bits = [false; NBITS];
    for i in 0..NBITS {
        bits[i] = 0x1 == (0x1 & (bytes[i >> 3] >> (i & 0x7)));
    }
    bits
}

/// Implodes bit arrays to byte arrays.
fn bits_to_bytes<const NBITS: usize, const NBYTES: usize>(bits: [Sbi1; NBITS]) -> [u8; NBYTES] {
    let mut bytes = [0; NBYTES];
    for i in 0..NBYTES {
        let mut byte = 0;
        for j in 0..8 {
            byte ^= (bits[(i << 3) ^ j] as u8) << j
        }
        bytes[i] = byte;
    }
    bytes
}

macro_rules! impl_from_to_bits {
    ($($inner_type:ty)*) => {
        $(
            #[doc = "Allows conversions from [`Sbi<"]
            #[doc = stringify!($inner_type)]
            #[doc = ">`] to array of bits and back."]
            impl FromToBits for Sbi<$inner_type> {
                type BitsType = [Sbi1; <$inner_type>::BITS as usize];
                fn to_le_bits(self) -> Self::BitsType {
                    bytes_to_bits(<$inner_type>::to_le_bytes(self.secret))
                }
                fn from_le_bits(bits: Self::BitsType) -> Self {
                    Self::from(<$inner_type>::from_le_bytes(bits_to_bytes(bits)))
                }
            }
        )*
    }
}

impl_from_to_bits!(
    i8
    i16
    i32
    i64
    i128
);

#[doc = "Implementation of [`SecretBinaryFixedSize`] trait for [`Sbi1`]."]
impl SecretBinaryFixedSize for Sbi1 {
    const BITS: u32 = 1;
}

#[doc = "Implementation of [`SecretBinary`] trait for [`Sbi1`]. Uses a full byte to present a single bit."]
impl SecretBinary for Sbi1 {
    fn secret_read_from<T: Read>(reader: &mut T) -> Self {
        reader.read_u8() != 0
    }

    fn secret_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u8(u8::from(*self))
    }
}

macro_rules! read_write_secret {
    ($($inner_type:ty, $read_method:ident, $write_method:ident)*) => {
        $(
            #[doc = "Implementation of [`SecretBinaryFixedSize`] trait for [`Sbi<"]
            #[doc = stringify!($inner_type)]
            #[doc = ">`]."]
            impl SecretBinaryFixedSize for Sbi<$inner_type> {
                const BITS: u32 = <$inner_type>::BITS;
            }

            #[doc = "Implementation of [`SecretBinary`] trait for [`Sbi<"]
            #[doc = stringify!($inner_type)]
            #[doc = ">`]. Encoded as a little-endian integer."]
            impl SecretBinary for Sbi<$inner_type> {
                fn secret_read_from<T: Read>(reader: &mut T) -> Self {
                    Self::from(reader.$read_method())
                }
                fn secret_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
                    writer.$write_method(self.secret)
                }
            }
        )*
    }
}

read_write_secret!(i8, read_i8, write_i8);
read_write_secret!(i16, read_i16_le, write_i16_le);
read_write_secret!(i32, read_i32_le, write_i32_le);
read_write_secret!(i64, read_i64_le, write_i64_le);
read_write_secret!(i128, read_i128_le, write_i128_le);
