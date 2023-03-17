//! Test module for builders.

use pbc_contract_common::address::{Address, AddressType, Shortname, ShortnameCallback};
use pbc_contract_common::context::ExecutionResult;
use pbc_contract_common::events::*;
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;

const DEFAULT_ADDRESS: Address = Address {
    address_type: AddressType::PublicContract,
    identifier: [0; 20],
};

#[derive(ReadRPC, WriteRPC, PartialEq, Clone, Debug)]
struct SomeStruct {
    string: String,
}

#[test]
fn some_test() {
    let mut e = EventGroup::builder();

    e.call(DEFAULT_ADDRESS, Shortname::from_be_bytes(&[0x33]).unwrap())
        .done();

    e.call(DEFAULT_ADDRESS, Shortname::from_be_bytes(&[0x79]).unwrap())
        .argument(99u8)
        .argument(DEFAULT_ADDRESS)
        .with_cost(100010)
        .done();

    e.build();
}

#[test]
fn should_have_return_data_if_no_callback() {
    let mut e = EventGroup::builder();
    e.return_data(42u64);
    let event_group = e.build();
    assert!(event_group.return_data().is_some());
}

#[test]
fn can_return_struct() {
    let mut e = EventGroup::builder();
    let some_struct = SomeStruct {
        string: "Hello World".to_string(),
    };
    e.return_data(some_struct.clone());
    let event_group = e.build();

    let mut execution_result = ExecutionResult {
        succeeded: false,
        return_data: vec![],
    };
    execution_result.return_data = event_group.return_data().unwrap().data();
    let returned_some_struct = execution_result.get_return_data::<SomeStruct>();
    assert_eq!(returned_some_struct, some_struct);
}

#[test]
#[should_panic(
    expected = "Attempted to build EventGroup with callback but no associated interactions"
)]
fn should_fail_with_callback_and_no_interactions() {
    let mut e = EventGroup::builder();
    let some_shortname = Shortname::from_be_bytes(&[0x79]).unwrap();
    let short_name_callback = ShortnameCallback::new(some_shortname);
    e.with_callback(short_name_callback).done();
    e.build();
}

#[test]
#[should_panic(expected = "Attempted to build EventGroup with both callback and return data")]
fn should_fail_with_callback_and_return_data() {
    let mut e = EventGroup::builder();

    e.call(DEFAULT_ADDRESS, Shortname::from_be_bytes(&[0x33]).unwrap())
        .done();

    let some_shortname = Shortname::from_be_bytes(&[0x79]).unwrap();
    let short_name_callback = ShortnameCallback::new(some_shortname);
    e.with_callback(short_name_callback).done();

    e.return_data(42u64);

    e.build();
}
