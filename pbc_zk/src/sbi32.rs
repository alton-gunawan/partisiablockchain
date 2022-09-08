use crate::sbi1::Sbi1;
use crate::Secret;
use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

/// A secret-shared i32 value
///
/// ### Fields:
/// * `secret`: [`i32`], the value of the secret
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sbi32 {
    secret: i32,
}

impl Secret for Sbi32 {}

/// Conversion from [`i32`] to [`Sbi32`].
///
/// ### Parameters:
///
/// * `val`: [`i32`], the value to be converted.
///
/// ### Returns:
///
/// The corresponding Sbi32 value.
pub fn sbi32_from(val: i32) -> Sbi32 {
    Sbi32 { secret: val }
}

impl Mul for Sbi32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret * rhs.secret,
        }
    }
}

impl Div for Sbi32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret / rhs.secret,
        }
    }
}

impl Rem for Sbi32 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Sbi32 {
        Self {
            secret: self.secret % rhs.secret,
        }
    }
}

impl Add for Sbi32 {
    type Output = Self;

    fn add(self, other: Self) -> Sbi32 {
        Self {
            secret: self.secret + other.secret,
        }
    }
}

impl Sub for Sbi32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret - rhs.secret,
        }
    }
}

impl Shl for Sbi32 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret << rhs.secret,
        }
    }
}

impl Shr for Sbi32 {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret >> rhs.secret,
        }
    }
}

impl BitAnd for Sbi32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret & rhs.secret,
        }
    }
}

impl BitXor for Sbi32 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret ^ rhs.secret,
        }
    }
}

impl BitOr for Sbi32 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            secret: self.secret | rhs.secret,
        }
    }
}

impl PartialOrd for Sbi32 {
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
