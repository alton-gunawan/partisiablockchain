use super::ReadWriteState;
use crate::read_int::ReadInt;
use crate::write_int::WriteInt;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::mem::MaybeUninit;

/// Implementation of the [`ReadWriteState`] trait for [`Vec<T>`] for any `T` that implements [`ReadWriteState`].
impl<T: ReadWriteState> ReadWriteState for Vec<T> {
    /// The vector buffer is stored behind a pointer, so must be `false`.
    const SERIALIZABLE_BY_COPY: bool = false;

    fn state_read_from<R: Read>(reader: &mut R) -> Self {
        match T::SERIALIZABLE_BY_COPY {
            true => static_sized_content_read_from(reader),
            false => dynamic_sized_content_read_from(reader),
        }
    }

    fn state_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match T::SERIALIZABLE_BY_COPY {
            true => static_sized_content_write_to([self], writer),
            false => dynamic_sized_content_write_to([self], writer),
        }
    }
}

/// Implementation of the [`ReadWriteState`] trait for [`VecDeque<T>`] for any `T` that implements [`ReadWriteState`].
impl<T: ReadWriteState> ReadWriteState for VecDeque<T> {
    /// The vector buffer is stored behind a pointer, so must be `false`.
    const SERIALIZABLE_BY_COPY: bool = false;

    fn state_read_from<R: Read>(reader: &mut R) -> Self {
        // Implementation reads as vec before converting to vecdeque
        let as_vec = Vec::<T>::state_read_from(reader);
        VecDeque::from(as_vec)
    }

    fn state_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let (slice_front, slice_back) = self.as_slices();
        let slices = [slice_front, slice_back];
        match T::SERIALIZABLE_BY_COPY {
            true => static_sized_content_write_to(slices, writer),
            false => dynamic_sized_content_write_to(slices, writer),
        }
    }
}

const fn length_of_slices<T, const N: usize>(slices: [&[T]; N]) -> usize {
    let mut idx = 0;
    let mut summed_length = 0;
    while idx < slices.len() {
        summed_length += slices[idx].len();
        idx += 1;
    }
    summed_length
}

/// Handles deserialization for vecs with dynamic-sized contents.
fn dynamic_sized_content_read_from<R: Read, T: ReadWriteState>(reader: &mut R) -> Vec<T> {
    let len = reader.read_u32_le() as usize;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::state_read_from(reader))
    }
    result
}

/// Handles serialization for vecs with dynamic-sized contents.
fn dynamic_sized_content_write_to<W: Write, T: ReadWriteState, const N: usize>(
    slices: [&[T]; N],
    writer: &mut W,
) -> std::io::Result<()> {
    writer
        .write_u32_le(length_of_slices(slices) as u32)
        .unwrap();
    for slice in slices {
        for item in slice {
            item.state_write_to(writer).unwrap();
        }
    }

    Ok(())
}

/// Handles deserialization for vecs with static-sized contents.
fn static_sized_content_read_from<R: Read, T: ReadWriteState>(reader: &mut R) -> Vec<T> {
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

/// Handles serialization for vecs with static-sized contents.
fn static_sized_content_write_to<W: Write, T: ReadWriteState, const N: usize>(
    slices: [&[T]; N],
    writer: &mut W,
) -> std::io::Result<()> {
    assert!(T::SERIALIZABLE_BY_COPY);

    writer
        .write_u32_le(length_of_slices(slices) as u32)
        .unwrap();
    if std::mem::size_of::<T>() > 0 {
        for slice in slices {
            unsafe {
                let (prefix, middle, suffix) = slice.align_to::<u8>();
                assert!(prefix.is_empty());
                assert!(suffix.is_empty());
                writer.write_all(middle)?;
            }
        }
    }
    Ok(())
}
