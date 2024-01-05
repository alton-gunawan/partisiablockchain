#![cfg(feature = "abi")]
#![allow(dead_code)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use create_type_spec_derive::create_type_spec_for_generic;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::abi::generate::{generate_types, LookupTable};
use pbc_contract_common::abi::AbiSerialize;
use pbc_contract_common::abi::NamedTypeSpec;
use pbc_contract_common::sorted_vec_map::SortedVecMap;
use pbc_traits::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

// Structs

#[allow(dead_code)]
#[derive(CreateTypeSpec)]
struct DeriveAbiForMe {
    a: String,
    b: u64,
    c: u8,
    d: Vec<Vec<Vec<Vec<u8>>>>,
}

#[allow(dead_code)]
#[derive(CreateTypeSpec)]
struct Nested {
    derived: DeriveAbiForMe,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, CreateTypeSpec, ReadWriteState)]
struct Inner {
    x: u8,
}

#[derive(Clone, CreateTypeSpec)]
struct Outer {
    #[allow(dead_code)]
    inner: Inner,
}

#[derive(Clone, CreateTypeSpec)]
struct OuterMapKey {
    #[allow(dead_code)]
    inner: SortedVecMap<Inner, String>,
}

#[derive(Clone, CreateTypeSpec)]
struct OuterMapValue {
    #[allow(dead_code)]
    inner: SortedVecMap<String, Inner>,
}

#[derive(Clone, CreateTypeSpec)]
struct OuterVec {
    #[allow(dead_code)]
    inner: Vec<Inner>,
}

#[derive(Clone, CreateTypeSpec)]
struct OuterBTreeSet {
    #[allow(dead_code)]
    inner: BTreeSet<Inner>,
}

#[derive(Clone, CreateTypeSpec)]
struct OuterComposite {
    #[allow(dead_code)]
    inner: Vec<BTreeSet<Vec<SortedVecMap<Inner, String>>>>,
}

#[derive(Clone, CreateTypeSpec)]
struct WithArray {
    #[allow(dead_code)]
    inner: [u8; 50],
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, CreateTypeSpec)]
enum MyRoleEnum<RankT: PartialOrd + PartialEq + Eq + Ord + Clone> {
    #[discriminant(1)]
    Admin {},
    #[discriminant(2)]
    Moderator {},
    #[discriminant(3)]
    User { rank: RankT },
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, CreateTypeSpec)]
struct MiniAccess {} // No access

#[derive(Clone, CreateTypeSpec)]
struct AccessControl<RoleT: Ord + Clone> {
    pub role_mappings: SortedVecMap<RoleT, RoleT>,
}

#[derive(Clone, CreateTypeSpec)]
struct StateWithAccessControl {
    pub access_control: AccessControl<MyRoleEnum<u32>>,
}

// Derive tests

#[test]
fn implemented_create_type_spec_trait() {
    assert_eq!(DeriveAbiForMe::__ty_name(), "DeriveAbiForMe".to_string());

    let mut ordinal: Vec<u8> = Vec::new();
    DeriveAbiForMe::__ty_spec_write(&mut ordinal, &BTreeMap::new());
    assert_eq!(ordinal, vec![0x00, 0xFF]);
}

#[test]
fn derived_for_struct() {
    let lut: BTreeMap<String, u8> = BTreeMap::new();

    let abi: NamedTypeSpec = __abi_for_type_deriveabiforme(&lut)
        .into_iter()
        .next()
        .unwrap();
    assert_eq!(abi.name, "DeriveAbiForMe".to_string());
    assert_eq!(abi.type_spec, vec![0x00, 0xFF]);

    let expected = vec![
        1, // It's a struct
        0, 0, 0, 14, // Length of name
        b'D', b'e', b'r', b'i', b'v', b'e', b'A', b'b', b'i', b'F', b'o', b'r', b'M',
        b'e', // Name
        0, 0, 0, 4, // 4 fields
        0, 0, 0, 1,    // Length of field name
        b'a', // Field name
        11,   // String
        0, 0, 0, 1,    // Length of field name
        b'b', // Field name
        4,    //u64
        0, 0, 0, 1,    // Length of field name
        b'c', // Field name
        1,    //u8
        0, 0, 0, 1,    // Length of field name
        b'd', // Field name
        14, 14, 14, 14, 1, // Vec<Vec<Vec<Vec<u8>>>>
    ];

    assert_abi(&abi, expected);
}

#[test]
fn nested_structs() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();
    lut.insert(DeriveAbiForMe::__ty_identifier(), 42);

    let abi: NamedTypeSpec = __abi_for_type_nested(&lut).into_iter().next().unwrap();
    assert_eq!(abi.name, "Nested".to_string());
    assert_eq!(abi.type_spec, vec![0x00, 0xFF]);

    let expected = vec![
        1, // It's a struct
        0, 0, 0, 6, // Length of name
        b'N', b'e', b's', b't', b'e', b'd', // Name
        0, 0, 0, 1, // 1 field
        0, 0, 0, 7, // Length of field name
        b'd', b'e', b'r', b'i', b'v', b'e', b'd', // Field name
        0, 42, // pointer to DeriveAbiForMe as inserted in lut
    ];

    assert_abi(&abi, expected);
}

#[test]
fn serialize_inner() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();
    lut.insert(Inner::__ty_identifier(), 1);

    let a: NamedTypeSpec = __abi_for_type_inner(&lut).into_iter().next().unwrap();

    let result: Vec<u8> = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 0x05, // Length of name
        73, 110, 110, 101, 114, // "Inner"
        0x00, 0x00, 0x00, 0x01, // 1 field
        0x00, 0x00, 0x00, 0x01, // 1 character name
        120,  // x
        0x01, // u8
    ];
    assert_abi(&a, result);
}

#[test]
fn serialize_outer_with_inner_struct() {
    // Look up table for types
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(Outer::__ty_identifier(), 1);

    let result: NamedTypeSpec = __abi_for_type_outer(&lut).into_iter().next().unwrap();
    let expected = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 0x05, // Length of name
        79, 117, 116, 101, 114, // "Outer"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, len 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x00, 0x02, // Struct at index = 2
    ];
    assert_abi(&result, expected);
}

#[test]
fn serialize_inner_as_key_in_map() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterMapKey::__ty_identifier(), 1);

    let result: NamedTypeSpec = __abi_for_type_outermapkey(&lut).into_iter().next().unwrap();
    let expected = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 11, // Length of name = 11
        79, 117, 116, 101, 114, 77, 97, 112, 75, 101, 121, // The string "OuterMapKey"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, length = 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0f, 0, 2, 0x0b, // SortedVecMap<Struct at index = 2, String>
    ];
    assert_abi(&result, expected);
}

#[test]
fn serialize_inner_as_value_in_map() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterMapValue::__ty_identifier(), 1);

    let result: NamedTypeSpec = __abi_for_type_outermapvalue(&lut)
        .into_iter()
        .next()
        .unwrap();
    let expected = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 13, // Length of name = 13
        0x4f, 0x75, 0x74, 0x65, 0x72, 0x4d, 0x61, 0x70, 0x56, 0x61, 0x6c, 0x75,
        0x65, // The string "OuterMapValue"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, length = 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0f, 0x0b, 0, 2, // SortedVecMap<String, Struct at index = 2>
    ];
    assert_abi(&result, expected);
}

#[test]
fn serialize_inner_in_btreeset() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterBTreeSet::__ty_identifier(), 1);

    let result: NamedTypeSpec = __abi_for_type_outerbtreeset(&lut)
        .into_iter()
        .next()
        .unwrap();
    let expected = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 13, // Length of name = 13
        79, 117, 116, 101, 114, 66, 84, 114, 101, 101, 83, 101,
        116, // The string "OuterBTreeSet"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, length = 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x10, 0, 2, // BTreeSet<Struct at index = 2>
    ];
    assert_abi(&result, expected);
}

#[test]
fn serialize_inner_in_vec() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterVec::__ty_identifier(), 1);

    let result: NamedTypeSpec = __abi_for_type_outervec(&lut).into_iter().next().unwrap();
    let expected = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 8, // Length of name = 8
        79, 117, 116, 101, 114, 86, 101, 99, // The string "OuterVec"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, len 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0e, 0, 2, // Vec<Struct at index = 2>
    ];
    assert_abi(&result, expected);
}

#[test]
fn serialize_inner_in_composite() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(Inner::__ty_identifier(), 2);
    lut.insert(OuterComposite::__ty_identifier(), 1);

    let result: NamedTypeSpec = __abi_for_type_outercomposite(&lut)
        .into_iter()
        .next()
        .unwrap();
    let expected = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 14, // Length of name = 8
        79, 117, 116, 101, 114, 67, 111, 109, 112, 111, 115, 105, 116,
        101, // The string "OuterComposite"
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length, len 5
        0x69, 0x6e, 0x6e, 0x65, 0x72, // "inner"
        0x0e, 0x10, 0x0e, 0x0f, 0x00, 0x02,
        0x0b, // Vec<BTreeSet<Vec<SortedVecMap<Inner, with index = 2, String>>>>
    ];
    assert_abi(&result, expected);
}

#[test]
fn serialize_struct_with_array() {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    lut.insert(WithArray::__ty_identifier(), 1);

    let result: NamedTypeSpec = __abi_for_type_witharray(&lut).into_iter().next().unwrap();
    let expected = vec![
        0x01, // It's a struct
        0x00, 0x00, 0x00, 9, // Length of name
        b'W', b'i', b't', b'h', b'A', b'r', b'r', b'a', b'y', // Name
        0, 0, 0, 1, // 1 field
        0, 0, 0, 5, // field name length
        b'i', b'n', b'n', b'e', b'r', // Field name
        17, 50, // Array with length 50.
    ];
    assert_abi(&result, expected);
}

#[allow(dead_code)]
#[derive(CreateTypeSpec)]
enum EnumItemStruct {
    #[discriminant(0)]
    A { a: u8 },
    #[discriminant(3)]
    B { a: u8, b: u8 },
    #[discriminant(125)]
    C { a: Inner },
}

#[test]
fn serialize_enum() {
    let functions: Vec<LookupTable<Vec<NamedTypeSpec>>> =
        vec![__abi_for_type_enumitemstruct, __abi_for_type_inner];
    let (_, types) = unsafe { generate_types(functions.iter()) };

    let enum_type_spec: NamedTypeSpec = types.into_iter().next().unwrap();

    let expected: Vec<u8> = vec![
        0x02, // It's an enum
        0x00, 0x00, 0x00, 0x0E, // Length of name
        b'E', b'n', b'u', b'm', b'I', b't', b'e', b'm', b'S', b't', b'r', b'u', b'c',
        b't', // Name
        0x00, 0x00, 0x00, 0x03, // 3 variants
        0x00, // discriminant for A
        0x00, 0x01, // pointer to A
        0x03, // discriminant for B
        0x00, 0x02, // pointer to B
        0x7D, // discriminant for C
        0x00, 0x03, // pointer to C
    ];
    assert_abi(&enum_type_spec, expected);
}

#[test]
fn serialize_enum_variants() {
    let functions: Vec<LookupTable<Vec<NamedTypeSpec>>> =
        vec![__abi_for_type_enumitemstruct, __abi_for_type_inner];
    let (_, types) = unsafe { generate_types(functions.iter()) };

    let enum_variant_a = types.get(1).unwrap();
    let expected: Vec<u8> = vec![
        0x01, // It's a struct
        0, 0, 0, 1,    // Length of name
        b'A', // Name
        0, 0, 0, 1, // 1 field
        0, 0, 0, 1,    // field name length
        b'a', // Field name
        0x01, // u8
    ];
    assert_abi(enum_variant_a, expected);

    let enum_variant_b = types.get(2).unwrap();
    let expected: Vec<u8> = vec![
        0x01, // It's a struct
        0, 0, 0, 1,    // Length of name
        b'B', // Name
        0, 0, 0, 2, // 2 fields
        0, 0, 0, 1,    // field name length
        b'a', // Field name
        0x01, // u8
        0, 0, 0, 1,    // field name length
        b'b', // Field name
        0x01, // u8
    ];
    assert_abi(enum_variant_b, expected);

    let enum_variant_c = types.get(3).unwrap();
    let expected: Vec<u8> = vec![
        0x01, // It's a struct
        0, 0, 0, 1,    // Length of name
        b'C', // Name
        0, 0, 0, 1, // 1 field
        0, 0, 0, 1,    // field name length
        b'a', // Field name
        0x00, 0x04, // Pointer to Inner
    ];
    assert_abi(enum_variant_c, expected);
}

#[allow(dead_code)]
#[derive(CreateTypeSpec)]
enum NestedEnum {
    #[discriminant(9)]
    One { a: EnumItemStruct },
}

#[test]
fn serialize_nested_enum() {
    let functions: Vec<LookupTable<Vec<NamedTypeSpec>>> = vec![
        __abi_for_type_nestedenum,
        __abi_for_type_enumitemstruct,
        __abi_for_type_inner,
    ];
    let (_, types) = unsafe { generate_types(functions.iter()) };

    let nested_enum = types.get(0).unwrap();

    let expected: Vec<u8> = vec![
        0x02, // It's an enum
        0, 0, 0, 0x0A, // Length of name
        b'N', b'e', b's', b't', b'e', b'd', b'E', b'n', b'u', b'm', // Name
        0, 0, 0, 1,    // 1 variant
        0x09, // discriminant for One
        0x00, 0x01, // pointer to One
    ];
    assert_abi(nested_enum, expected);

    let one_struct = types.get(1).unwrap();
    let expected: Vec<u8> = vec![
        0x01, // It's a struct
        0, 0, 0, 3, // Length of name
        b'O', b'n', b'e', // Name
        0, 0, 0, 1, // 1 field
        0, 0, 0, 1,    // field name length
        b'a', // Field name
        0x00, 0x02, // Pointer to EnumItemStruct
    ];
    assert_abi(one_struct, expected);

    assert_eq!(types.get(2).unwrap().name, "EnumItemStruct");
}

fn assert_abi<T: AbiSerialize>(obj: &T, expected: Vec<u8>) {
    let mut actual = Vec::new();
    obj.serialize_abi(&mut actual).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn type_names_for_generic() {
    assert_eq!(MyRoleEnum::<u32>::__ty_name(), "MyRoleEnumU32");
    assert_eq!(
        AccessControl::<MiniAccess>::__ty_name(),
        "AccessControlMiniAccess"
    );
    assert_eq!(
        StateWithAccessControl::__ty_name(),
        "StateWithAccessControl"
    );
    assert!(MyRoleEnum::<u32>::__ty_identifier().ends_with("U32"));
    assert_eq!(AccessControl::<MiniAccess>::__ty_identifier().len(), 72);
}

create_type_spec_for_generic! {MyRoleEnum::<u32>}
create_type_spec_for_generic! {AccessControl::<MyRoleEnum<u32>>}

#[test]
fn create_type_spec_for_generics() {
    // Test ABI
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();
    lut.insert(MyRoleEnum::<u32>::__ty_identifier(), 1);
    lut.insert(AccessControl::<MyRoleEnum<u32>>::__ty_identifier(), 2);
    lut.insert(StateWithAccessControl::__ty_identifier(), 3);

    let abi: NamedTypeSpec = __abi_for_type_myroleenum::<u32>(&lut)
        .into_iter()
        .next()
        .unwrap();
    assert_eq!(abi.name, "MyRoleEnumU32".to_string());
    assert_eq!(abi.type_spec, vec![0x00, 1]);

    let abi: NamedTypeSpec = __abi_for_type_accesscontrol::<MyRoleEnum<u32>>(&lut)
        .into_iter()
        .next()
        .unwrap();
    assert_eq!(abi.name, "AccessControlMyRoleEnumU32".to_string());
    assert_eq!(abi.type_spec, vec![0x00, 2]);

    let abi: NamedTypeSpec = __abi_for_type_statewithaccesscontrol(&lut)
        .into_iter()
        .next()
        .unwrap();
    assert_eq!(abi.name, "StateWithAccessControl".to_string());
    assert_eq!(abi.type_spec, vec![0x00, 3]);

    let expected = vec![
        1, // Struct
        0, 0, 0, 22, // Name length
        83, 116, 97, 116, 101, 87, 105, 116, 104, 65, 99, 99, 101, 115, 115, 67, 111, 110, 116,
        114, 111, 108, // StateWithAccessControl
        0, 0, 0, 1, // One field
        0, 0, 0, 14, // Field name
        97, 99, 99, 101, 115, 115, 95, 99, 111, 110, 116, 114, 111, 108, // access_control
        0, 2, // pointer to type inserted in lut
    ];

    assert_abi(&abi, expected);
}
