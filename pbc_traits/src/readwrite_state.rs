use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};

use crate::read_int::ReadInt;
use crate::write_int::WriteInt;

/// Marks implementations of the [State serialization format](https://privacyblockchain.gitlab.io/language/rust-contract-sdk/abiv1.html).
///
/// # Serialization invariants
///
/// For any given value `v` in a type `T` with `impl ReadWriteState for T`, the expected invariants
/// are:
///
/// - The serialization `b` of `v_1` should be deserializable to a `v_2` identical to `v_1`
/// - If a buffer `b_1` is deserializable to `v`, then the serialization `b_2` of `v` should
///   equal to `b_1`.
///
/// The default implementations of [`ReadWriteState`] uphold these invariants, but any custom
/// implementation may choose to forgo these invariants at their own expense, if they deem the
/// confusion worth it.
pub trait ReadWriteState: Sized {
    /// Indicates whether the value's byte representation is identical in memory and when in
    /// serialized form.
    ///
    /// When set to `true`, some usages may choose to implement the serialization by `memcpy`ing
    /// instead of calling recursively, hence the requirement for identical representation.
    ///
    /// # Safety and invariants
    ///
    /// For any given value `v` in a type `T` with `impl ReadWriteState for T`, and
    /// `T::SERIALIZABLE_BY_COPY == true`, the expected invariants are:
    ///
    /// - The serialization `b` of `v` should be identical to `v`'s memory representation
    ///   `bytes(v)`.
    /// - If a buffer `b` is deserializable to `v`, then `v`'s memory representation
    ///   `bytes(v)` must be identical to `b`.
    ///
    /// It is unsafe to set `SERIALIZABLE_BY_COPY = true` when above invariants doesn't hold, as it
    /// may violate Rust's type safety. If in doubt, set `SERIALIZABLE_BY_COPY = false`.
    const SERIALIZABLE_BY_COPY: bool;

    /// Deserialization method for state.
    fn state_read_from<T: Read>(reader: &mut T) -> Self;

    /// Serialization method for state.
    fn state_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;
}

/// Implementation of the [`ReadWriteState`] trait for [`Vec<T>`] for any `T` that implements [`ReadWriteState`].
impl<T: ReadWriteState> ReadWriteState for Vec<T> {
    /// Not supported, due to internal pointers.
    const SERIALIZABLE_BY_COPY: bool = false;

    fn state_read_from<R: Read>(reader: &mut R) -> Self {
        match T::SERIALIZABLE_BY_COPY {
            true => serialize_vec::static_sized_content_read_from(reader),
            false => serialize_vec::dynamic_sized_content_read_from(reader),
        }
    }

    fn state_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match T::SERIALIZABLE_BY_COPY {
            true => serialize_vec::static_sized_content_write_to(self, writer),
            false => serialize_vec::dynamic_sized_content_write_to(self, writer),
        }
    }
}

/// Contains [`ReadWriteState`] methods for the [`std::vec::Vec`] impl.
mod serialize_vec {

    use std::io::{Read, Write};
    use std::mem::MaybeUninit;

    use super::ReadWriteState;
    use crate::read_int::ReadInt;
    use crate::write_int::WriteInt;

    pub fn dynamic_sized_content_read_from<R: Read, T: ReadWriteState>(reader: &mut R) -> Vec<T> {
        let len = reader.read_u32_le() as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(T::state_read_from(reader))
        }
        result
    }

    pub fn dynamic_sized_content_write_to<W: Write, T: ReadWriteState>(
        vec: &[T],
        writer: &mut W,
    ) -> std::io::Result<()> {
        writer.write_u32_le(vec.len() as u32).unwrap();
        for item in vec {
            item.state_write_to(writer).unwrap();
        }

        Ok(())
    }

    pub fn static_sized_content_read_from<R: Read, T: ReadWriteState>(reader: &mut R) -> Vec<T> {
        assert!(T::SERIALIZABLE_BY_COPY);

        let count = reader.read_u32_le() as usize;
        let mut result: Vec<MaybeUninit<T>> = Vec::with_capacity(count);
        unsafe {
            result.set_len(count);
            if std::mem::size_of::<T>() > 0 {
                let (prefix, middle, suffix) = result.align_to_mut::<u8>();
                assert!(prefix.is_empty());
                assert!(suffix.is_empty());
                reader.read_exact(middle).unwrap();
            }

            std::mem::transmute::<_, Vec<T>>(result)
        }
    }

    pub fn static_sized_content_write_to<W: Write, T: ReadWriteState>(
        vec: &[T],
        writer: &mut W,
    ) -> std::io::Result<()> {
        assert!(T::SERIALIZABLE_BY_COPY);

        writer.write_u32_le(vec.len() as u32)?;
        if std::mem::size_of::<T>() > 0 {
            unsafe {
                let (prefix, middle, suffix) = vec.align_to::<u8>();
                assert!(prefix.is_empty());
                assert!(suffix.is_empty());
                writer.write_all(&*middle)?;
            }
        }
        Ok(())
    }
}

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

/// Implementation of the [`ReadWriteState`] trait for [`BTreeMap<K, V>`].
/// for any `K`, `V` that implement [`ReadWriteState`]
impl<K: ReadWriteState + Ord, V: ReadWriteState> ReadWriteState for BTreeMap<K, V> {
    /// Not supported, due to internal pointers.
    const SERIALIZABLE_BY_COPY: bool = false;

    fn state_read_from<R: Read>(reader: &mut R) -> Self {
        let mut result = BTreeMap::new();
        let len = reader.read_u32_le();

        for _ in 0..len {
            let key = K::state_read_from(reader);
            let value = V::state_read_from(reader);
            if result.insert(key, value).is_some() {
                panic!("Duplicate key added");
            }
        }

        result
    }

    fn state_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u32_le(self.len() as u32)?;
        for (key, value) in self.iter() {
            key.state_write_to(writer)?;
            value.state_write_to(writer)?;
        }
        Ok(())
    }
}

/// Implementation of the [`ReadWriteState`] trait for [`BTreeSet<T>`].
/// for any T that implements [`ReadWriteState`]
impl<T: ReadWriteState + Ord> ReadWriteState for BTreeSet<T> {
    /// Not supported, due to internal pointers.
    const SERIALIZABLE_BY_COPY: bool = false;

    fn state_read_from<R: Read>(reader: &mut R) -> Self {
        let mut result = BTreeSet::new();
        let mut previous = None;

        let len = reader.read_u32_le();
        for _ in 0..len {
            let value = T::state_read_from(reader);
            if let Some(prev_value) = previous {
                if value <= prev_value {
                    panic!("Unordered or duplicate key added");
                }
                result.insert(prev_value);
                previous = Some(value)
            } else {
                previous = Some(value);
            }
        }

        if let Some(prev_value) = previous {
            result.insert(prev_value);
        }

        result
    }

    fn state_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u32_le(self.len() as u32)?;
        for value in self.iter() {
            value.state_write_to(writer)?;
        }
        Ok(())
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
impl<const LEN: usize> ReadWriteState for [u8; LEN] {
    const SERIALIZABLE_BY_COPY: bool = <u8 as ReadWriteState>::SERIALIZABLE_BY_COPY;

    fn state_read_from<T: Read>(reader: &mut T) -> Self {
        let mut buf: [u8; LEN] = [0; LEN];
        reader.read_exact(&mut buf).unwrap();
        buf
    }

    fn state_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_all(self)
    }
}
