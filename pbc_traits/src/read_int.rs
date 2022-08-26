use std::io::Read;

/// Specifies functions for reading integers statefully, both big and little endians.
pub trait ReadInt {
    /// Read an i128 as big endian.
    fn read_u128_be(&mut self) -> u128;
    /// Read a u128 as big endian.
    fn read_i128_be(&mut self) -> i128;

    /// Read an i64 as big endian.
    fn read_u64_be(&mut self) -> u64;
    /// Read a u64 as big endian.
    fn read_i64_be(&mut self) -> i64;

    /// Read an i32 as big endian.
    fn read_i32_be(&mut self) -> i32;
    /// Read a u32 as big endian.
    fn read_u32_be(&mut self) -> u32;

    /// Read an i16 as big endian.
    fn read_i16_be(&mut self) -> i16;
    /// Read a u16 as big endian.
    fn read_u16_be(&mut self) -> u16;

    /// Read an i128 as little endian.
    fn read_u128_le(&mut self) -> u128;
    /// Read a u128 as little endian.
    fn read_i128_le(&mut self) -> i128;

    /// Read an i64 as little endian.
    fn read_u64_le(&mut self) -> u64;
    /// Read a u64 as little endian.
    fn read_i64_le(&mut self) -> i64;

    /// Read an i32 as little endian.
    fn read_i32_le(&mut self) -> i32;
    /// Read a u32 as little endian.
    fn read_u32_le(&mut self) -> u32;

    /// Read an i16 as little endian.
    fn read_i16_le(&mut self) -> i16;
    /// Read a u16 as little endian.
    fn read_u16_le(&mut self) -> u16;

    /// Read an i8
    fn read_i8(&mut self) -> i8;
    /// Read a u8
    fn read_u8(&mut self) -> u8;
}

/// Generate a read method given the <Type>::from_endian_bytes.
macro_rules! read_int {
    ($($type:ty, $len:literal, $fn_name:ident, $from_ee_bytes:ident)*) => {
        $(
            #[doc = "Implementation of ReadInt-trait for "]
            #[doc = stringify!($type)]
            fn $fn_name(&mut self) -> $type {
                let mut buf = [0u8; $len];
                self.read_exact(&mut buf).expect(concat!("Encountered end of stream while reading ", stringify!($type)));

                <$type>::$from_ee_bytes(buf)
            }
        )*
    }
}

impl<T: Read> ReadInt for T {
    read_int!(u128, 16, read_u128_be, from_be_bytes);
    read_int!(i128, 16, read_i128_be, from_be_bytes);

    read_int!(u64, 8, read_u64_be, from_be_bytes);
    read_int!(i64, 8, read_i64_be, from_be_bytes);

    read_int!(u32, 4, read_u32_be, from_be_bytes);
    read_int!(i32, 4, read_i32_be, from_be_bytes);

    read_int!(u16, 2, read_u16_be, from_be_bytes);
    read_int!(i16, 2, read_i16_be, from_be_bytes);

    read_int!(u128, 16, read_u128_le, from_le_bytes);
    read_int!(i128, 16, read_i128_le, from_le_bytes);

    read_int!(u64, 8, read_u64_le, from_le_bytes);
    read_int!(i64, 8, read_i64_le, from_le_bytes);

    read_int!(u32, 4, read_u32_le, from_le_bytes);
    read_int!(i32, 4, read_i32_le, from_le_bytes);

    read_int!(u16, 2, read_u16_le, from_le_bytes);
    read_int!(i16, 2, read_i16_le, from_le_bytes);

    read_int!(u8, 1, read_u8, from_ne_bytes);
    read_int!(i8, 1, read_i8, from_ne_bytes);
}

#[cfg(test)]
#[path = "../unit_tests/read_ints.rs"]
mod write_ints;
