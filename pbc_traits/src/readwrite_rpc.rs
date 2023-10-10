use std::io::{Read, Write};

use crate::read_int::ReadInt;
use crate::write_int::WriteInt;

/// Marks implementations that read [RPC serialization format](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html).
///
/// # Serialization invariants and safety
///
/// For any given value `v` in a type `T` with `impl ReadRPC for T`, the expected invariants
/// are:
///
/// - The serialization `b` of `v_1` must be deserializable to a `v_2` identical to `v_1`
/// - If a buffer `b_1` is deserializable to `v`, then the serialization `b_2` of `v` must
///   equal to `b_1`.
///
/// The default implementations of [`ReadRPC`] uphold these invariants; custom implementations
/// must make sure they satisfy the above invariants, or risk miscommunication with the blockchain
/// and other contracts.
pub trait ReadRPC: Sized {
    /// Deserialization method for RPC arguments.
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self;
}

/// Marks implementations that the [RPC serialization format](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html).
///
/// # Serialization invariants and safety
///
/// For any given value `v` in a type `T` with `impl WriteRPC for T`, the expected invariants
/// are:
///
/// - The serialization `b` of `v_1` must be deserializable to a `v_2` identical to `v_1`
/// - If a buffer `b_1` is deserializable to `v`, then the serialization `b_2` of `v` must
///   equal to `b_1`.
///
/// The default implementations of [`WriteRPC`] uphold these invariants; custom implementations
/// must make sure they satisfy the above invariants, or risk miscommunication with the blockchain
/// and other contracts.
pub trait WriteRPC: Sized {
    /// Serialization method for RPC arguments.
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;
}

/// Implementation of the [`ReadRPC`] trait for a vector of any type `T`
/// that implements [`ReadRPC`]..
impl<T: ReadRPC> ReadRPC for Vec<T> {
    fn rpc_read_from<R: Read>(reader: &mut R) -> Self {
        let len = reader.read_u32_be() as usize;
        let mut result = Vec::with_capacity(usize::min(len, 128usize));
        for _ in 0..len {
            result.push(T::rpc_read_from(reader))
        }
        result
    }
}

/// Implementation of the [`WriteRPC`] trait for a vector of any type `T`
/// that implements [`WriteRPC`]..
impl<T: WriteRPC> WriteRPC for Vec<T> {
    fn rpc_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_i32_be(self.len() as i32).unwrap();
        for item in self {
            item.rpc_write_to(writer).unwrap();
        }

        Ok(())
    }
}

/// Implementation of the [`ReadRPC`] trait for [`Option<T>`] of any type that implements [`ReadRPC`].
impl<T: ReadRPC> ReadRPC for Option<T> {
    fn rpc_read_from<R: Read>(reader: &mut R) -> Self {
        let marker = reader.read_u8();
        match marker {
            0 => None,
            _ => Some(T::rpc_read_from(reader)),
        }
    }
}

/// Implementation of the [`WriteRPC`] trait for [`Option<T>`] of any type that implements [`WriteRPC`].
impl<T: WriteRPC> WriteRPC for Option<T> {
    fn rpc_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match &self {
            None => writer.write_u8(0),
            Some(value) => {
                writer.write_u8(1).unwrap();
                value.rpc_write_to(writer)
            }
        }
    }
}

/// Implementation of the [`ReadRPC`] trait for [`String`].
impl ReadRPC for String {
    /// To avoid copying the bytes we have an "asymmetrical" read write for String, where
    /// the write method writes using slices of bytes and the read method reads vectors of bytes.
    ///
    /// The reason this asymmetry works is that a [`&[u8]`] is the result of borrowing
    /// a [`Vec<u8>`].
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        // We can read this as an vector of bytes even though we wrote it as a slice,
        // since a byte slice &[u8] is simply a borrowed Vec<u8>.
        let vec: Vec<u8> = Vec::rpc_read_from(reader);
        String::from_utf8(vec).unwrap()
    }
}

/// Implementation of the [`WriteRPC`] trait for [`String`].
impl WriteRPC for String {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        let utf_bytes = self.as_bytes();
        writer.write_u32_be(utf_bytes.len() as u32).unwrap();
        writer.write_all(utf_bytes)
    }
}

/// Implementation of the [`ReadRPC`] trait for [`bool`].
impl ReadRPC for bool {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        reader.read_u8() != 0
    }
}

/// Implementation of the [`WriteRPC`] trait for [`bool`].
impl WriteRPC for bool {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u8(u8::from(*self))
    }
}

macro_rules! rw_int {
    ($($type:ty, $read_method:ident, $write_method:ident)*) => {
        $(
            #[doc = "Implementation of [`ReadRPC`] trait for [`"]
            #[doc = stringify!($type)]
            #[doc = "`]."]
            impl ReadRPC for $type {
                fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
                    reader.$read_method()
                }
            }

            #[doc = "Implementation of [`WriteRPC`] trait for [`"]
            #[doc = stringify!($type)]
            #[doc = "`]."]
            impl WriteRPC for $type {
                fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
                    writer.$write_method(*self)
                }
            }
        )*
    }
}

rw_int!(u8, read_u8, write_u8);
rw_int!(u16, read_u16_be, write_u16_be);
rw_int!(u32, read_u32_be, write_u32_be);
rw_int!(u64, read_u64_be, write_u64_be);
rw_int!(u128, read_u128_be, write_u128_be);

rw_int!(i8, read_i8, write_i8);
rw_int!(i16, read_i16_be, write_i16_be);
rw_int!(i32, read_i32_be, write_i32_be);
rw_int!(i64, read_i64_be, write_i64_be);
rw_int!(i128, read_i128_be, write_i128_be);

/// Implementation of [`ReadRPC`] for byte arrays of arbitrary sizes.
impl<const LEN: usize> ReadRPC for [u8; LEN] {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        let mut buf: [u8; LEN] = [0; LEN];
        reader.read_exact(&mut buf).unwrap();
        buf
    }
}

/// Implementation of [`WriteRPC`] for byte arrays of arbitrary sizes.
impl<const LEN: usize> WriteRPC for [u8; LEN] {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_all(self)
    }
}
