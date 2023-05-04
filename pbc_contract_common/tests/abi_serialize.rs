#![cfg(feature = "abi")]
use pbc_contract_common::abi::{AbiSerialize, FnAbi, NamedEntityAbi, NamedTypeSpec};
use pbc_contract_common::address::Shortname;
use pbc_contract_common::{FunctionKind, FunctionName};
use std::collections::BTreeMap;

fn assert_serialized_to<T: AbiSerialize>(e: &T, expected_buf: &[u8]) {
    let mut gotten_buf = Vec::<u8>::new();
    e.serialize_abi(&mut gotten_buf).unwrap();
    assert_eq!(gotten_buf.as_slice(), expected_buf);
}

#[test]
pub fn serialize_named_entity_0() {
    let lut = BTreeMap::new();
    let obj = NamedEntityAbi::new::<u64>("name".to_string(), &lut);
    let expected_buf = [
        0, 0, 0, 4, // Name Length
        0x6e, 0x61, 0x6d, 0x65, // Name
        0x04, // Field 0 type ordinal
    ];
    assert_serialized_to(&obj, &expected_buf);
}

#[test]
pub fn serialize_type_abi_0() {
    let obj =
        NamedTypeSpec::new_struct("name".to_string(), "some_uid".to_string(), vec![0x00, 0x00]);
    let expected_buf = [
        1, // It's a struct
        0, 0, 0, 4, // Name Length
        0x6e, 0x61, 0x6d, 0x65, // Name
        0, 0, 0, 0, // Arguments length
    ];
    assert_serialized_to(&obj, &expected_buf);
}

#[test]
pub fn serialize_type_abi_1() {
    let mut obj =
        NamedTypeSpec::new_struct("name".to_string(), "some_uid".to_string(), vec![0x00, 0x00]);

    let lut = BTreeMap::new();
    let field = NamedEntityAbi::new::<u64>("field".to_string(), &lut);

    obj.add_field(field);
    let expected_buf = [
        1, // It's a struct
        0, 0, 0, 4, // Name Length
        0x6e, 0x61, 0x6d, 0x65, // Name
        0, 0, 0, 1, // Arguments length
        0, 0, 0, 5, // Field 0 name length
        0x66, 0x69, 0x65, 0x6c, 0x64, // Field 0 name
        0x04, // Field 0 type ordinal
    ];
    assert_serialized_to(&obj, &expected_buf);
}

#[test]
pub fn serialize_fn_abi_0() {
    let obj = FnAbi::new("name".to_string(), None, FunctionKind::Action);
    let expected_buf = [
        2, // Function kind: Action
        0, 0, 0, 4, // Name Length
        0x6e, 0x61, 0x6d, 0x65, // Name
        255, 166, 141, 149, 8, // shortname
        0, 0, 0, 0, // Arguments length
    ];
    assert_serialized_to(&obj, &expected_buf);
}

#[test]
fn serialize_function_name_0() {
    let obj = FunctionName::new("my_name".to_string(), None);
    let expected_buf = [
        0, 0, 0, 7, // Name Length
        109, 121, 95, 110, 97, 109, 101, // Name
        166, 159, 212, 251, 6, // shortname
    ];

    assert_serialized_to(&obj, &expected_buf);
}

#[test]
fn serialize_function_name_defined_name() {
    let obj = FunctionName::new("my_name".to_string(), Some(Shortname::from_u32(42)));
    let expected_buf = [
        0, 0, 0, 7, // Name Length
        109, 121, 95, 110, 97, 109, 101, // Name
        42,  // shortname
    ];

    assert_serialized_to(&obj, &expected_buf);
}
