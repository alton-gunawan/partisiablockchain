//! Contains [`Entry`] struct for [`super::SortedVecMap`].

use pbc_traits::ReadWriteState;
use read_write_state_derive::ReadWriteState;

/// Container for a key-value pair.
///
/// ### Fields:
/// * `key`: `K`, the key of the entry.
///
/// * `value`: `V`, the value of the entry.
#[repr(C)]
#[derive(PartialEq, Debug, Clone, Eq, ReadWriteState)]
pub struct Entry<K, V> {
    /// The key of the entry.
    pub(crate) key: K,
    /// The value of the entry.
    pub(crate) value: V,
}

impl<K, V> Entry<K, V> {
    pub(crate) fn into_tuple(self) -> (K, V) {
        (self.key, self.value)
    }

    pub(crate) fn tuple(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }
}
