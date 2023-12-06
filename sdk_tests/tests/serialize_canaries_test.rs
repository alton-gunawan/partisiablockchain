use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::zk::ZkInputDef;
use pbc_traits::WriteRPC;
use pbc_zk::{Sbi32, SecretBinary};
use read_write_state_derive::ReadWriteState;

#[test]
fn canary_zk_input_def_1() {
    let zk_input_def: ZkInputDef<u32, Sbi32> = ZkInputDef::with_metadata(99);

    let mut rpc = Vec::with_capacity(21);
    zk_input_def.rpc_write_to(&mut rpc).unwrap();

    let expected: Vec<u8> = vec![
        0, 0, 0, 1, 0, 0, 0, 32, // Bit lengths
        0,  // Sealed
        99, 0, 0, 0, // Metadata
    ];

    assert_eq!(rpc, expected);
}

#[derive(ReadWriteState, CreateTypeSpec)]
struct MyMetadata {
    a: u32,
    b: u32,
}

#[derive(SecretBinary, CreateTypeSpec)]
struct SecretValue {
    a: Sbi32,
    b: Sbi32,
}

#[test]
fn canary_zk_input_def_2() {
    let zk_input_def: ZkInputDef<MyMetadata, SecretValue> =
        ZkInputDef::with_metadata(MyMetadata { a: 101, b: 202 });

    let mut rpc = Vec::with_capacity(21);
    zk_input_def.rpc_write_to(&mut rpc).unwrap();

    let expected: Vec<u8> = vec![
        0, 0, 0, 1, 0, 0, 0, 64, // Bit lengths
        0,  // Sealed
        101, 0, 0, 0, 202, 0, 0, 0, // Metadata
    ];

    assert_eq!(rpc, expected);
}
