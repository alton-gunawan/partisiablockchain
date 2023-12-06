//! Provides a sorted [`Vec`]-based map.
//!
//! Backed by a simple [`Vec<Entry<K,V>>`] for use in PBC smart contracts. [`SortedVecMap`]
//! provides constant time serialization/deserialization if the entries is serializable by copy
//! (Constant size determinable at compile time).
//!
//! [`SortedVecMap`] provides amortized `O(n)` insert and `O(log(n))` lookup.

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use std::borrow::Borrow;
#[cfg(feature = "abi")]
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::{Index, RangeBounds};

use crate::sorted_vec_map::entry::Entry;
use pbc_traits::ReadWriteState;
use read_write_state_derive::ReadWriteState;

/// A [`Vec`]-based map.
/// [`SortedVecMap`] provides amortized `O(n)` insert and `O(log(n))` lookup.
///
/// ### Fields:
/// * `entries`: [`Vec<Entry<K,V>>`], the entries in [`SortedVecMap`].
#[derive(PartialEq, Debug, Clone, Eq, ReadWriteState)]
pub struct SortedVecMap<K, V> {
    entries: Vec<Entry<K, V>>,
}

/// Implementation of the [`CreateTypeSpec`] trait for [`SortedVecMap<K, V>`]
/// for any key and value type `K`, `V` that implement [`CreateTypeSpec`].
#[cfg(feature = "abi")]
impl<K: CreateTypeSpec, V: CreateTypeSpec> CreateTypeSpec for SortedVecMap<K, V> {
    /// Type name is `VecMap<K, V>`.
    fn __ty_name() -> String {
        format!("SortedVecMap<{}, {}>", K::__ty_name(), V::__ty_name())
    }

    fn __ty_identifier() -> String {
        format!(
            "SortedVecMap<{}, {}>",
            K::__ty_identifier(),
            V::__ty_identifier()
        )
    }

    /// Ordinal is `0x0f` followed by ordinals of `K` and `V`,
    /// as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).
    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        w.push(0x0f);
        K::__ty_spec_write(w, lut);
        V::__ty_spec_write(w, lut);
    }
}

impl<K: Ord, V> SortedVecMap<K, V> {
    fn get_index_of<Q>(&self, key: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Eq + Ord + ?Sized,
    {
        self.entries
            .binary_search_by_key(&key, |entry| entry.key.borrow())
    }

    fn get_entry<Q>(&self, key: &Q) -> Option<&Entry<K, V>>
    where
        K: Borrow<Q>,
        Q: Eq + Ord + ?Sized,
    {
        self.get_index_of(key)
            .ok()
            .map(|index| &self.entries[index])
    }

    fn get_entry_mut<Q>(&mut self, key: &Q) -> Option<&mut Entry<K, V>>
    where
        K: Borrow<Q>,
        Q: Eq + Ord + ?Sized,
    {
        self.get_index_of(key)
            .ok()
            .map(|index| &mut self.entries[index])
    }
}

impl<K, V> Default for SortedVecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> SortedVecMap<K, V> {
    /// Constructor for [`SortedVecMap`].
    pub fn new() -> Self {
        SortedVecMap {
            entries: Vec::new(),
        }
    }

    /// Clears [`SortedVecMap`], removing all elements.
    pub fn clear(&mut self) {
        self.entries.clear()
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get_entry(key).map(|entry| &entry.value)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get_entry(k).map(Entry::tuple)
    }

    /// Returns the first key-value pair in [`SortedVecMap`].
    /// The key in this pair is the minimum key in [`SortedVecMap`].
    pub fn first_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        self.entries.first().map(Entry::tuple)
    }

    /// Removes and returns the first element in [`SortedVecMap`].
    /// The key of this element is the minimum key that was in [`SortedVecMap`].
    pub fn pop_first(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        if !self.entries.is_empty() {
            Some(self.entries.remove(0).into_tuple())
        } else {
            None
        }
    }

    /// Returns the last key-value pair in [`SortedVecMap`].
    /// The key in this pair is the maximum key in [`SortedVecMap`].
    pub fn last_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        self.entries.last().map(Entry::tuple)
    }

    /// Removes and returns the last element in [`SortedVecMap`].
    /// The key of this element is the maximum key that was in [`SortedVecMap`].
    pub fn pop_last(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        self.entries.pop().map(Entry::into_tuple)
    }

    /// Returns `true` if [`SortedVecMap`] contains a value for the specified key.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get_entry_mut(key).map(|entry| &mut entry.value)
    }

    /// Inserts a key-value pair into [`SortedVecMap`].
    ///
    /// If [`SortedVecMap`] did not have this key present, None is returned.
    /// If [`SortedVecMap`] did have this key present, the value is updated, and the old value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        match self.get_index_of(&key) {
            Ok(idx) => {
                self.entries.push(Entry { key, value });
                Some(self.entries.swap_remove(idx).value)
            }
            Err(idx) => {
                self.entries.insert(idx, Entry { key, value });
                None
            }
        }
    }

    /// Removes a key from [`SortedVecMap`], returning the value at the key if the key was previously in [`SortedVecMap`].
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|entry| entry.1)
    }

    /// Removes a key from [`SortedVecMap`], returning the stored key and value if the key was previously in [`SortedVecMap`].
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        match self.get_index_of(key) {
            Ok(idx) => {
                let entry = self.entries.remove(idx);
                Some(entry.into_tuple())
            }
            Err(_) => None,
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` for which `f(&k, &mut v)` returns `false`.
    pub fn retain<F>(&mut self, mut f: F)
    where
        K: Ord,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.entries
            .retain_mut(|entry| f(&entry.key, &mut entry.value));
    }

    /// Moves all elements from other into self, leaving other empty.
    ///
    /// If a key from other is already present in self, the respective value from self will be
    /// overwritten with the respective value from other.
    pub fn append(&mut self, other: &mut Self)
    where
        K: Ord,
    {
        self.entries.reserve(other.len());
        while let Some((key, value)) = other.pop_last() {
            self.insert(key, value);
        }
    }

    /// Constructs a double-ended iterator over a sub-range of elements in [`SortedVecMap`].
    /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
    /// yield elements from min (inclusive) to max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
    /// `range((Excluded(4), Included(10)))` will yield a left-exclusive,
    /// right-inclusive range from 4 to 10.
    ///
    /// Unlike [`BTreeMap::range`](std::collections::BTreeMap::range) this returns an empty iterator if range `start > end`
    /// or if range `start == end` and both bounds are `Excluded`.
    pub fn range<T, R>(&self, range: R) -> impl DoubleEndedIterator<Item = (&'_ K, &'_ V)>
    where
        T: Ord,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        self.entries
            .iter()
            .filter(move |entry| range.contains(entry.key.borrow()))
            .map(Entry::tuple)
    }

    /// Constructs a mutable double-ended iterator over a sub-range of elements in [`SortedVecMap`].
    /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
    /// yield elements from min (inclusive) to max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
    /// `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
    /// range from 4 to 10.
    ///
    /// Unlike [`BTreeMap::range_mut`](std::collections::BTreeMap::range_mut) this returns an empty iterator if range `start > end`
    /// or if range `start == end` and both bounds are `Excluded`.
    pub fn range_mut<T, R>(
        &mut self,
        range: R,
    ) -> impl DoubleEndedIterator<Item = (&'_ K, &'_ mut V)>
    where
        T: Ord,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        self.entries
            .iter_mut()
            .filter(move |entry| range.contains(entry.key.borrow()))
            .map(|entry| (&entry.key, &mut entry.value))
    }

    /// Creates a consuming iterator visiting all the keys, in sorted order.
    pub fn into_keys(self) -> impl DoubleEndedIterator<Item = K> {
        self.entries.into_iter().map(|entry| entry.key)
    }

    /// Creates a consuming iterator visiting all the values, in order by key.
    pub fn into_values(self) -> impl DoubleEndedIterator<Item = V> {
        self.entries.into_iter().map(|entry| entry.value)
    }

    /// Gets an iterator over the entries of [`SortedVecMap`], sorted by key.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&'_ K, &'_ V)> {
        self.entries.iter().map(Entry::tuple)
    }

    /// Gets a mutable iterator over the entries of [`SortedVecMap`], sorted by key.
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (&'_ mut K, &'_ mut V)> {
        self.entries
            .iter_mut()
            .map(|entry| (&mut entry.key, &mut entry.value))
    }

    /// Gets an iterator over the keys of [`SortedVecMap`], in sorted order.
    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &'_ K> {
        self.entries.iter().map(|entry| &entry.key)
    }

    /// Gets an iterator over the values of [`SortedVecMap`], in order by key.
    pub fn values(&self) -> impl DoubleEndedIterator<Item = &'_ V> {
        self.entries.iter().map(|entry| &entry.value)
    }

    /// Gets a mutable iterator over the values of [`SortedVecMap`], in order by key.
    pub fn values_mut(&mut self) -> impl DoubleEndedIterator<Item = &'_ mut V> {
        self.entries.iter_mut().map(|entry| &mut entry.value)
    }

    /// Returns the number of elements in [`SortedVecMap`].
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if [`SortedVecMap`] contains no elements.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<K, V, Q> Index<&Q> for SortedVecMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `SortedVecMap`.
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl<K: Ord + Copy, V, const N: usize> From<[(K, V); N]> for SortedVecMap<K, V> {
    /// Converts a `[(K, V); N]` into a `SortedVecMap<K, V>`.
    fn from(arr: [(K, V); N]) -> Self {
        // use stable sort to preserve the insertion order.
        let mut entries: Vec<Entry<K, V>> = Vec::from(arr)
            .into_iter()
            .map(|(key, value)| Entry { key, value })
            .collect();
        entries.sort_by_key(|a| a.key);
        entries.dedup_by_key(|a| a.key);
        SortedVecMap { entries }
    }
}
