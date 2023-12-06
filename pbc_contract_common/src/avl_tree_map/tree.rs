//! Provides a [`AvlTreeMap`] which allows for partial deserialization of state.
//!
//! [`AvlTreeMap`]s are stored in an efficient data-structure as a separate part of the
//! contract state, outside of the Rust-side contract state. Values will be dynamically
//! loaded when requested.
//! This means that only the requested values from the [`AvlTreeMap`] will be deserialized,
//! potentially saving significant amounts of gas, in contrast to [`SortedVecMap`]
//! which will always be fully deserialized when stored in `#[state]`.
//! Partial state deserialization can thereby enable smart contracts to have must bigger states
//! with gas fees independent of the size of the state.

//! ## Example
//! A token contract might use [`AvlTreeMap`] in its state to store balances and allowances:
//!
//! ```rust
//! use pbc_contract_common::address::Address;
//! use pbc_contract_common::avl_tree_map::AvlTreeMap;
//! use read_write_state_derive::ReadWriteState;
//! pub type TokenAmount = u128;
//!
//! #[derive(ReadWriteState, PartialOrd, Ord, Eq, PartialEq)]
//! pub struct AllowancePair {
//!     token_owner: Address,
//!     token_spender: Address,
//! }
//!
//! #[derive(ReadWriteState)]
//! pub struct TokenState {
//!     balances: AvlTreeMap<Address, TokenAmount>,
//!     allowances: AvlTreeMap<AllowancePair, TokenAmount>,
//! }
//! ```
//!
//! Value types can contain structures and even maps:
//!
//! ```
//! use pbc_contract_common::address::Address;
//! use pbc_contract_common::avl_tree_map::AvlTreeMap;
//! use pbc_contract_common::sorted_vec_map::SortedVecSet;
//! use read_write_state_derive::ReadWriteState;
//! #[derive(ReadWriteState, PartialOrd, Ord, Eq, PartialEq)]
//! pub enum Permission { READ = 0, APPEND = 1, MODIFY = 2, EXECUTE = 3 }
//!
//! pub type DocumentUUID = u128;
//! pub type PermissionSetId = u128;
//!
//! #[derive(ReadWriteState)]
//! pub struct PermissionSet {
//!     members: SortedVecSet<Address>,
//!     permissions: SortedVecSet<Permission>,
//!     documents: SortedVecSet<DocumentUUID>,
//! }
//!
//! pub struct DocumentPermissionContractState {
//!     document_permission_sets: AvlTreeMap<PermissionSetId, PermissionSet>,
//! }
//! ```
//!
//! ## Safety
//! Initialized [`AvlTreeMap`]s are permanently stored in the contract state, and cannot be automatically garbage collected.
//! Careful consideration should thus be taken before calling new [`AvlTreeMap::new`], as indiscriminate use will bloat the contract state, and raise the storage fees required.
//!
//! A useful rule of thumb when working with [`AvlTreeMap`]s is to only initialize them in the `#[init]` invocation, and to never use them for internal computations.
//! Using [`AvlTreeMap`]s in the values of other [`AvlTreeMap`] is technically possible, but discourage, due to the garbage collection issues.

#[cfg(feature = "abi")]
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::marker::PhantomData;

use crate::sorted_vec_map::entry::Entry;
use pbc_lib::wasm_avl::{
    avl_tree_len, get, get_next, get_next_size, get_size, insert, new, remove,
};
#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;
use pbc_traits::{ReadInt, ReadWriteState, WriteInt};

const U32_MAX: usize = u32::MAX as usize;

/// [`AvlTreeMap`] provides partial deserialization, enabling bigger smart contract states.
/// [`AvlTreeMap`] must only be used at the top level of the smart contract and should not be
/// initialized for intermediate computation.
#[derive(Debug)]
pub struct AvlTreeMap<K, V> {
    /// Ties the key type to the [`AvlTreeMap`]
    key_type: PhantomData<K>,
    /// Ties the value type to the [`AvlTreeMap`]
    value_type: PhantomData<V>,
    /// Unique id in WASM state.
    tree_id: i32,
}

/// The state contains the unique `tree_id` for which the [`AvlTreeMap`] points to in the WASM state.
impl<K, V> ReadWriteState for AvlTreeMap<K, V> {
    const SERIALIZABLE_BY_COPY: bool = false;

    fn state_read_from<T: Read>(reader: &mut T) -> Self {
        let tree_id = reader.read_i32_le();
        AvlTreeMap {
            key_type: PhantomData,
            value_type: PhantomData,
            tree_id,
        }
    }

    fn state_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_i32_le(self.tree_id)
    }
}

/// Implementation of the [`CreateTypeSpec`] trait for [`AvlTreeMap<K, V>`]
/// for any key and value type `K`, `V` that implement [`CreateTypeSpec`].
#[cfg(feature = "abi")]
impl<K: CreateTypeSpec, V: CreateTypeSpec> CreateTypeSpec for AvlTreeMap<K, V> {
    /// Type name is `AvlTreeMap<K, V>`
    fn __ty_name() -> String {
        format!("AvlTreeMap<{}, {}>", K::__ty_name(), V::__ty_name())
    }

    fn __ty_identifier() -> String {
        format!(
            "AvlTreeMap<{}, {}>",
            K::__ty_identifier(),
            V::__ty_identifier()
        )
    }

    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        w.push(0x19);
        K::__ty_spec_write(w, lut);
        V::__ty_spec_write(w, lut);
    }
}

impl<K: ReadWriteState + Ord, V: ReadWriteState> Default for AvlTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: ReadWriteState + Ord, V: ReadWriteState> AvlTreeMap<K, V> {
    const VALUE_SERIALIZABLE_BY_COPY: bool = <V as ReadWriteState>::SERIALIZABLE_BY_COPY;

    /// Constructor of [`AvlTreeMap`].
    /// ## Safety
    /// Initialized [`AvlTreeMap`]s are permanently stored in the contract state, and cannot
    /// be automatically garbage collected.
    /// Careful consideration should thus be taken before initializing new maps, see
    /// [`AvlTreeMap`] for further discussion of this issue.
    pub fn new() -> Self {
        AvlTreeMap {
            key_type: PhantomData,
            value_type: PhantomData,
            tree_id: new(),
        }
    }

    /// Gets a value for the corresponding key in the [`AvlTreeMap`].
    /// If no value exists for the given key `None` is returned.
    ///
    /// ### Parameter:
    ///
    /// * `key`: the key to retrieve the value from.
    ///
    /// ### Returns:
    ///
    /// * Option containing the value.
    pub fn get(&self, key: &K) -> Option<V> {
        let mut key_bytes = Vec::new();
        key.state_write_to(&mut key_bytes).unwrap();
        let value_size: usize = if AvlTreeMap::<K, V>::VALUE_SERIALIZABLE_BY_COPY {
            std::mem::size_of::<V>()
        } else {
            get_size(self.tree_id, &key_bytes)
        };
        if value_size == U32_MAX {
            return None;
        }
        let mut value_bytes: Vec<u8> = vec![0; value_size];
        if get(self.tree_id, &key_bytes, &mut value_bytes) {
            let value: V = ReadWriteState::state_read_from(&mut value_bytes.as_slice());
            Some(value)
        } else {
            None
        }
    }

    /// Checks if the [`AvlTreeMap`] has the specified key.
    ///
    /// ### Parameter:
    ///
    /// * `key`: the key to check for
    ///
    /// ### Returns:
    ///
    /// * boolean indicating the keys presence in the [`AvlTreeMap`].
    pub fn contains_key(&self, key: &K) -> bool {
        let mut key_bytes = Vec::new();
        key.state_write_to(&mut key_bytes).unwrap();
        U32_MAX != get_size(self.tree_id, &key_bytes)
    }

    /// Inserts a key-value pair into the [`AvlTreeMap`].
    ///
    /// Any value for this key is overwritten if present.
    ///
    /// ### Parameter:
    ///
    /// * `key`: the key to insert
    /// * `value`: the corresponding value to insert
    pub fn insert(&self, key: K, value: V) {
        let mut key_bytes = Vec::new();
        key.state_write_to(&mut key_bytes).unwrap();
        let mut value_bytes = Vec::new();
        value.state_write_to(&mut value_bytes).unwrap();
        insert(self.tree_id, &key_bytes, &value_bytes);
    }

    /// Removes a key-value pair from the [`AvlTreeMap`].
    ///
    /// ### Parameter:
    ///
    /// * `key`: the key to remove from the map
    pub fn remove(&self, key: &K) {
        let mut key_bytes = Vec::new();
        key.state_write_to(&mut key_bytes).unwrap();
        remove(self.tree_id, &key_bytes);
    }

    /// Returns the number of elements in [`AvlTreeMap`].
    pub fn len(&self) -> usize {
        avl_tree_len(self.tree_id)
    }

    /// Returns `true` if [`AvlTreeMap`] contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets an iterator over the entries of [`AvlTreeMap`], sorted by the byte serialization of key.
    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        AvlIterator {
            prev_key: None,
            key_type: PhantomData,
            value_type: PhantomData,
            tree_id: self.tree_id,
        }
    }
}

pub struct AvlIterator<K, V> {
    prev_key: Option<Vec<u8>>,
    /// Ties the key type to the [`AvlTreeMap`]
    key_type: PhantomData<K>,
    /// Ties the value type to the [`AvlTreeMap`]
    value_type: PhantomData<V>,
    /// Unique id in WASM state.
    tree_id: i32,
}

impl<K: ReadWriteState, V: ReadWriteState> Iterator for AvlIterator<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let value_size: usize = if Entry::<K, V>::SERIALIZABLE_BY_COPY {
            std::mem::size_of::<Entry<K, V>>()
        } else {
            get_next_size(self.tree_id, self.prev_key.as_deref())
        };
        if value_size == U32_MAX {
            return None;
        }

        // Fetch entry data
        let mut entry_bytes: Vec<u8> = vec![0; value_size];
        let could_get_next = get_next(self.tree_id, self.prev_key.as_deref(), &mut entry_bytes);
        if !could_get_next {
            return None;
        }

        // Deserialize
        let reader: &mut &[u8] = &mut entry_bytes.as_slice();
        let key: K = ReadWriteState::state_read_from(reader);
        let value_len = reader.len();
        let value: V = ReadWriteState::state_read_from(reader);
        let key_bytes = entry_bytes.split_at(entry_bytes.len() - value_len).0;
        self.prev_key = Some(key_bytes.to_vec());
        Some((key, value))
    }
}
