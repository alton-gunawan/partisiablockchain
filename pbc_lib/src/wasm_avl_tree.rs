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
}

#[cfg(all(not(feature = "abi"), any(target_arch = "wasm32", doc)))]
pub mod wasm_avl {
    use super::*;
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
}

/// Functions for interacting wth the WASM invoker.
#[cfg(not(all(not(feature = "abi"), any(target_arch = "wasm32", doc))))]
pub mod wasm_avl {
    /// Initialize the AvlTree.
    pub fn new() -> i32 {
        0
    }

    /// Gets value bytes in AvlTree for given key bytes.
    pub fn get(_tree_id: i32, _key_bytes: &[u8], _destination: &mut [u8]) -> bool {
        true
    }

    /// Inserts key and value bytes into AvlTree.
    pub fn insert(_tree_id: i32, _key_bytes: &[u8], _value_bytes: &[u8]) {}

    /// Remove key and value bytes from AvlTree.
    pub fn remove(_tree_id: i32, _key_bytes: &[u8]) {}

    /// Gets value bytes size in AvlTree for given key bytes.
    pub fn get_size(_tree_id: i32, _key_bytes: &[u8]) -> usize {
        0
    }
}
