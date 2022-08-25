use std::io::Write;

/// Specifies functions for writing integers statefully, both big and little endians.
pub trait WriteInt {
    /// Write an i128 as big endian.
    fn write_i128_be(&mut self, val: i128) -> std::io::Result<()>;
    /// Write a u128 as big endian.
    fn write_u128_be(&mut self, val: u128) -> std::io::Result<()>;

    /// Write an i64 as big endian.
    fn write_i64_be(&mut self, val: i64) -> std::io::Result<()>;
    /// Write a u64 as big endian.
    fn write_u64_be(&mut self, val: u64) -> std::io::Result<()>;

    /// Write an i32 as big endian.
    fn write_i32_be(&mut self, val: i32) -> std::io::Result<()>;
    /// Write a u32 as big endian.
    fn write_u32_be(&mut self, val: u32) -> std::io::Result<()>;

    /// Write an i16 as big endian.
    fn write_i16_be(&mut self, val: i16) -> std::io::Result<()>;
    /// Write a u16 as big endian.
    fn write_u16_be(&mut self, val: u16) -> std::io::Result<()>;

    /// Write an i128 as little endian.
    fn write_i128_le(&mut self, val: i128) -> std::io::Result<()>;
    /// Write a u128 as little endian.
    fn write_u128_le(&mut self, val: u128) -> std::io::Result<()>;

    /// Write an i64 as little endian.
    fn write_i64_le(&mut self, val: i64) -> std::io::Result<()>;
    /// Write a u64 as little endian.
    fn write_u64_le(&mut self, val: u64) -> std::io::Result<()>;

    /// Write an i32 as little endian.
    fn write_i32_le(&mut self, val: i32) -> std::io::Result<()>;
    /// Write a u32 as little endian.
    fn write_u32_le(&mut self, val: u32) -> std::io::Result<()>;

    /// Write an i16 as little endian.
    fn write_i16_le(&mut self, val: i16) -> std::io::Result<()>;
    /// Write a u16 as little endian.
    fn write_u16_le(&mut self, val: u16) -> std::io::Result<()>;

    /// Write an i8
    fn write_i8(&mut self, val: i8) -> std::io::Result<()>;
    /// Write a u8
    fn write_u8(&mut self, val: u8) -> std::io::Result<()>;
}

/// A macro for implementing integer writes using the <type>::to_be_bytes methods.
macro_rules! write_int {
    ($($type:ty, $fn_name:ident, $to_ee_bytes:ident)*) => {
        $(
            #[doc = "Implementation of WriteInt trait for "]
            #[doc = stringify!($type)]
            fn $fn_name(&mut self, val: $type) -> std::io::Result<()> {
                let buf = <$type>::$to_ee_bytes(val);
                self.write_all(&buf)
            }
        )*
    }
}

impl<T: Write> WriteInt for T {
    write_int!(u128, write_u128_be, to_be_bytes);
    write_int!(i128, write_i128_be, to_be_bytes);

    write_int!(u64, write_u64_be, to_be_bytes);
    write_int!(i64, write_i64_be, to_be_bytes);

    write_int!(u32, write_u32_be, to_be_bytes);
    write_int!(i32, write_i32_be, to_be_bytes);

    write_int!(u16, write_u16_be, to_be_bytes);
    write_int!(i16, write_i16_be, to_be_bytes);

    write_int!(u128, write_u128_le, to_le_bytes);
    write_int!(i128, write_i128_le, to_le_bytes);

    write_int!(u64, write_u64_le, to_le_bytes);
    write_int!(i64, write_i64_le, to_le_bytes);

    write_int!(u32, write_u32_le, to_le_bytes);
    write_int!(i32, write_i32_le, to_le_bytes);

    write_int!(u16, write_u16_le, to_le_bytes);
    write_int!(i16, write_i16_le, to_le_bytes);

    write_int!(u8, write_u8, to_ne_bytes);
    write_int!(i8, write_i8, to_ne_bytes);
}

#[cfg(test)]
#[path = "../unit_tests/write_ints.rs"]
mod write_ints;
