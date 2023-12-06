#![cfg(feature = "abi")]
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use pbc_contract_common::sorted_vec_map::{SortedVec, SortedVecMap, SortedVecSet};
use pbc_traits::CreateTypeSpec;

#[test]
pub fn ty_names_simple_types() {
    assert_eq!(u8::__ty_name(), "u8");
    assert_eq!(i8::__ty_name(), "i8");
    assert_eq!(u16::__ty_name(), "u16");
    assert_eq!(i16::__ty_name(), "i16");
    assert_eq!(u32::__ty_name(), "u32");
    assert_eq!(i32::__ty_name(), "i32");
    assert_eq!(u64::__ty_name(), "u64");
    assert_eq!(i64::__ty_name(), "i64");
    assert_eq!(u128::__ty_name(), "u128");
    assert_eq!(i128::__ty_name(), "i128");
    assert_eq!(String::__ty_name(), "String");
    assert_eq!(bool::__ty_name(), "bool");
}

#[test]
pub fn ty_names_complex_types() {
    assert_eq!(<BTreeSet<String>>::__ty_name(), "BTreeSet<String>");
    assert_eq!(<Vec<String>>::__ty_name(), "Vec<String>");
    assert_eq!(<VecDeque<String>>::__ty_name(), "VecDeque<String>");
    assert_eq!(<VecDeque<String>>::__ty_identifier(), "VecDeque<String>");
    assert_eq!(<Option<String>>::__ty_name(), "Option<String>");
    assert_eq!(<Option<String>>::__ty_identifier(), "Option<String>");

    assert_eq!(
        <Vec<Vec<Vec<BTreeSet<BTreeSet<Vec<BTreeSet<String>>>>>>>>::__ty_name(),
        "Vec<Vec<Vec<BTreeSet<BTreeSet<Vec<BTreeSet<String>>>>>>>"
    );
}

#[test]
pub fn ty_names_arrays() {
    assert_eq!(<[u8; 1]>::__ty_name(), "[u8; 1]");
    assert_eq!(<[u8; 2]>::__ty_name(), "[u8; 2]");
    assert_eq!(<[u8; 3]>::__ty_name(), "[u8; 3]");
    assert_eq!(<[u8; 4]>::__ty_name(), "[u8; 4]");
    assert_eq!(<[u8; 5]>::__ty_name(), "[u8; 5]");
    assert_eq!(<[u8; 6]>::__ty_name(), "[u8; 6]");
    assert_eq!(<[u8; 7]>::__ty_name(), "[u8; 7]");
    assert_eq!(<[u8; 8]>::__ty_name(), "[u8; 8]");
    assert_eq!(<[u8; 9]>::__ty_name(), "[u8; 9]");
    assert_eq!(<[u8; 10]>::__ty_name(), "[u8; 10]");
    assert_eq!(<[u8; 11]>::__ty_name(), "[u8; 11]");
    assert_eq!(<[u8; 12]>::__ty_name(), "[u8; 12]");
    assert_eq!(<[u8; 13]>::__ty_name(), "[u8; 13]");
    assert_eq!(<[u8; 14]>::__ty_name(), "[u8; 14]");
    assert_eq!(<[u8; 15]>::__ty_name(), "[u8; 15]");
    assert_eq!(<[u8; 16]>::__ty_name(), "[u8; 16]");
    assert_eq!(<[u8; 17]>::__ty_name(), "[u8; 17]");
    assert_eq!(<[u8; 18]>::__ty_name(), "[u8; 18]");
    assert_eq!(<[u8; 19]>::__ty_name(), "[u8; 19]");
    assert_eq!(<[u8; 20]>::__ty_name(), "[u8; 20]");
    assert_eq!(<[u8; 21]>::__ty_name(), "[u8; 21]");
    assert_eq!(<[u8; 22]>::__ty_name(), "[u8; 22]");
    assert_eq!(<[u8; 23]>::__ty_name(), "[u8; 23]");
    assert_eq!(<[u8; 24]>::__ty_name(), "[u8; 24]");
    assert_eq!(<[u8; 25]>::__ty_name(), "[u8; 25]");
    assert_eq!(<[u8; 26]>::__ty_name(), "[u8; 26]");
    assert_eq!(<[u8; 27]>::__ty_name(), "[u8; 27]");
    assert_eq!(<[u8; 28]>::__ty_name(), "[u8; 28]");
    assert_eq!(<[u8; 29]>::__ty_name(), "[u8; 29]");
    assert_eq!(<[u8; 30]>::__ty_name(), "[u8; 30]");
    assert_eq!(<[u8; 31]>::__ty_name(), "[u8; 31]");
    assert_eq!(<[u8; 32]>::__ty_name(), "[u8; 32]");
    assert_eq!(<[u8; 101]>::__ty_name(), "[u8; 101]");
}

#[track_caller]
fn assert_ty<T: CreateTypeSpec>(ord: &[u8]) {
    let mut vec = Vec::new();
    T::__ty_spec_write(&mut vec, &BTreeMap::new());
    assert_eq!(&vec, ord);
}

#[test]
pub fn ty_ordinals_simple_types() {
    assert_ty::<u8>(&[0x01]);
    assert_ty::<u16>(&[0x02]);
    assert_ty::<u32>(&[0x03]);
    assert_ty::<u64>(&[0x04]);
    assert_ty::<u128>(&[0x05]);

    assert_ty::<i8>(&[0x06]);
    assert_ty::<i16>(&[0x07]);
    assert_ty::<i32>(&[0x08]);
    assert_ty::<i64>(&[0x09]);
    assert_ty::<i128>(&[0x0a]);

    assert_ty::<String>(&[0x0b]);
    assert_ty::<bool>(&[0x0c]);
}

#[test]
pub fn ty_ordinals_complex_types() {
    assert_ty::<Vec<u8>>(&[0x0e, 0x01]);
    assert_ty::<Vec<u16>>(&[0x0e, 0x02]);
    assert_ty::<Vec<u32>>(&[0x0e, 0x03]);
    assert_ty::<Vec<u64>>(&[0x0e, 0x04]);
    assert_ty::<Vec<u128>>(&[0x0e, 0x05]);

    assert_ty::<Vec<i8>>(&[0x0e, 0x06]);
    assert_ty::<Vec<i16>>(&[0x0e, 0x07]);
    assert_ty::<Vec<i32>>(&[0x0e, 0x08]);
    assert_ty::<Vec<i64>>(&[0x0e, 0x09]);
    assert_ty::<Vec<i128>>(&[0x0e, 0x0a]);

    assert_ty::<VecDeque<u8>>(&[0x0e, 0x01]);
    assert_ty::<VecDeque<u16>>(&[0x0e, 0x02]);
    assert_ty::<VecDeque<u32>>(&[0x0e, 0x03]);
    assert_ty::<VecDeque<u64>>(&[0x0e, 0x04]);
    assert_ty::<VecDeque<u128>>(&[0x0e, 0x05]);

    assert_ty::<VecDeque<i8>>(&[0x0e, 0x06]);
    assert_ty::<VecDeque<i16>>(&[0x0e, 0x07]);
    assert_ty::<VecDeque<i32>>(&[0x0e, 0x08]);
    assert_ty::<VecDeque<i64>>(&[0x0e, 0x09]);
    assert_ty::<VecDeque<i128>>(&[0x0e, 0x0a]);

    assert_ty::<Option<u8>>(&[0x12, 0x01]);
    assert_ty::<Option<u16>>(&[0x12, 0x02]);
    assert_ty::<Option<u32>>(&[0x12, 0x03]);
    assert_ty::<Option<u64>>(&[0x12, 0x04]);
    assert_ty::<Option<u128>>(&[0x12, 0x05]);

    assert_ty::<Option<i8>>(&[0x12, 0x06]);
    assert_ty::<Option<i16>>(&[0x12, 0x07]);
    assert_ty::<Option<i32>>(&[0x12, 0x08]);
    assert_ty::<Option<i64>>(&[0x12, 0x09]);
    assert_ty::<Option<i128>>(&[0x12, 0x0a]);
    assert_ty::<Option<Option<i128>>>(&[0x12, 0x12, 0x0a]);

    assert_ty::<BTreeSet<i128>>(&[0x10, 0x0a]);

    assert_ty::<BTreeSet<Vec<BTreeSet<Vec<Vec<String>>>>>>(&[0x10, 0x0e, 0x10, 0x0e, 0x0e, 0x0b]);
}

#[test]
pub fn ty_ordinals_arrays() {
    assert_ty::<[u8; 1]>(&[0x11, 0x1]);
    assert_ty::<[u8; 2]>(&[0x11, 0x2]);
    assert_ty::<[u8; 3]>(&[0x11, 0x3]);
    assert_ty::<[u8; 4]>(&[0x11, 0x4]);
    assert_ty::<[u8; 5]>(&[0x11, 0x5]);
    assert_ty::<[u8; 6]>(&[0x11, 0x6]);
    assert_ty::<[u8; 7]>(&[0x11, 0x7]);
    assert_ty::<[u8; 8]>(&[0x11, 0x8]);
    assert_ty::<[u8; 9]>(&[0x11, 0x9]);
    assert_ty::<[u8; 10]>(&[0x11, 0xa]);
    assert_ty::<[u8; 11]>(&[0x11, 0xb]);
    assert_ty::<[u8; 12]>(&[0x11, 0xc]);
    assert_ty::<[u8; 13]>(&[0x11, 0xd]);
    assert_ty::<[u8; 14]>(&[0x11, 0xe]);
    assert_ty::<[u8; 15]>(&[0x11, 0xf]);
    assert_ty::<[u8; 16]>(&[0x11, 0x10]);
    assert_ty::<[u8; 17]>(&[0x11, 0x11]);
    assert_ty::<[u8; 18]>(&[0x11, 0x12]);
    assert_ty::<[u8; 19]>(&[0x11, 0x13]);
    assert_ty::<[u8; 20]>(&[0x11, 0x14]);
    assert_ty::<[u8; 21]>(&[0x11, 0x15]);
    assert_ty::<[u8; 22]>(&[0x11, 0x16]);
    assert_ty::<[u8; 23]>(&[0x11, 0x17]);
    assert_ty::<[u8; 24]>(&[0x11, 0x18]);
    assert_ty::<[u8; 25]>(&[0x11, 0x19]);
    assert_ty::<[u8; 26]>(&[0x11, 0x1a]);
    assert_ty::<[u8; 27]>(&[0x11, 0x1b]);
    assert_ty::<[u8; 28]>(&[0x11, 0x1c]);
    assert_ty::<[u8; 29]>(&[0x11, 0x1d]);
    assert_ty::<[u8; 30]>(&[0x11, 0x1e]);
    assert_ty::<[u8; 31]>(&[0x11, 0x1f]);
    assert_ty::<[u8; 32]>(&[0x11, 0x20]);
    assert_ty::<[u8; 101]>(&[0x11, 101]);
}

#[test]
pub fn sorted_vec() {
    assert_eq!(<SortedVec<u32>>::__ty_name(), "SortedVec<u32>");
    assert_eq!(
        <SortedVec<SortedVec<u32>>>::__ty_name(),
        "SortedVec<SortedVec<u32>>"
    );
    assert_eq!(<SortedVec<u32>>::__ty_identifier(), "SortedVec<u32>");
    assert_eq!(
        <SortedVec<SortedVec<u32>>>::__ty_identifier(),
        "SortedVec<SortedVec<u32>>"
    );
    assert_ty::<SortedVec<u32>>(&[0x0e, 0x03]);
    assert_ty::<SortedVec<SortedVec<u32>>>(&[0x0e, 0x0e, 0x03]);
}

#[test]
pub fn sorted_vec_map() {
    assert_eq!(
        <SortedVecMap<u32, String>>::__ty_name(),
        "SortedVecMap<u32, String>"
    );
    assert_eq!(
        <SortedVecMap<u32, SortedVecMap<String, u32>>>::__ty_name(),
        "SortedVecMap<u32, SortedVecMap<String, u32>>"
    );
    assert_eq!(
        <SortedVecMap<u32, String>>::__ty_identifier(),
        "SortedVecMap<u32, String>"
    );
    assert_eq!(
        <SortedVecMap<u32, SortedVecMap<String, u32>>>::__ty_identifier(),
        "SortedVecMap<u32, SortedVecMap<String, u32>>"
    );
    assert_ty::<SortedVecMap<u32, String>>(&[0x0f, 0x03, 0xb]);
    assert_ty::<SortedVecMap<u32, SortedVecMap<String, u32>>>(&[0x0f, 0x03, 0x0f, 0xb, 0x03]);
}

#[test]
pub fn sorted_vec_set() {
    assert_eq!(<SortedVecSet<u32>>::__ty_name(), "SortedVecSet<u32>");
    assert_eq!(
        <SortedVecSet<SortedVecSet<u32>>>::__ty_name(),
        "SortedVecSet<SortedVecSet<u32>>"
    );
    assert_eq!(<SortedVecSet<u32>>::__ty_identifier(), "SortedVecSet<u32>");
    assert_eq!(
        <SortedVecSet<SortedVecSet<u32>>>::__ty_identifier(),
        "SortedVecSet<SortedVecSet<u32>>"
    );
    assert_ty::<SortedVecSet<u32>>(&[0x10, 0x03]);
    assert_ty::<SortedVecSet<SortedVecSet<u32>>>(&[0x10, 0x10, 0x03]);
}
