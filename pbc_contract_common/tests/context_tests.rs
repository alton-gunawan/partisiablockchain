use pbc_contract_common::context::*;
use pbc_contract_common::test_examples::EXAMPLE_CONTEXT;
use pbc_traits::WriteRPC;

#[test]
fn debug() {
    assert_eq!(format!("{:?}", EXAMPLE_CONTEXT), "ContractContext { contract_address: Address { address_type: PublicContract, identifier: [2, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2] }, sender: Address { address_type: Account, identifier: [29, 3, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2] }, block_time: 53, block_production_time: 53, current_transaction: Hash { bytes: [0, 1, 23, 213, 124, 23, 3, 1, 23, 12, 31, 23, 123, 24, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2] }, original_transaction: Hash { bytes: [124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2] } }");
}

#[test]
fn test_get_return_data() {
    let mut e = ExecutionResult {
        succeeded: false,
        return_data: vec![],
    };
    42u64.rpc_write_to(&mut e.return_data).unwrap();
    let return_data_u64 = e.get_return_data::<u64>();
    assert_eq!(return_data_u64, 42);

    let mut e = ExecutionResult {
        succeeded: false,
        return_data: vec![],
    };
    String::from("Hello")
        .rpc_write_to(&mut e.return_data)
        .unwrap();
    let return_data_string = e.get_return_data::<String>();
    assert_eq!(return_data_string, "Hello");

    let mut e = ExecutionResult {
        succeeded: false,
        return_data: vec![],
    };
    true.rpc_write_to(&mut e.return_data).unwrap();
    let return_data_bool = e.get_return_data::<bool>();
    assert!(return_data_bool)
}
