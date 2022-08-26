#[cfg(feature = "abi")]
use std::collections::BTreeMap;
#[cfg(feature = "abi")]
use std::collections::BTreeSet;

#[cfg(feature = "abi")]
use create_type_spec_derive::CreateTypeSpec;
#[cfg(feature = "abi")]
use pbc_contract_common::abi::AbiSerialize;
#[cfg(feature = "abi")]
use pbc_contract_common::abi::NamedEntityAbi;
#[cfg(feature = "abi")]
use pbc_contract_common::abi::TypeAbi;
#[cfg(feature = "abi")]
use pbc_traits::CreateTypeSpec;

#[allow(dead_code)]
#[cfg(feature = "abi")]
#[derive(CreateTypeSpec)]
struct DeriveAbiForMe {
    a: String,
    b: u64,
    c: u8,
    d: Vec<Vec<Vec<Vec<u8>>>>,
}

#[allow(dead_code)]
#[cfg(feature = "abi")]
#[derive(CreateTypeSpec)]
struct Nested {
    derived: DeriveAbiForMe,
}

#[cfg(feature = "abi")]
fn assert_field(field: &NamedEntityAbi, name: &str, ord: Vec<u8>, type_index: Option<u8>) {
    assert_eq!(&field.name, name);
    assert_eq!(field.type_spec, ord);
    assert_eq!(field.type_index, type_index);
}

#[test]
#[cfg(feature = "abi")]
fn implemented_create_type_spec_trait() {
    assert_eq!(DeriveAbiForMe::__ty_name(), "DeriveAbiForMe".to_string());

    let mut ordinal: Vec<u8> = Vec::new();
    DeriveAbiForMe::__ty_spec_write(&mut ordinal, &BTreeMap::new());
    assert_eq!(ordinal, vec![0x00, 0x00]);
}

#[test]
#[cfg(feature = "abi")]
fn derived_for_struct() {
    let lut: BTreeMap<String, u8> = BTreeMap::new();

    let abi: TypeAbi = __abi_for_type_deriveabiforme(&lut);
    assert_eq!(abi.name, "DeriveAbiForMe".to_string());
    assert_eq!(abi.type_spec, vec![0x00, 0x00]);

    assert_eq!(abi.fields.len(), 4);
    assert_field(abi.fields.get(0).unwrap(), "a", vec![0x0b], None);
    assert_field(abi.fields.get(1).unwrap(), "b", vec![0x04], None);
    assert_field(abi.fields.get(2).unwrap(), "c", vec![0x01], None);
    assert_field(
        abi.fields.get(3).unwrap(),
        "d",
        vec![0x0e, 0x0e, 0x0e, 0x0e, 0x01],
        None,
    );
}

#[test]
#[cfg(feature = "abi")]
fn nested_structs() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();
    lut.insert(DeriveAbiForMe::__ty_identifier(), 42);

    let abi: TypeAbi = __abi_for_type_nested(&lut);
    assert_eq!(abi.name, "Nested".to_string());
    assert_eq!(abi.type_spec, vec![0x00, 0x00]);

    assert_eq!(abi.fields.len(), 1);
    assert_field(
        abi.fields.get(0).unwrap(),
        "derived",
        vec![0x00, 42],
        Some(42),
    );
}

#[cfg(feature = "abi")]
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, CreateTypeSpec)]
struct Inner {
    x: u8,
}

#[cfg(feature = "abi")]
#[derive(Clone, CreateTypeSpec)]
struct Outer {
    #[allow(dead_code)]
    inner: Inner,
}

#[cfg(feature = "abi")]
#[derive(Clone, CreateTypeSpec)]
struct OuterBTreeMapKey {
    #[allow(dead_code)]
    inner: BTreeMap<Inner, String>,
}

#[cfg(feature = "abi")]
#[derive(Clone, CreateTypeSpec)]
struct OuterBTreeMapValue {
    #[allow(dead_code)]
    inner: BTreeMap<String, Inner>,
}

#[cfg(feature = "abi")]
#[derive(Clone, CreateTypeSpec)]
struct OuterVec {
    #[allow(dead_code)]
    inner: Vec<Inner>,
}

#[cfg(feature = "abi")]
#[derive(Clone, CreateTypeSpec)]
struct OuterBTreeSet {
    #[allow(dead_code)]
    inner: BTreeSet<Inner>,
}

#[cfg(feature = "abi")]
#[derive(Clone, CreateTypeSpec)]
struct OuterComposite {
    #[allow(dead_code)]
    inner: Vec<BTreeSet<Vec<BTreeMap<Inner, String>>>>,
}

#[cfg(feature = "abi")]
#[derive(Clone, CreateTypeSpec)]
struct WithArray {
    #[allow(dead_code)]
    inner: [u8; 50],
}

#[cfg(feature = "abi")]
#[test]
fn serialize_inner() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();
    lut.insert(Inner::__ty_identifier(), 1);

    let a: TypeAbi = __abi_for_type_inner(&lut);

    let result: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x05, // Length of name
        73, 110, 110, 101, 114, // "Inner"
        0x00, 0x00, 0x00, 0x01, // 1 field
        0x00, 0x00, 0x00, 0x01, // 1 character name
        120,  // x
        0x01, // u8
    ];
    assert_abi(a, result);
}

#[cfg(feature = "abi")]
#[test]
fn serialize_outer_with_inner_struct() {
    // Look up table for types
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(Outer::__ty_identifier(), 1);

    let result: TypeAbi = __abi_for_type_outer(&lut);
    let expected = vec![
        0x00, 0x00, 0x00, 0x05, // Length of name
        79, 117, 116, 101, 114, // "Outer"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, len 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x00, 0x02, // Struct at index = 2
    ];
    assert_abi(result, expected);
}

#[cfg(feature = "abi")]
#[test]
fn serialize_inner_as_key_in_btreemap() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterBTreeMapKey::__ty_identifier(), 1);

    let result: TypeAbi = __abi_for_type_outerbtreemapkey(&lut);
    let expected = vec![
        0x00, 0x00, 0x00, 16, // Length of name = 16
        79, 117, 116, 101, 114, 66, 84, 114, 101, 101, 77, 97, 112, 75, 101,
        121, // The string "OuterBTreeMapKey"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, length = 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0f, 0, 2, 0x0b, // BTreeMap<Struct at index = 2, String>
    ];
    assert_abi(result, expected);
}

#[cfg(feature = "abi")]
#[test]
fn serialize_inner_as_value_in_btreemap() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterBTreeMapValue::__ty_identifier(), 1);

    let result: TypeAbi = __abi_for_type_outerbtreemapvalue(&lut);
    let expected = vec![
        0x00, 0x00, 0x00, 18, // Length of name = 18
        0x4f, 0x75, 0x74, 0x65, 0x72, 0x42, 0x54, 0x72, 0x65, 0x65, 0x4d, 0x61, 0x70, 0x56, 0x61,
        0x6c, 0x75, 0x65, // The string "OuterBTreeMapValue"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, length = 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0f, 0x0b, 0, 2, // BtreeMap<String, Struct at index = 2>
    ];
    assert_abi(result, expected);
}

#[cfg(feature = "abi")]
#[test]
fn serialize_inner_in_btreeset() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterBTreeSet::__ty_identifier(), 1);

    let result: TypeAbi = __abi_for_type_outerbtreeset(&lut);
    let expected = vec![
        0x00, 0x00, 0x00, 13, // Length of name = 13
        79, 117, 116, 101, 114, 66, 84, 114, 101, 101, 83, 101,
        116, // The string "OuterBTreeSet"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, length = 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x10, 0, 2, // BTreeSet<Struct at index = 2>
    ];
    assert_abi(result, expected);
}

#[cfg(feature = "abi")]
#[test]
fn serialize_inner_in_vec() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterVec::__ty_identifier(), 1);

    let result: TypeAbi = __abi_for_type_outervec(&lut);
    let expected = vec![
        0x00, 0x00, 0x00, 8, // Length of name = 8
        79, 117, 116, 101, 114, 86, 101, 99, // The string "OuterVec"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, len 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0e, 0, 2, // Vec<Struct at index = 2>
    ];
    assert_abi(result, expected);
}

#[cfg(feature = "abi")]
#[test]
fn serialize_inner_in_composite() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterComposite::__ty_identifier(), 1);

    let result: TypeAbi = __abi_for_type_outercomposite(&lut);
    let expected = vec![
        0x00, 0x00, 0x00, 14, // Length of name = 8
        79, 117, 116, 101, 114, 67, 111, 109, 112, 111, 115, 105, 116,
        101, // The string "OuterComposite"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, len 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0e, 0x10, 0x0e, 0x0f, 0x00, 0x02,
        0x0b, // Vec<BTreeSet<Vec<BTreeMap<Inner, with index = 2, String>>>>
    ];
    assert_abi(result, expected);
}

#[cfg(feature = "abi")]
#[test]
fn serialize_struct_with_array() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(WithArray::__ty_identifier(), 1);

    let result: TypeAbi = __abi_for_type_witharray(&lut);
    let expected = vec![
        0x00, 0x00, 0x00, 9, // Length of name
        b'W', b'i', b't', b'h', b'A', b'r', b'r', b'a', b'y', // Name
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length
        b'i', b'n', b'n', b'e', b'r', // Field name
        17, 50, // Array with length 50.
    ];
    assert_abi(result, expected);
}

#[cfg(feature = "abi")]
fn assert_abi<T: AbiSerialize>(obj: T, expected: Vec<u8>) {
    let mut actual = Vec::new();
    obj.serialize_abi(&mut actual).unwrap();
    assert_eq!(actual, expected);
}
