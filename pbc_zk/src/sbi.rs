//! Contains implementation of [`Sbi`].

use crate::Sbi1;
use std::cmp::Ordering;
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
    NT: Add<Output = NT>,
{
    type Output = Sbi<NT>;

    fn add(self, other: Self) -> Sbi<NT> {
        Self {
            secret: self.secret + other.secret,
        }
    }
}

impl<NT> Sub for Sbi<NT>
where
    NT: Sub<Output = NT>,
{
    type Output = Sbi<NT>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret - rhs.secret,
        }
    }
}

impl<NT> Shl for Sbi<NT>
where
    NT: Shl<Output = NT>,
{
    type Output = Sbi<NT>;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret << rhs.secret,
        }
    }
}

impl<NT> Shr for Sbi<NT>
where
    NT: Shr<Output = NT>,
{
    type Output = Sbi<NT>;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret >> rhs.secret,
        }
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.secret.partial_cmp(&other.secret)
    }

    fn lt(&self, other: &Self) -> Sbi1 {
        self.secret < other.secret
    }

    fn le(&self, other: &Self) -> Sbi1 {
        self.secret <= other.secret
    }

    fn gt(&self, other: &Self) -> Sbi1 {
        self.secret > other.secret
    }

    fn ge(&self, other: &Self) -> Sbi1 {
        self.secret >= other.secret
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
