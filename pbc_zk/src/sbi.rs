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
