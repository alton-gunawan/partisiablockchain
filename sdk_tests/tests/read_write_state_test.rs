#![allow(dead_code)]
use std::fmt::Debug;

use core::cmp::Ordering;
use create_type_spec_derive::{create_type_spec_for_generic, CreateTypeSpec};
use pbc_contract_common::address::{Address, AddressType};
use pbc_contract_common::avl_tree_map::AvlTreeMap;
use pbc_contract_common::signature::Signature;
use pbc_contract_common::sorted_vec_map::{SortedVec, SortedVecMap, SortedVecSet};
use pbc_traits::ReadWriteState;
use pbc_zk::SecretBinary;
use read_write_state_derive::ReadWriteState;
use std::collections::VecDeque;

use proptest::proptest;

// Test Utility

fn read_write_state_roundtrip<T: ReadWriteState>(struct_1: &T, byte_repr: &[u8]) -> T {
    let mut buf: Vec<u8> = Vec::new();
    struct_1.state_write_to(&mut buf).unwrap();
    assert_eq!(&buf, byte_repr);

    let mut buf: Vec<u8> = Vec::new();
    struct_1.state_write_to(&mut buf).unwrap();

    let mut ctx_reader = std::io::Cursor::new(buf);
    T::state_read_from(&mut ctx_reader)
}

fn read_write_state_roundtrip_with_eq<T: ReadWriteState + Debug + Eq>(
    struct_1: &T,
    byte_repr: &[u8],
) -> T {
    let struct_2 = read_write_state_roundtrip(struct_1, byte_repr);
    assert_eq!(format!("{:?}", struct_1), format!("{:?}", struct_2));
    assert_eq!(*struct_1, struct_2);
    struct_2
}

fn read_write_secret_roundtrip<T: SecretBinary>(struct_1: &T, byte_repr: &[u8]) -> T {
    let mut buf: Vec<u8> = Vec::new();
    struct_1.secret_write_to(&mut buf).unwrap();
    assert_eq!(&buf, byte_repr);

    let mut buf: Vec<u8> = Vec::new();
    struct_1.secret_write_to(&mut buf).unwrap();

    let mut ctx_reader = std::io::Cursor::new(buf);
    T::secret_read_from(&mut ctx_reader)
}

fn read_write_secret_roundtrip_with_eq<T: SecretBinary + Debug + Eq>(
    struct_1: &T,
    byte_repr: &[u8],
) -> T {
    let struct_2 = read_write_secret_roundtrip(struct_1, byte_repr);
    assert_eq!(format!("{:?}", struct_1), format!("{:?}", struct_2));
    assert_eq!(*struct_1, struct_2);
    struct_2
}

// Structures

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct EmptyStruct {}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct SimpleStruct {
    a: u8,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct ComplexStruct {
    a: SimpleStruct,
    b: SimpleStruct,
    c: u16,
}

#[derive(ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct StructWithVec {
    ls: Vec<SimpleStruct>,
    b: u32,
}

#[derive(ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct StructWithEmptyStructVec {
    ls: Vec<EmptyStruct>,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct StructWithPadding {
    a: u8,
    b: u16,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct VecStructWithPadding {
    ls: Vec<StructWithPadding>,
}

#[derive(Eq, PartialEq, ReadWriteState, Debug)]
#[repr(C)]
struct VecDequeStructWithPadding {
    ls: VecDeque<StructWithPadding>,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C, align(4))]
struct StructWithAlignment {
    b: u16,
    a: u8,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C, align(2))]
struct StructWithSizeLargerThanAlignment {
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C, align(4))]
struct StructWithSizeSmallerThanAlignment {
    v1: u8,
    v2: u8,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct VecStructWithSizeLargerThanAlignment {
    ls: Vec<StructWithSizeLargerThanAlignment>,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct VecStructWithSizeSmallerThanAlignment {
    ls: Vec<StructWithSizeSmallerThanAlignment>,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
struct VecStructWithAlignment {
    ls: Vec<StructWithAlignment>,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct StructWithOption {
    a: Option<ComplexStruct>,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct StructWithByteArray {
    a: [u8; 5],
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct VecWithByteArray {
    ls: Vec<StructWithByteArray>,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct StructWithLargeByteArray {
    a: [u8; 100],
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct NestedStruct {
    a: StructWithByteArray,
    b: SimpleStruct,
    c: SimpleStruct,
    d: SimpleStruct,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct VecNestedStruct {
    ls: Vec<NestedStruct>,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct VecWithAddresses {
    sender: Address,
    recipients: Vec<Address>,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
#[repr(C)]
struct VecDequeWithAddresses {
    sender: Address,
    recipients: VecDeque<Address>,
}

#[derive(Eq, PartialEq, ReadWriteState, Debug)]
#[repr(C)]
struct AddressTuple {
    sender: Address,
    recipient: Address,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, Debug)]
#[repr(C)]
struct Tuple1<T> {
    v: T,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct Tuple2<V1, V2> {
    v1: V1,
    v2: V2,
}

create_type_spec_for_generic! {Tuple2<u8, u8>}
create_type_spec_for_generic! {Tuple2<[u8; 5], [u8; 3]>}
create_type_spec_for_generic! {Tuple2<u8, u16>}
create_type_spec_for_generic! {Tuple2<u32, Tuple2<u8, u16>>}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct Tuple6<V1, V2, V3, V4, V5, V6> {
    v1: V1,
    v2: V2,
    v3: V3,
    v4: V4,
    v5: V5,
    v6: V6,
}

create_type_spec_for_generic! {Tuple6<u8, i8, u16, i16, u32, i32>}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, CreateTypeSpec, Debug)]
#[repr(C)]
struct WeirdState {
    f1: Tuple2<u8, u8>,
    f2: Tuple2<[u8; 5], [u8; 3]>,
    f3: Tuple2<u8, u16>,
    f4: Tuple2<u32, Tuple2<u8, u16>>,
    f5: Tuple6<u8, i8, u16, i16, u32, i32>,
}

#[derive(Eq, PartialEq, ReadWriteState, SecretBinary, Debug)]
#[repr(C)]
struct Range<T>
where
    T: Ord,
{
    low: T,
    high: T,
}

#[derive(Eq, PartialEq, ReadWriteState, Debug)]
#[repr(u8)]
enum StatusEnum {
    Waiting = 0,
    Running = 1,
    Done = 2,
}

#[derive(Eq, PartialEq, ReadWriteState, Debug)]
#[repr(C)]
struct VecWithStatusEnums {
    ls: Vec<StatusEnum>,
}

const DISCR_VAR0: u8 = 0;
const DISCR_VAR1: u8 = 1;
const DISCR_VAR2: u8 = 2;

#[derive(Eq, PartialEq, ReadWriteState, Debug)]
#[repr(u8)]
enum EnumWithConstantDiscriminants {
    Var0 = DISCR_VAR0,
    Var1 = DISCR_VAR1,
    Var2 = DISCR_VAR2,
}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
enum EnumItemStruct {
    #[discriminant(0)]
    A { a: bool },
    #[discriminant(3)]
    B { a: u8, b: u8 },
    #[discriminant(125)]
    C { a: SimpleStruct },
}

#[derive(ReadWriteState, Debug, CreateTypeSpec)]
struct StateWithAvlMap {
    map: AvlTreeMap<u32, String>,
}

#[derive(ReadWriteState, Debug, CreateTypeSpec, Clone)]
struct StateWithSortedVecMap {
    map: SortedVecMap<u32, String>,
}

#[derive(ReadWriteState, Debug, CreateTypeSpec, Clone)]
struct StateWithSortedVecSet {
    map: SortedVecSet<String>,
}

#[derive(ReadWriteState, Debug, CreateTypeSpec, Clone)]
struct StateWithSortedVec {
    map: SortedVec<String>,
}

#[derive(Eq, PartialEq, ReadWriteState, Debug, Clone)]
#[repr(C)]
struct SignatureMapState {
    signatures: SortedVecMap<Signature, String>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, ReadWriteState, CreateTypeSpec, Debug)]
enum RoleEnum {
    #[discriminant(1)]
    Admin {},
    #[discriminant(2)]
    Moderator {},
    #[discriminant(3)]
    User {},
}

create_type_spec_for_generic! {AccessControl<RoleEnum>}
create_type_spec_for_generic! {AccessControl<u32>}

#[derive(Eq, PartialEq, ReadWriteState, CreateTypeSpec, Debug)]
pub struct AccessControl<RoleEnum: Ord + Clone + Eq + PartialEq> {
    pub roles_admin: SortedVecMap<RoleEnum, RoleEnum>,
}

// Tests

proptest! {
    #[test]
    fn serialize_bool(state: bool) {
        let expected_bytes = [if state { 1 } else { 0 }];
        read_write_state_roundtrip_with_eq(&state, &expected_bytes);
        read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
    }
}

proptest! {
    #[test]
    fn serialize_u8(state: u8) {
        let expected_bytes = [state];
        read_write_state_roundtrip_with_eq(&state, &expected_bytes);
        read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
    }
}

proptest! {
    #[test]
    fn serialize_simple_struct(a: u8) {
        let state = SimpleStruct { a };
        let expected_bytes = [a];
        read_write_state_roundtrip_with_eq(&state, &expected_bytes);
        read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
    }
}

proptest! {
    #[test]
    fn serialize_tuple2_u8_u8(v1: u8, v2: u8) {
        let state = Tuple2 { v1, v2 };
        let expected_bytes = [v1, v2];
        read_write_state_roundtrip_with_eq(&state, &expected_bytes);
        read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
    }
}

proptest! {
    #[test]
    fn serialize_tuple2_byte_arrays(v1: [u8; 5], v2: [u8; 3]) {
        let state = Tuple2 { v1, v2 };
        let expected_bytes = [
             v1[0],
             v1[1],
             v1[2],
             v1[3],
             v1[4],
             v2[0],
             v2[1],
             v2[2],
        ];
        read_write_state_roundtrip_with_eq(&state, &expected_bytes);
        read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
    }
}

#[test]
pub fn serialize_complex_struct() {
    let simple_struct_1 = SimpleStruct { a: 42 };
    let simple_struct_2 = SimpleStruct { a: 43 };

    let state = ComplexStruct {
        a: simple_struct_1,
        b: simple_struct_2,
        c: 15432,
    };

    let expected_bytes = [
        42, // a.a
        43, // b.a
        72, 60, // c
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_struct_with_vec() {
    let simple_struct_1 = SimpleStruct { a: 42 };
    let simple_struct_2 = SimpleStruct { a: 43 };

    let state = StructWithVec {
        ls: vec![simple_struct_1, simple_struct_2],
        b: 15432,
    };

    let expected_bytes = [
        2, 0, 0, 0,  // ls.len()
        42, // ls[0]
        43, // ls[1]
        72, 60, 0, 0, // b
    ];

    let state2 = read_write_state_roundtrip(&state, &expected_bytes);
    assert_eq!(state.ls.len(), state2.ls.len());
    assert_eq!(state.ls[0], state2.ls[0]);
    assert_eq!(state.ls[1], state2.ls[1]);
    assert_eq!(state.b, state2.b);
}

#[test]
pub fn serialize_empty_struct() {
    let state = EmptyStruct {};
    let expected_bytes = [];
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_struct_vec_of_empty_struct() {
    let state = StructWithEmptyStructVec {
        ls: vec![
            EmptyStruct {},
            EmptyStruct {},
            EmptyStruct {},
            EmptyStruct {},
        ],
    };
    let expected_bytes = [
        4, 0, 0, 0, // ls.len()
    ];
    let state2 = read_write_state_roundtrip(&state, &expected_bytes);
    assert_eq!(state.ls.len(), state2.ls.len());
}

#[test]
pub fn serialize_struct_with_padding() {
    let state = StructWithPadding { a: 0x42, b: 0x1234 };
    let expected_bytes = [
        0x42, // a
        0x34, 0x12, // b
    ];

    assert_eq!(std::mem::size_of_val(&state), 4);
    assert_eq!(std::mem::align_of_val(&state), 2);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_vec_struct_with_padding() {
    let state = VecStructWithPadding {
        ls: vec![
            StructWithPadding { a: 0x42, b: 0x1234 },
            StructWithPadding { a: 0x43, b: 0x1333 },
            StructWithPadding { a: 0x44, b: 0x1432 },
            StructWithPadding { a: 0x45, b: 0x1531 },
        ],
    };
    let buffer = [
        0x04, 0x00, 0x00, 0x00, // Vec length
        0x42, 0x34, 0x12, // Element 1
        0x43, 0x33, 0x13, // Element 2
        0x44, 0x32, 0x14, // Element 3
        0x45, 0x31, 0x15, // Element 4
    ];
    let state2 = read_write_state_roundtrip(&state, &buffer);
    assert_eq!(state.ls.len(), state2.ls.len());
}

#[test]
pub fn serialize_state_with_avltree() {
    let _unused1: AvlTreeMap<u32, u32> = AvlTreeMap::default();
    let _unused2: AvlTreeMap<u32, u32> = AvlTreeMap::default();
    let _unused3: AvlTreeMap<u32, u32> = AvlTreeMap::default();
    let _unused4: AvlTreeMap<u32, u32> = AvlTreeMap::new();
    let mut state = StateWithAvlMap {
        map: AvlTreeMap::new(),
    };
    state.map.insert(1, "Hello, world!".to_string());

    let buffer = [
        0x04, 0x00, 0x00, 0x00, // Tree id
    ];
    let state2 = read_write_state_roundtrip(&state, &buffer);
    assert_eq!(state.map.len(), state2.map.len());
}

#[test]
pub fn signature_map_state() {
    let mut signatures = SortedVecMap::default();
    let sig1 = Signature {
        recovery_id: 1,
        value_r: [0; 32],
        value_s: [1; 32],
    };
    let sig2 = Signature {
        recovery_id: 5,
        value_r: [2; 32],
        value_s: [3; 32],
    };
    assert_eq!(sig1.partial_cmp(&sig2), Some(Ordering::Less));

    signatures.insert(sig1, "Hello, world 1!".to_string());
    signatures.insert(sig2, "Hello, world 2!".to_string());

    let state = SignatureMapState { signatures };

    let buffer = [
        2, 0, 0, 0, // Len
        // Signature 1
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, // Value 1
        15, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 49, 33,
        // Signature 1
        5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, // Value 1
        15, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 50, 33,
    ];
    let state2 = read_write_state_roundtrip_with_eq(&state, &buffer);
    assert_eq!(state2.clone(), state2);
    assert_eq!(format!("{:?}", state2), "SignatureMapState { signatures: SortedVecMap { entries: [Entry { key: Signature { recovery_id: 1, value_r: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], value_s: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] }, value: \"Hello, world 1!\" }, Entry { key: Signature { recovery_id: 5, value_r: [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], value_s: [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3] }, value: \"Hello, world 2!\" }] } }");
}

#[test]
pub fn serialize_state_with_sortedvecmap() {
    let mut state = StateWithSortedVecMap {
        map: SortedVecMap::default(),
    };
    state.map.insert(1, "Hello, world!".to_string());

    let buffer = [
        1, 0, 0, 0, 1, 0, 0, 0, 13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108,
        100, 33,
    ];
    let state2 = read_write_state_roundtrip(&state, &buffer);
    assert_eq!(state.map.len(), state2.map.len());
    assert_eq!(state.map.clone(), state2.map.clone());
    assert_eq!(
        format!("{:?}", state.map),
        "SortedVecMap { entries: [Entry { key: 1, value: \"Hello, world!\" }] }"
    );
}

#[test]
pub fn serialize_state_with_sortedvec() {
    let mut state = StateWithSortedVec {
        map: SortedVec::default(),
    };
    state.map.insert("Hello, world!".to_string());

    let buffer = [
        1, 0, 0, 0, 13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33,
    ];
    let state2 = read_write_state_roundtrip(&state.clone(), &buffer);
    assert_eq!(state.map.len(), state2.map.len());
    assert_eq!(
        format!("{:?}", state.map),
        "SortedVec { elements: SortedVecSet { elements: [\"Hello, world!\"] } }"
    );
}

#[test]
pub fn serialize_state_with_sortedvecset() {
    let mut state = StateWithSortedVecSet {
        map: SortedVecSet::default(),
    };
    state.map.insert("Hello, world!".to_string());

    let buffer = [
        1, 0, 0, 0, 13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33,
    ];
    let state2 = read_write_state_roundtrip(&state.clone(), &buffer);
    assert_eq!(state.map.len(), state2.map.len());
    assert_eq!(
        format!("{:?}", state.map),
        "SortedVecSet { elements: [\"Hello, world!\"] }"
    );
}

#[test]
pub fn serialize_state_with_vec_deque() {
    let state = VecDequeStructWithPadding {
        ls: vec![
            StructWithPadding { a: 0x42, b: 0x1234 },
            StructWithPadding { a: 0x43, b: 0x1333 },
            StructWithPadding { a: 0x44, b: 0x1432 },
            StructWithPadding { a: 0x45, b: 0x1531 },
        ]
        .into_iter()
        .collect(),
    };
    let buffer = [
        0x04, 0x00, 0x00, 0x00, // Vec length
        0x42, 0x34, 0x12, // Element 1
        0x43, 0x33, 0x13, // Element 2
        0x44, 0x32, 0x14, // Element 3
        0x45, 0x31, 0x15, // Element 4
    ];
    read_write_state_roundtrip_with_eq(&state, &buffer);
}

#[test]
pub fn serialize_vecdeque_with_addresses() {
    let state = VecDequeWithAddresses {
        sender: ADDR_1,
        recipients: vec![ADDR_2, ADDR_3, ADDR_4].into_iter().collect(),
    };
    let expected_bytes = [
        2, // sender.address_type
        0, 1, 2, 3, 4, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 6, 7, 8, 9, // sender.identifier
        3, 0, 0, 0, // recipients.len()
        1, // recipients[0].address_type
        0, 1, 2, 3, 4, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 5, 7, 7, 8, 9, // recipients[0].ide
        0, // recipients[1].address_type
        0, 1, 2, 3, 4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 5, 8, 7, 8, 9, // recipients[1].ide
        3, // recipients[2].address_type
        0, 1, 2, 3, 4, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 5, 9, 7, 8, 9, // recipients[2].ide
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_struct_with_alignment() {
    let state = StructWithAlignment { b: 0x1234, a: 0x42 };
    let expected_bytes = [
        0x34, 0x12, // b
        0x42, // a
    ];
    assert_eq!(std::mem::size_of_val(&state), 4);
    assert_eq!(std::mem::align_of_val(&state), 4);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_struct_with_size_larger_than_alignment() {
    let state = StructWithSizeLargerThanAlignment {
        v1: 2,
        v2: 3,
        v3: 5,
        v4: 7,
    };

    let expected_bytes = [
        2, // v1
        3, // v2
        5, // v3
        7, // v4
    ];

    assert_eq!(std::mem::size_of_val(&state), 4);
    assert_eq!(std::mem::align_of_val(&state), 2);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_struct_with_size_smaller_than_alignment() {
    let state = StructWithSizeSmallerThanAlignment { v1: 5, v2: 13 };

    let expected_bytes = [
        5,  // v1
        13, // v2
    ];

    assert_eq!(std::mem::size_of_val(&state), 4);
    assert_eq!(std::mem::align_of_val(&state), 4);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_vec_struct_with_size_larger_than_alignment() {
    let state = VecStructWithSizeLargerThanAlignment {
        ls: vec![
            StructWithSizeLargerThanAlignment {
                v1: 2,
                v2: 3,
                v3: 5,
                v4: 7,
            },
            StructWithSizeLargerThanAlignment {
                v1: 11,
                v2: 13,
                v3: 17,
                v4: 19,
            },
            StructWithSizeLargerThanAlignment {
                v1: 23,
                v2: 29,
                v3: 31,
                v4: 37,
            },
        ],
    };

    let expected_bytes = [
        3, 0, 0, 0, // ls.len()
        2, 3, 5, 7, // ls[0]
        11, 13, 17, 19, // ls[0]
        23, 29, 31, 37, // ls[0]
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_vec_struct_with_size_smaller_than_alignment() {
    let state = VecStructWithSizeSmallerThanAlignment {
        ls: vec![
            StructWithSizeSmallerThanAlignment { v1: 2, v2: 3 },
            StructWithSizeSmallerThanAlignment { v1: 11, v2: 13 },
            StructWithSizeSmallerThanAlignment { v1: 23, v2: 29 },
        ],
    };

    let expected_bytes = [
        3, 0, 0, 0, // ls.len()
        2, 3, // ls[0]
        11, 13, // ls[1]
        23, 29, // ls[2]
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_vec_struct_with_alignment() {
    let state = VecStructWithAlignment {
        ls: vec![
            StructWithAlignment { a: 0x42, b: 0x1234 },
            StructWithAlignment { a: 0x43, b: 0x1333 },
            StructWithAlignment { a: 0x44, b: 0x1432 },
            StructWithAlignment { a: 0x45, b: 0x1531 },
        ],
    };
    let buffer = [
        0x04, 0x00, 0x00, 0x00, // Vec length
        0x34, 0x12, 0x42, // Element 1
        0x33, 0x13, 0x43, // Element 2
        0x32, 0x14, 0x44, // Element 3
        0x31, 0x15, 0x45, // Element 4
    ];
    let state2 = read_write_state_roundtrip(&state, &buffer);
    assert_eq!(state.ls.len(), state2.ls.len());
}

#[test]
pub fn serialize_struct_with_option() {
    let simple_struct_1 = SimpleStruct { a: 42 };
    let simple_struct_2 = SimpleStruct { a: 43 };

    let complex = ComplexStruct {
        a: simple_struct_1,
        b: simple_struct_2,
        c: 15432,
    };

    let state1 = StructWithOption { a: Some(complex) };
    let state2 = StructWithOption { a: None };

    let expected_bytes1 = [
        1,  // a.discriminant
        42, // a.some.a
        43, // a.some.b
        72, 60, // a.some.c
    ];

    read_write_state_roundtrip_with_eq(&state1, &expected_bytes1);
    read_write_state_roundtrip_with_eq(&state2, &[0]);
}

#[test]
pub fn serialize_struct_with_bytearray() {
    let state = StructWithByteArray { a: [1, 2, 3, 4, 5] };
    let expected_bytes = [
        1, 2, 3, 4, 5, // a
    ];

    assert_eq!(std::mem::size_of_val(&state), 5);
    assert_eq!(std::mem::align_of_val(&state), 1);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_struct_with_bytearray_large() {
    let state = StructWithLargeByteArray { a: [1; 100] };
    let expected_bytes = [1; 100];

    assert_eq!(std::mem::size_of_val(&state), 100);
    assert_eq!(std::mem::align_of_val(&state), 1);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_struct_with_bytearray_zeroes() {
    let state = StructWithByteArray { a: [0, 0, 0, 0, 0] };
    let expected_bytes = [
        0, 0, 0, 0, 0, // a
    ];

    assert_eq!(std::mem::size_of_val(&state), 5);
    assert_eq!(std::mem::align_of_val(&state), 1);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_vec_with_bytearray() {
    let state = VecWithByteArray {
        ls: vec![
            StructWithByteArray { a: [1, 2, 3, 4, 5] },
            StructWithByteArray { a: [9, 8, 7, 6, 5] },
        ],
    };
    let expected_bytes = [
        2, 0, 0, 0, // ls.len()
        1, 2, 3, 4, 5, // ls[0].a
        9, 8, 7, 6, 5, // ls[1].a
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_nested_struct() {
    let state = NestedStruct {
        a: StructWithByteArray {
            a: [1, 2, 4, 8, 16],
        },
        b: SimpleStruct { a: 32 },
        c: SimpleStruct { a: 64 },
        d: SimpleStruct { a: 128 },
    };
    let expected_bytes = [
        1, 2, 4, 8, 16,  // a
        32,  // b
        64,  // c
        128, // d
    ];

    assert_eq!(std::mem::size_of_val(&state), 8);
    assert_eq!(std::mem::align_of_val(&state), 1);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_vec_nested_struct() {
    let struct1 = NestedStruct {
        a: StructWithByteArray {
            a: [1, 2, 4, 8, 16],
        },
        b: SimpleStruct { a: 32 },
        c: SimpleStruct { a: 64 },
        d: SimpleStruct { a: 128 },
    };
    let struct2 = NestedStruct {
        a: StructWithByteArray {
            a: [0, 1, 3, 7, 15],
        },
        b: SimpleStruct { a: 31 },
        c: SimpleStruct { a: 63 },
        d: SimpleStruct { a: 127 },
    };
    let state = VecNestedStruct {
        ls: vec![struct1, struct2],
    };
    let expected_bytes = [
        2, 0, 0, 0, // ls.len()
        1, 2, 4, 8, 16, 32, 64, 128, // ls[0]
        0, 1, 3, 7, 15, 31, 63, 127, // ls[1]
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

const ADDR_1: Address = Address {
    address_type: AddressType::PublicContract,
    identifier: [0, 1, 2, 3, 4, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 6, 7, 8, 9],
};
const ADDR_2: Address = Address {
    address_type: AddressType::SystemContract,
    identifier: [0, 1, 2, 3, 4, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 5, 7, 7, 8, 9],
};
const ADDR_3: Address = Address {
    address_type: AddressType::Account,
    identifier: [0, 1, 2, 3, 4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 5, 8, 7, 8, 9],
};
const ADDR_4: Address = Address {
    address_type: AddressType::ZkContract,
    identifier: [0, 1, 2, 3, 4, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 5, 9, 7, 8, 9],
};

#[test]
pub fn serialize_vec_with_addresses() {
    let state = VecWithAddresses {
        sender: ADDR_1,
        recipients: vec![ADDR_2, ADDR_3, ADDR_4],
    };
    let expected_bytes = [
        2, // sender.address_type
        0, 1, 2, 3, 4, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 6, 7, 8, 9, // sender.identifier
        3, 0, 0, 0, // recipients.len()
        1, // recipients[0].address_type
        0, 1, 2, 3, 4, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 5, 7, 7, 8, 9, // recipients[0].ide
        0, // recipients[1].address_type
        0, 1, 2, 3, 4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 5, 8, 7, 8, 9, // recipients[1].ide
        3, // recipients[2].address_type
        0, 1, 2, 3, 4, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 5, 9, 7, 8, 9, // recipients[2].ide
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_address_tuple() {
    let state = AddressTuple {
        sender: ADDR_1,
        recipient: ADDR_2,
    };
    let expected_bytes = [
        2, // sender.address_type
        0, 1, 2, 3, 4, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 6, 7, 8, 9, // sender.identifier
        1, // recipient.address_type
        0, 1, 2, 3, 4, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 5, 7, 7, 8, 9, // recipient.identif
    ];

    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_tuple1() {
    let state = Tuple1 {
        v: Tuple1 {
            v: Tuple1 { v: 0x53u32 },
        },
    };

    let expected_bytes = [
        0x53, 0x0, 0x0, 0x0, // v.v.v
    ];

    assert_eq!(std::mem::size_of_val(&state), 4);
    assert_eq!(std::mem::align_of_val(&state), 4);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_tuple2() {
    let state = Tuple2 {
        v1: 0x53u32,
        v2: Tuple2 {
            v1: 0x7u8,
            v2: 0x9u16,
        },
    };

    let expected_bytes = [
        0x53, 0, 0, 0,   // v1
        0x7, // v2.v1
        0x9, 0, // v2.v2
    ];

    assert_eq!(std::mem::size_of_val(&state), 8);
    assert_eq!(std::mem::align_of_val(&state), 4);
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
    read_write_secret_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_status_enum() {
    read_write_state_roundtrip_with_eq(&StatusEnum::Waiting, &[0x00]);
    read_write_state_roundtrip_with_eq(&StatusEnum::Running, &[0x01]);
    read_write_state_roundtrip_with_eq(&StatusEnum::Done, &[0x02]);
}

#[test]
pub fn serialize_status_enum_vec() {
    let state = VecWithStatusEnums {
        ls: vec![
            StatusEnum::Waiting,
            StatusEnum::Done,
            StatusEnum::Waiting,
            StatusEnum::Running,
            StatusEnum::Running,
        ],
    };
    let expected_bytes = [
        0x05, 0x00, 0x00, 0x00, // ls.len()
        0x00, 0x02, 0x00, 0x01, 0x01, // ls[0] to ls[4]
    ];
    read_write_state_roundtrip_with_eq(&state, &expected_bytes);
}

#[test]
pub fn serialize_enum_with_constant_fields() {
    read_write_state_roundtrip_with_eq(&EnumWithConstantDiscriminants::Var0, &[0x00]);
    read_write_state_roundtrip_with_eq(&EnumWithConstantDiscriminants::Var1, &[0x01]);
    read_write_state_roundtrip_with_eq(&EnumWithConstantDiscriminants::Var2, &[0x02]);
}

#[test]
pub fn serialize_enum_item_struct() {
    let my_enum_a = EnumItemStruct::A { a: true };
    let my_enum_b = EnumItemStruct::B { a: 2, b: 3 };
    let my_enum_c = EnumItemStruct::C {
        a: SimpleStruct { a: 0 },
    };
    read_write_state_roundtrip_with_eq(&my_enum_a, &[0x00, 0x01]);
    read_write_state_roundtrip_with_eq(&my_enum_b, &[0x03, 0x02, 0x03]);
    read_write_state_roundtrip_with_eq(&my_enum_c, &[0x7D, 0x00]);
}

macro_rules! assert_serializable_by_copy{
    ($($type:ty)*) => {
        $(
            #[allow(dead_code)]
            const _: () = if !< $type as ReadWriteState > ::SERIALIZABLE_BY_COPY {
                panic!(concat!(stringify!($type), " was not SERIALIZABLE_BY_COPY as was otherwise expected!"));
            };
        )*
    }
}

macro_rules! assert_serializable_by_copy_not{
    ($($type:ty)*) => {
        $(
            #[allow(dead_code)]
            const _: () = if < $type as ReadWriteState > ::SERIALIZABLE_BY_COPY {
                panic!(concat!(stringify!($type), " was SERIALIZABLE_BY_COPY, but this was unexpected!"));
            };
        )*
    }
}

assert_serializable_by_copy!(EmptyStruct);
assert_serializable_by_copy!(SimpleStruct);
assert_serializable_by_copy!(StructWithSizeLargerThanAlignment);
assert_serializable_by_copy!(ComplexStruct);
assert_serializable_by_copy!(StructWithByteArray);
assert_serializable_by_copy!(NestedStruct);

// Address support
assert_serializable_by_copy!(AddressType);
assert_serializable_by_copy!(Address);
assert_serializable_by_copy!(AddressTuple);

// Structs with generics
assert_serializable_by_copy!(Tuple1<u8>);
assert_serializable_by_copy!(Tuple1<Tuple1<u8>>);
assert_serializable_by_copy!(Tuple1<u64>);
assert_serializable_by_copy!(Tuple2<u8, u8>);
assert_serializable_by_copy!(Tuple2<u8, Tuple2<u8, u8>>);
assert_serializable_by_copy!(Tuple2<[u8; 3], Tuple2<[u8; 3], [u8; 2]>>);
assert_serializable_by_copy!(Tuple2<u64, u64>);

// Map entry
assert_serializable_by_copy!(pbc_contract_common::sorted_vec_map::entry::Entry<Address, u8>);

// Enum StatusEnum
assert_serializable_by_copy!(StatusEnum);
assert_serializable_by_copy!(EnumWithConstantDiscriminants);

assert_serializable_by_copy_not!(StructWithSizeSmallerThanAlignment);
assert_serializable_by_copy_not!(StructWithVec);
assert_serializable_by_copy_not!(StructWithEmptyStructVec);
assert_serializable_by_copy_not!(StructWithPadding);
assert_serializable_by_copy_not!(VecStructWithPadding);
assert_serializable_by_copy_not!(StructWithAlignment);
assert_serializable_by_copy_not!(VecStructWithAlignment);
assert_serializable_by_copy_not!(StructWithOption);
assert_serializable_by_copy_not!(VecWithByteArray);
assert_serializable_by_copy_not!(VecNestedStruct);

// Address support
assert_serializable_by_copy_not!(VecWithAddresses);

// EnumItemStruct
assert_serializable_by_copy_not!(EnumItemStruct);
