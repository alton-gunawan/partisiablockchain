//! Module containing definition of readwrite state and implementations for several different data
//! structures.
use std::io::{Read, Write};

//// Sub-modules
mod impl_misc;
mod impl_vec;

//// Re-exported trait implementations

pub use impl_misc::*;
pub use impl_vec::*;

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
