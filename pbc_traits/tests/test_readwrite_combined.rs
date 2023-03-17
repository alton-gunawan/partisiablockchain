use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::io::Cursor;

use pbc_traits::{ReadRPC, ReadWriteState, WriteRPC};

fn assert_read_write_raw<T: Eq + Debug>(
    expected: T,
    write: fn(&T, &mut Vec<u8>) -> std::io::Result<()>,
    read: fn(&mut Cursor<Vec<u8>>) -> T,
) {
    let mut out: Vec<u8> = Vec::new();
    write(&expected, &mut out).unwrap();

    let mut reader = Cursor::new(out);
    let actual = read(&mut reader);
    assert_eq!(actual, expected)
}

fn assert_rpc<T: ReadRPC + WriteRPC + Eq + Debug + Clone>(expected: T) {
    assert_read_write_raw(expected, WriteRPC::rpc_write_to, ReadRPC::rpc_read_from);
}

fn assert_state<T: ReadWriteState + Eq + Debug + Clone>(expected: T) {
    assert_read_write_raw(
        expected,
        ReadWriteState::state_write_to,
        ReadWriteState::state_read_from,
    );
}

fn assert_serializes<T: ReadRPC + WriteRPC + ReadWriteState + Eq + Debug + Clone>(expected: T) {
    assert_rpc(expected.clone());
    assert_state(expected);
}

#[test]
pub fn vectors() {
    let simple = vec![42u64, 43u64];
    assert_serializes(simple.clone());
    assert_serializes(vec![simple.clone(), simple]);
}

#[test]
pub fn option() {
    assert_serializes(None::<String>);
    assert_serializes(Some(vec![42u64, 43u64]));

    let complex = vec![Some(42u64), Some(43u64)];
    assert_serializes(Some(complex.clone()));
    assert_serializes(complex);
}

#[test]
pub fn simple_types() {
    assert_serializes(u8::MIN);
    assert_serializes(u8::MAX);
    assert_serializes(i8::MIN);
    assert_serializes(i8::MAX);

    assert_serializes(u16::MIN);
    assert_serializes(u16::MAX);
    assert_serializes(i16::MIN);
    assert_serializes(i16::MAX);

    assert_serializes(u32::MIN);
    assert_serializes(u32::MAX);
    assert_serializes(i32::MIN);
    assert_serializes(i32::MAX);

    assert_serializes(u64::MIN);
    assert_serializes(u64::MAX);
    assert_serializes(i64::MIN);
    assert_serializes(i64::MAX);

    assert_serializes(u128::MIN);
    assert_serializes(u128::MAX);
    assert_serializes(i128::MIN);
    assert_serializes(i128::MAX);
}

#[test]
pub fn strings() {
    assert_serializes("".to_string());
    assert_serializes("This is a string".to_string());
    assert_serializes("Tæstång unícöde".to_string());
}

#[test]
pub fn btree_set_1() {
    let mut simple: BTreeSet<u64> = BTreeSet::new();
    simple.insert(2u64);

    assert_state(simple.clone());

    let mut complex: BTreeSet<BTreeSet<u64>> = BTreeSet::new();
    complex.insert(simple.clone());

    assert_state(complex.clone());
}

#[test]
pub fn btree_set_2() {
    let mut simple: BTreeSet<u64> = BTreeSet::new();
    simple.insert(2u64);
    simple.insert(42u64);

    assert_state(simple.clone());

    let mut complex: BTreeSet<BTreeSet<u64>> = BTreeSet::new();
    complex.insert(simple.clone());
    complex.insert(simple.clone());

    assert_state(complex.clone());
}

#[test]
pub fn btree_set_n_mod_2() {
    let mut simple: BTreeSet<u64> = BTreeSet::new();
    for i in 1..=10 {
        simple.insert(i as u64);
    }

    assert_state(simple.clone());
}

#[test]
pub fn btree_set_n_not_mod_2() {
    let mut simple: BTreeSet<u64> = BTreeSet::new();
    for i in 1..=11 {
        simple.insert(i as u64);
    }

    assert_state(simple.clone());
}

#[test]
pub fn btree_map() {
    let mut simple: BTreeMap<u64, u64> = BTreeMap::new();
    simple.insert(1u64, 2u64);
    simple.insert(21u64, 42u64);

    assert_state(simple.clone());

    let mut complex: BTreeMap<u64, BTreeMap<u64, u64>> = BTreeMap::new();
    complex.insert(2, simple.clone());
    complex.insert(3, simple.clone());

    assert_state(complex);
}
