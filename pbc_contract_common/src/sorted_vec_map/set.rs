//! Provides a sorted [`Vec`]-based set.
//!
//! Backed by a simple [`Vec<T>`] for use in PBC smart contracts. [`SortedVecSet`] provides
//! constant time serialization/deserialization if the elements is serializable by copy (Constant
//! size determinable at compile time).
//!
//! [`SortedVecSet`] provides amortized `O(n)` insert and `O(log(n))` lookup.

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use std::borrow::Borrow;
#[cfg(feature = "abi")]
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::RangeBounds;

use pbc_traits::ReadWriteState;
use read_write_state_derive::ReadWriteState;

/// A [`Vec`]-based set.
/// [`SortedVecSet`] provides amortized `O(n)` insert and `O(log(n))` lookup.
#[derive(PartialEq, Debug, Clone, Eq, ReadWriteState)]
pub struct SortedVecSet<T> {
    /// The contained elements.
    elements: Vec<T>,
}

/// Implementation of the [`CreateTypeSpec`] trait for [`SortedVecSet<T>`]
/// for any element and element type `T`, `T` that implement [`CreateTypeSpec`].
#[cfg(feature = "abi")]
impl<T: CreateTypeSpec> CreateTypeSpec for SortedVecSet<T> {
    /// Type name is `SortedVecSet<T>`.
    fn __ty_name() -> String {
        format!("SortedVecSet<{}>", T::__ty_name())
    }

    fn __ty_identifier() -> String {
        format!("SortedVecSet<{}>", T::__ty_identifier(),)
    }

    /// Ordinal is `0x10` followed by ordinal of `T`.
    /// as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html).
    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        w.push(0x10); // Set
        T::__ty_spec_write(w, lut);
    }
}

impl<T: Ord> SortedVecSet<T> {
    fn get_index_of<Q>(&self, element: &Q) -> Result<usize, usize>
    where
        T: Borrow<Q>,
        Q: Eq + Ord,
    {
        self.elements
            .binary_search_by_key(&element, |element| element.borrow())
    }

    fn get_entry<Q>(&self, element: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Eq + Ord,
    {
        self.get_index_of(element)
            .ok()
            .map(|index| &self.elements[index])
    }
}

impl<T> Default for SortedVecSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SortedVecSet<T> {
    /// Constructor for [`SortedVecSet`].
    pub fn new() -> Self {
        SortedVecSet {
            elements: Vec::new(),
        }
    }

    /// Clears [`SortedVecSet`], removing all elements.
    pub fn clear(&mut self) {
        self.elements.clear()
    }

    /// Returns a reference to the element corresponding to the element.
    pub fn get<Q>(&self, element: &Q) -> Option<&T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.get_entry(element)
    }

    /// Removes and returns the first/minimal element in [`SortedVecSet`].
    pub fn pop_first(&mut self) -> Option<T>
    where
        T: Ord,
    {
        if !self.elements.is_empty() {
            Some(self.elements.remove(0))
        } else {
            None
        }
    }

    /// Removes and returns the last/maximal element in [`SortedVecSet`].
    pub fn pop_last(&mut self) -> Option<T>
    where
        T: Ord,
    {
        self.elements.pop()
    }

    /// Returns `true` if [`SortedVecSet`] contains a element for the specified element.
    pub fn contains<Q>(&self, element: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.get(element).is_some()
    }

    /// Inserts an element into [`SortedVecSet`].
    ///
    /// If [`SortedVecSet`] did not have this element present, None is returned.
    /// If [`SortedVecSet`] did have this element present, the element is updated, and the old element is returned.
    pub fn insert(&mut self, element: T) -> Option<T>
    where
        T: Ord,
    {
        match self.get_index_of(&element) {
            Ok(idx) => {
                self.elements.push(element);
                Some(self.elements.swap_remove(idx))
            }
            Err(idx) => {
                self.elements.insert(idx, element);
                None
            }
        }
    }

    /// Removes a element from [`SortedVecSet`], returning the element at the element if the element was previously in [`SortedVecSet`].
    pub fn remove<Q>(&mut self, element: &Q) -> Option<T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        match self.get_index_of(element) {
            Ok(idx) => Some(self.elements.remove(idx)),
            Err(_) => None,
        }
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&mut self, f: F)
    where
        T: Ord,
        F: FnMut(&T) -> bool,
    {
        self.elements.retain(f)
    }

    /// Moves all elements from other into self, leaving other empty.
    ///
    /// If a element from other is already present in self, the respective element from self will be
    /// overwritten with the respective element from other.
    pub fn append(&mut self, other: &mut Self)
    where
        T: Ord,
    {
        self.elements.reserve(other.len());
        while let Some(element) = other.pop_last() {
            self.insert(element);
        }
    }

    /// Constructs a double-ended iterator over a sub-range of elements in [`SortedVecSet`].
    /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
    /// yield elements from min (inclusive) to max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
    /// `range((Excluded(4), Included(10)))` will yield a left-exclusive,
    /// right-inclusive range from 4 to 10.
    ///
    /// Unlike [`BTreeMap::range`] this returns an empty iterator if range `start > end`
    /// or if range `start == end` and both bounds are `Excluded`.
    pub fn range<'a, K, R>(&'a self, range: R) -> impl DoubleEndedIterator<Item = &'a T>
    where
        K: Ord,
        T: 'a,
        &'a T: Borrow<K>,
        R: RangeBounds<K>,
    {
        self.elements
            .iter()
            .filter(move |element| range.contains(element.borrow()))
    }

    /// Gets an iterator over the elements of [`SortedVecSet`], sorted by element.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &'_ T> {
        self.elements.iter()
    }

    /// Returns the number of elements in [`SortedVecSet`].
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` if [`SortedVecSet`] contains no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> IntoIterator for SortedVecSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Creates a consuming iterator visiting all the keys, in sorted order.
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<T> FromIterator<T> for SortedVecSet<T>
where
    T: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::from(Vec::<T>::from_iter(iter))
    }
}

impl<T: Ord> From<Vec<T>> for SortedVecSet<T> {
    fn from(mut elements: Vec<T>) -> Self {
        // use stable sort to preserve the insertion order.
        elements.sort();
        elements.dedup();
        Self { elements }
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for SortedVecSet<T> {
    /// Converts a `[T; N]` into a `SortedVecSet<T>`.
    fn from(arr: [T; N]) -> Self {
        Self::from(Vec::from(arr))
    }
}
