//! Allows for initialization of AvlTree and its corresponding getter, insert, contains_key and remove functions.
//!
#[allow(unused_imports)]
use std::slice::from_raw_parts;

/// Functions for interacting wth the WASM invoker.
#[cfg(all(not(feature = "abi"), any(target_arch = "wasm32", doc)))]
#[link(wasm_import_module = "pbc")]
extern "C" {
    /// Creates a new AvlTree in the WASM state with a unique `tree_id`.
    #[link_name = "avl_new"]
    fn pbc_avl_new() -> i32;

    /// Tries to get the value for the given key. Returns a boolean indicating success of retrieval.
    #[link_name = "avl_get"]
    fn pbc_avl_get(
        tree_id: i32,
        key_ptr: *const u8,
        key_len: usize,
        destination: *const u8,
    ) -> bool;

    /// Inserts for the value for the given key. If there already exists a value it is overwritten and not returned.
    #[link_name = "avl_insert"]
    fn pbc_avl_insert(
        tree_id: i32,
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    );

    /// Removes the key and value in the WASM state
    #[link_name = "avl_remove"]
    fn pbc_avl_remove(tree_id: i32, key_ptr: *const u8, key_len: usize);

    /// Gets the size in bytes of the value object in the WASM state.
    /// If no value exists for the given key usize::MAX is returned.
    #[link_name = "avl_get_size"]
    fn pbc_avl_get_size(tree_id: i32, key_ptr: *const u8, key_len: usize) -> usize;

    /// Gets the size of the avl tree.
    #[link_name = "avl_len"]
    fn pbc_avl_len(tree_id: i32) -> usize;

    #[link_name = "avl_get_next"]
    fn pbc_avl_get_next(
        tree_id: i32,
        key_ptr: *const u8,
        key_len: usize,
        destination: *const u8,
    ) -> bool;

    #[link_name = "avl_get_next_size"]
    fn pbc_avl_get_next_size(tree_id: i32, key_ptr: *const u8, key_len: usize) -> usize;
}

#[cfg(all(not(feature = "abi"), any(target_arch = "wasm32", doc)))]
pub mod wasm_avl {
    use super::*;
    use std::ptr::null;

    const U32_MAX: usize = u32::MAX as usize;

    /// Initialize the AvlTree.
    pub fn new() -> i32 {
        unsafe { pbc_avl_new() }
    }

    /// Gets value bytes in AvlTree for given key bytes.
    pub fn get(tree_id: i32, key_bytes: &[u8], destination: &mut [u8]) -> bool {
        unsafe {
            pbc_avl_get(
                tree_id,
                key_bytes.as_ptr(),
                key_bytes.len(),
                destination.as_ptr(),
            )
        }
    }

    /// Inserts key and value bytes into AvlTree.
    pub fn insert(tree_id: i32, key_bytes: &[u8], value_bytes: &[u8]) {
        unsafe {
            pbc_avl_insert(
                tree_id,
                key_bytes.as_ptr(),
                key_bytes.len(),
                value_bytes.as_ptr(),
                value_bytes.len(),
            );
        }
    }

    /// Remove key and value bytes from AvlTree.
    pub fn remove(tree_id: i32, key_bytes: &[u8]) {
        unsafe {
            pbc_avl_remove(tree_id, key_bytes.as_ptr(), key_bytes.len());
        }
    }

    /// Gets value bytes size in AvlTree for given key bytes.
    pub fn get_size(tree_id: i32, key_bytes: &[u8]) -> usize {
        unsafe { pbc_avl_get_size(tree_id, key_bytes.as_ptr(), key_bytes.len()) }
    }

    /// Gets the number of elements in the avl tree
    pub fn avl_tree_len(tree_id: i32) -> usize {
        unsafe { pbc_avl_len(tree_id) }
    }

    /// Get the next entry in the avl tree
    pub fn get_next(tree_id: i32, key_bytes: Option<&[u8]>, destination: &mut [u8]) -> bool {
        let (key_ptr, key_len) = if let Some(key_bytes) = key_bytes {
            (key_bytes.as_ptr(), key_bytes.len())
        } else {
            (null(), u32::MAX as usize)
        };
        unsafe { pbc_avl_get_next(tree_id, key_ptr, key_len, destination.as_ptr()) }
    }

    /// Gets the size of the next entry in the avl tree
    pub fn get_next_size(tree_id: i32, key_bytes: Option<&[u8]>) -> usize {
        let (key_ptr, key_len) = if let Some(key_bytes) = key_bytes {
            (key_bytes.as_ptr(), key_bytes.len())
        } else {
            (null(), u32::MAX as usize)
        };
        unsafe { pbc_avl_get_next_size(tree_id, key_ptr, key_len) }
    }
}

/// Native implementations of the wasm avl tree maps.
/// Used when testing contracts for coverage, as they then are run using a native runner.
#[cfg(not(all(not(feature = "abi"), any(target_arch = "wasm32", doc))))]
pub mod wasm_avl {
    use once_cell::sync::Lazy;
    use pbc_traits::ReadInt;
    use std::collections::{BTreeMap, HashMap};
    use std::io::Read;
    use std::ops::Bound::{Excluded, Unbounded};
    use std::sync::Mutex;
    use std::thread;
    use std::thread::ThreadId;

    type BackingMapType = BTreeMap<i32, BTreeMap<Vec<u8>, Vec<u8>>>;
    /// Native map to mimic the hosted avl tree map in the wasm invoker
    static BACKING_MAP: Lazy<Mutex<HashMap<ThreadId, BackingMapType>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    fn get_backing_map<A>(f: impl FnOnce(&mut BackingMapType) -> A) -> A {
        let mut guard = BACKING_MAP.lock().unwrap();
        let map: &mut BackingMapType = guard
            .entry(thread::current().id())
            .or_insert(BTreeMap::new());
        f(map)
    }

    /// Deserialize avl trees into backing map.
    /// The input bytes are serialized as the state representation of the abi type
    /// `Map<Option<i32>, Option<Option<Map<Option<Option<Vec<u8>>>, Option<Vec<u8>>>>>>`
    /// with every option being a Some value. These option bytes are ignored.
    pub fn deserialize_avl_tree(avl_tree_bytes: &mut &[u8]) {
        // length of map
        let length = avl_tree_bytes.read_u32_le();
        for _ in 0..length {
            // option some
            avl_tree_bytes.read_u8();
            // tree id
            let tree_id = avl_tree_bytes.read_i32_le();
            // 2x option some
            avl_tree_bytes.read_u16_le();
            // length of inner map
            let inner_length = avl_tree_bytes.read_u32_le();
            let mut inner_map: BTreeMap<Vec<u8>, Vec<u8>> = BTreeMap::new();
            for _ in 0..inner_length {
                // 2x option some
                avl_tree_bytes.read_u16_le();
                // key length
                let key_length = avl_tree_bytes.read_u32_le() as usize;
                let mut key: Vec<u8> = vec![0; key_length];
                // key bytes
                avl_tree_bytes.read_exact(&mut key).unwrap();

                // option some
                avl_tree_bytes.read_u8();
                // value length
                let value_length = avl_tree_bytes.read_u32_le() as usize;
                let mut value: Vec<u8> = vec![0; value_length];
                // value bytes
                avl_tree_bytes.read_exact(&mut value).unwrap();
                inner_map.insert(key, value);
            }
            get_backing_map(|map| map.insert(tree_id, inner_map.clone()));
        }
    }

    /// Initialize the AvlTree.
    pub fn new() -> i32 {
        let tree_id = get_backing_map(|map| map.keys().max().unwrap_or(&-1) + 1);
        get_backing_map(|map| map.insert(tree_id, BTreeMap::new()));
        tree_id
    }

    /// Gets value bytes in AvlTree for given key bytes.
    pub fn get(tree_id: i32, key_bytes: &[u8], destination: &mut [u8]) -> bool {
        get_backing_map(|map| {
            let value = map.get(&tree_id).unwrap().get(key_bytes);
            if let Some(val) = value {
                destination.clone_from_slice(val);
                true
            } else {
                false
            }
        })
    }

    /// Inserts key and value bytes into AvlTree.
    pub fn insert(tree_id: i32, key_bytes: &[u8], value_bytes: &[u8]) {
        get_backing_map(|map| {
            map.get_mut(&tree_id)
                .unwrap()
                .insert(key_bytes.into(), value_bytes.into())
        });
    }

    /// Remove key and value bytes from AvlTree.
    pub fn remove(tree_id: i32, key_bytes: &[u8]) {
        get_backing_map(|map| map.get_mut(&tree_id).unwrap().remove(key_bytes));
    }

    /// Gets value bytes size in AvlTree for given key bytes.
    pub fn get_size(tree_id: i32, key_bytes: &[u8]) -> usize {
        get_backing_map(|map| {
            let value = map.get(&tree_id).unwrap().get(key_bytes);
            if let Some(val) = value {
                val.len()
            } else {
                u32::MAX as usize
            }
        })
    }

    /// Gets the number of elements in the avl tree
    pub fn avl_tree_len(tree_id: i32) -> usize {
        get_backing_map(|map| map.get(&tree_id).unwrap().len())
    }

    /// Gets next element in native map.
    ///
    /// Assumes that a next value exists, and may crash if not.
    pub fn get_next(tree_id: i32, key_bytes: Option<&[u8]>, destination: &mut [u8]) -> bool {
        get_backing_map(|map| {
            let pair: Option<(&Vec<u8>, &Vec<u8>)> = if let Some(key_bytes) = key_bytes {
                map.get(&tree_id)
                    .unwrap()
                    .range((Excluded(key_bytes.to_vec()), Unbounded))
                    .next()
            } else {
                map.get(&tree_id).unwrap().iter().next()
            };

            if let Some((k, v)) = pair {
                let entry_bytes = [k.as_slice(), v.as_slice()].concat();
                destination.clone_from_slice(&entry_bytes);
            }
            pair.is_some()
        })
    }

    /// Gets the size of the next element in native map.
    pub fn get_next_size(tree_id: i32, key_bytes: Option<&[u8]>) -> usize {
        get_backing_map(|map| {
            let value: Option<(&Vec<u8>, &Vec<u8>)> = if let Some(key_bytes) = key_bytes {
                map.get(&tree_id)
                    .unwrap()
                    .range((Excluded(key_bytes.to_vec()), Unbounded))
                    .next()
            } else {
                map.get(&tree_id).unwrap().iter().next()
            };
            if let Some(val) = value {
                val.0.len() + val.1.len()
            } else {
                u32::MAX as usize
            }
        })
    }
}
