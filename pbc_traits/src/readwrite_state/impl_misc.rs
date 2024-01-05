use super::ReadWriteState;
use crate::read_int::ReadInt;
use crate::write_int::WriteInt;
use std::io::{Read, Write};

/// Implementation of the [`ReadWriteState`] trait for [`Option<T>`] for any `T` that
/// implements [`ReadWriteState`].
impl<T: ReadWriteState> ReadWriteState for Option<T> {
    /// Not supported ATM, due to unknown memory layout. Might require ABI changes.
    const SERIALIZABLE_BY_COPY: bool = false;

    fn state_read_from<R: Read>(reader: &mut R) -> Self {
        let marker = reader.read_u8();
        match marker {
            0 => None,
            _ => Some(T::state_read_from(reader)),
        }
    }

    fn state_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match &self {
            None => writer.write_u8(0),
            Some(value) => {
                writer.write_u8(1).unwrap();
                value.state_write_to(writer)
            }
        }
    }
}

impl ReadWriteState for String {
    /// Impossible due to internal pointers.
    const SERIALIZABLE_BY_COPY: bool = false;

    /// To avoid copying the bytes we have an "asymmetrical" read write for String, where
    /// the write method writes using slices of bytes and the read method reads vectors of bytes.
    ///
    /// The reason this asymmetry works is that a &\[u8] is the result of borrowing a Vec\<u8>.
    fn state_read_from<T: Read>(reader: &mut T) -> Self {
        // We can read this as an vector of bytes even though we wrote it as a slice,
        // since a byte slice &[u8] is simply a borrowed Vec<u8>.
        let vec: Vec<u8> = Vec::state_read_from(reader);
        String::from_utf8(vec).unwrap()
    }

    fn state_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        let utf_bytes = self.as_bytes();
        writer.write_u32_le(utf_bytes.len() as u32).unwrap();
        writer.write_all(utf_bytes)
    }
}

/// Implementation of the [`ReadWriteState`] trait for [`bool`].
impl ReadWriteState for bool {
    const SERIALIZABLE_BY_COPY: bool = true;
    fn state_read_from<T: Read>(reader: &mut T) -> Self {
        reader.read_u8() != 0
    }

    fn state_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u8(u8::from(*self))
    }
}

macro_rules! rw_int_copyable {
    ($($type:ty, $read_method:ident, $write_method:ident)*) => {
        $(
            #[doc = "Implementation of [`ReadWriteState`] trait for [`"]
            #[doc = stringify!($type)]
            #[doc = "`]."]
            impl ReadWriteState for $type {
                const SERIALIZABLE_BY_COPY: bool = true;
                fn state_read_from<T: Read>(reader: &mut T) -> Self {
                    reader.$read_method()
                }

                fn state_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
                    writer.$write_method(*self)
                }
            }
        )*
    }
}

rw_int_copyable!(u8, read_u8, write_u8);
rw_int_copyable!(i8, read_i8, write_i8);
rw_int_copyable!(u16, read_u16_le, write_u16_le);
rw_int_copyable!(u32, read_u32_le, write_u32_le);
rw_int_copyable!(u64, read_u64_le, write_u64_le);
rw_int_copyable!(u128, read_u128_le, write_u128_le);

rw_int_copyable!(i16, read_i16_le, write_i16_le);
rw_int_copyable!(i32, read_i32_le, write_i32_le);
rw_int_copyable!(i64, read_i64_le, write_i64_le);
rw_int_copyable!(i128, read_i128_le, write_i128_le);

/// Implementation of [`ReadWriteState`] for byte arrays of arbitrary sizes.
impl<const LEN: usize, ElementT: ReadWriteState + Sized> ReadWriteState for [ElementT; LEN] {
    const SERIALIZABLE_BY_COPY: bool = <ElementT as ReadWriteState>::SERIALIZABLE_BY_COPY;

    fn state_read_from<T: Read>(reader: &mut T) -> Self {
        let mut data: [std::mem::MaybeUninit<ElementT>; LEN] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for element_addr in &mut data[..] {
            element_addr.write(<ElementT as ReadWriteState>::state_read_from(reader));
        }
        data.map(|x| unsafe { x.assume_init() })
    }

    fn state_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        for elem in self {
            <ElementT as ReadWriteState>::state_write_to(elem, writer)?;
        }
        Ok(())
    }
}
