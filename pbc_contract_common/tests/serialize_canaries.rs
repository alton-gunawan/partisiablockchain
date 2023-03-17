use std::fmt::Debug;

use pbc_contract_common::address::AddressType;
use pbc_contract_common::test_examples;
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
