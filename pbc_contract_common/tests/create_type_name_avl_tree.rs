#![cfg(feature = "abi")]
use pbc_contract_common::avl_tree_map::AvlTreeMap;
use std::collections::BTreeMap;

use pbc_traits::CreateTypeSpec;

fn assert_ty<T: CreateTypeSpec>(ord: &[u8]) {
    let mut vec = Vec::new();
    T::__ty_spec_write(&mut vec, &BTreeMap::new());
    assert_eq!(&vec, ord);
}

#[test]
pub fn names() {
    assert_eq!(
        AvlTreeMap::<u32, String>::__ty_name(),
        "AvlTreeMap<u32, String>"
    );
    assert_eq!(
        AvlTreeMap::<u32, String>::__ty_identifier(),
        "AvlTreeMap<u32, String>"
    );
}

#[test]
pub fn ty_ordinals_simple_types() {
    assert_ty::<AvlTreeMap<u32, String>>(&[0x19, 0x03, 0x0b]);
}
