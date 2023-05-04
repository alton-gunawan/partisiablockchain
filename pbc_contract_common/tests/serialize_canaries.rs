use std::fmt::Debug;

use pbc_contract_common::address::AddressType;
use pbc_contract_common::{test_examples, U256};
use pbc_traits::{ReadRPC, ReadWriteState, WriteRPC};

fn canary_rpc_noeq<T: ReadRPC + WriteRPC>(example: &T) -> Vec<u8> {
    // Check serializable
    let mut buf_rpc_1 = Vec::with_capacity(21);
    example
        .rpc_write_to(&mut buf_rpc_1)
        .expect("Write entire value");

    // Check deserializable
    let example_from_rpc = T::rpc_read_from(&mut buf_rpc_1.as_slice());

    // Check that serialization of deserialized is identical to original
    let mut buf_rpc_2 = Vec::with_capacity(21);
    example_from_rpc
        .rpc_write_to(&mut buf_rpc_2)
        .expect("Write entire value");
    assert_eq!(buf_rpc_1, buf_rpc_2);

    // Result!
    buf_rpc_2
}

fn canary_rpc<T: ReadRPC + WriteRPC + PartialEq + Debug>(example: &T) -> Vec<u8> {
    // Check serializable
    let buf_rpc = canary_rpc_noeq(example);

    // Check deserialize identically
    let example_from_rpc = T::rpc_read_from(&mut buf_rpc.as_slice());

    assert_eq!(example, &example_from_rpc);
    buf_rpc
}

fn canary_state<T: ReadWriteState + PartialEq + Debug>(example: &T) -> Vec<u8> {
    // Check serializable identically
    let mut buf_state = Vec::with_capacity(21);
    example
        .state_write_to(&mut buf_state)
        .expect("Write entire value");

    // Check deserialize identically
    let example_from_state = T::state_read_from(&mut buf_state.as_slice());

    assert_eq!(example, &example_from_state);
    buf_state
}

fn canary_rpc_state_eq<T: ReadWriteState + ReadRPC + WriteRPC + PartialEq + Debug>(example: &T) {
    // Check serializable identically
    let buf_rpc = canary_rpc(example);
    let buf_state = canary_state(example);
    assert_eq!(buf_rpc, buf_state);
}

#[test]
fn canary_address_type() {
    let examples = vec![
        AddressType::Account,
        AddressType::SystemContract,
        AddressType::PublicContract,
        AddressType::ZkContract,
    ];
    for example in examples {
        canary_rpc_state_eq(&example);
    }
}

#[test]
fn canary_address() {
    canary_rpc_state_eq(&test_examples::EXAMPLE_ADDRESS_1);
    canary_rpc_state_eq(&test_examples::EXAMPLE_ADDRESS_2);
}

#[test]
fn canary_u256() {
    // Canary rpc
    let example_u256 = test_examples::EXAMPLE_U256;
    let mut buf = Vec::with_capacity(example_u256.bytes.len());
    example_u256
        .rpc_write_to(&mut buf)
        .expect("Write entire value");
    let read_example_u256 = U256::rpc_read_from(&mut buf.as_slice());
    assert_eq!(read_example_u256, example_u256);
    canary_state(&test_examples::EXAMPLE_U256);
}

#[test]
fn canary_hash() {
    canary_rpc_state_eq(&test_examples::EXAMPLE_HASH_1);
    canary_rpc_state_eq(&test_examples::EXAMPLE_HASH_2);
}

#[test]
fn canary_public_key() {
    canary_rpc_state_eq(&test_examples::EXAMPLE_PUBLIC_KEY);
}

#[test]
fn canary_bls_public_key() {
    canary_rpc_state_eq(&test_examples::EXAMPLE_BLS_PUBLIC_KEY);
}

#[test]
fn canary_bls_signature() {
    canary_rpc_state_eq(&test_examples::EXAMPLE_BLS_SIGNATURE);
}

#[test]
fn canary_sorted_vec_map() {
    let buffer = canary_state(&test_examples::example_vec_map());
    let expected: Vec<u8> = vec![
        2, 0, 0, 0, // length of the map
        1, // key
        3, 0, 0, 0, // Length of vec
        2, 0, 0, 0, // String length
        b'm', b'y', // "my"
        4, 0, 0, 0, // String length
        b'n', b'a', b'm', b'e', // "name"
        2, 0, 0, 0, // String length
        b'i', b's', // "is"
        2,    // key
        1, 0, 0, 0, // Length of vec
        4, 0, 0, 0, // String length
        b'w', b'h', b'a', b't', // "what"
    ];

    assert_eq!(buffer, expected);
}

#[test]
fn canary_context() {
    canary_rpc(&test_examples::EXAMPLE_CONTEXT);
}

#[test]
fn canary_callback_context() {
    canary_rpc_noeq(&test_examples::example_callback_context());
}

#[test]
#[cfg(feature = "zk")]
fn canary_secret_var() {
    let examples = vec![
        test_examples::SECRET_VAR_ID_4,
        test_examples::SECRET_VAR_ID_30,
        test_examples::SECRET_VAR_ID_31,
    ];
    for example in examples {
        let buf_rpc = canary_rpc(&example);
        let buf_state = canary_state(&example);
        assert_ne!(buf_rpc, buf_state);
    }
}

#[test]
#[cfg(feature = "zk")]
fn canary_zk_input_def() {
    let examples = vec![
        test_examples::zk_input_def(1),
        test_examples::zk_input_def(2),
        test_examples::zk_input_def(0xFF),
    ];
    for example in examples {
        canary_rpc_noeq(&example);
    }
}

#[test]
#[cfg(feature = "zk")]
fn canary_zk_closed() {
    canary_rpc_noeq(&test_examples::ZK_CLOSED_1);
    canary_rpc_noeq(&test_examples::ZK_CLOSED_2);
    canary_rpc_noeq(&test_examples::zk_closed_open());
}

#[test]
#[cfg(feature = "zk")]
fn canary_zk_state() {
    canary_rpc_noeq(&test_examples::example_zk_state());
}
