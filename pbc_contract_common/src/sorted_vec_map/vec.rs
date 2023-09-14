//! Provides a sorted [`Vec`].
//!
//! Backed by a simple [`Vec<T>`] for use in PBC smart contracts. [`SortedVec`] provides
//! constant time serialization/deserialization if the elements is serializable by copy (Constant
//! size determinable at compile time).
//!
//! [`SortedVec`] provides amortized `O(n)` insert and `O(log(n))` lookup.

#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use pbc_traits::ReadWriteState;
use read_write_state_derive::ReadWriteState;
use std::borrow::Borrow;
#[cfg(feature = "abi")]
use std::collections::BTreeMap;
use std::fmt::Debug;

use super::SortedVecSet;

/// A sorted [`Vec`] with [`SortedVecSet`]-semantics.
///
/// Implemented using [`SortedVecSet`], which provides a more complete interface. [`SortedVec`]
/// has its [`CreateTypeSpec`] set specifically as a [`Vec`] instead of a `Set`. It is recommended
/// to use [`SortedVecSet`] when possible.
///
/// [`SortedVec`] ensures that elements are stored in sorted [`Ord`]er, which gives the advantage of fast binary searches.
/// The sorted vector has Set semantics, meaning that it can only contain one copy of an element.
///
/// [`SortedVec`] provides amortized `O(n)` insert and `O(log(n))` lookup.
#[derive(Debug, Clone, ReadWriteState)]
pub struct SortedVec<T> {
    /// The contained elements.
    elements: SortedVecSet<T>,
}

/// Implementation of the [`CreateTypeSpec`] trait for [`SortedVec<T>`]
/// for any element and element type `T`, `T` that implement [`CreateTypeSpec`].
#[cfg(feature = "abi")]
impl<T: CreateTypeSpec> CreateTypeSpec for SortedVec<T> {
    /// Type name is `SortedVec<T>`.
    fn __ty_name() -> String {
        format!("SortedVec<{}>", T::__ty_name())
    }

    fn __ty_identifier() -> String {
        format!("SortedVec<{}>", T::__ty_identifier(),)
    }

    /// Ordinal is `0x10` followed by ordinal of `T`.
    /// as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html).
    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        w.push(0x0e); // Vec
        T::__ty_spec_write(w, lut);
    }
}

impl<T> Default for SortedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SortedVec<T> {
    /// Constructor for [`SortedVec`].
    pub fn new() -> Self {
        SortedVec {
            elements: SortedVecSet::new(),
        }
    }

    /// Clears [`SortedVec`], removing all elements.
    pub fn clear(&mut self) {
        self.elements.clear()
    }

    /// Returns a reference to the element corresponding to the element.
    pub fn get<Q>(&self, element: &Q) -> Option<&T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.elements.get(element)
    }

    /// Removes the last element from a vector and returns it, or None if it is empty.
    ///
    /// If you’d like to pop the first element, consider using [`SortedVecSet::pop_first`] instead.
    pub fn pop_last(&mut self) -> Option<T>
    where
        T: Ord,
    {
        self.elements.pop_last()
    }

    /// Returns `true` if [`SortedVec`] contains a element for the specified element.
    pub fn contains<Q>(&self, element: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.elements.contains(element)
    }

    /// Inserts an element into [`SortedVec`].
    ///
    /// If [`SortedVec`] did not have this element present, None is returned.
    /// If [`SortedVec`] did have this element present, the element is updated, and the old element is returned.
    pub fn insert(&mut self, element: T) -> Option<T>
    where
        T: Ord,
    {
        self.elements.insert(element)
    }

    /// Removes a element from [`SortedVec`], returning the element at the element if the element was previously in [`SortedVec`].
    pub fn remove<Q>(&mut self, element: &Q) -> Option<T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.elements.remove(element)
    }

    /// Gets an iterator over the elements of [`SortedVec`], sorted by element.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &'_ T> {
        self.elements.iter()
    }

    /// Returns the number of elements in [`SortedVec`].
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` if [`SortedVec`] contains no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
