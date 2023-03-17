extern crate core;

use pbc_contract_common::address::{Address, AddressType, Shortname};
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::ContractResultBuffer;
use pbc_traits::ReadWriteState;
use read_write_state_derive::ReadWriteState;

fn assert_written<T: ReadWriteState>(state: T, events: Vec<EventGroup>, expected_bytes: &[u8]) {
    let mut gotten_buffer = ContractResultBuffer::new();
    gotten_buffer.write_state(state);
    gotten_buffer.write_events(events);
    assert_eq!(gotten_buffer.data, expected_bytes);
}

#[derive(ReadWriteState)]
struct EmptyState {}

#[test]
pub fn empty_result() {
    let state = EmptyState {};
    let events = vec![];
    let expected_bytes = [0, 0, 0, 0];
    assert_written(state, events, &expected_bytes)
}

#[derive(ReadWriteState)]
struct SmallState {
    field: u8,
}

#[derive(ReadWriteState)]
#[repr(C)]
struct LargeState<T: ReadWriteState> {
    field1: SmallState,
    field2: SmallState,
    field3: T,
}

#[test]
pub fn small_state() {
    let state = SmallState { field: 54 };
    let events = vec![];
    let expected_bytes = [
        0, 0, 0, 0, // Empty length
        1, // Section id: State
        0, 0, 0, 1,  // Section len: 1
        54, // State
    ];
    assert_written(state, events, &expected_bytes)
}

#[test]
pub fn large_state() {
    let state = LargeState {
        field1: SmallState { field: 41 },
        field2: SmallState { field: 54 },
        field3: LargeState {
            field1: SmallState { field: 12 },
            field2: SmallState { field: 21 },
            field3: 0x220u32,
        },
    };
    let events = vec![];
    let expected_bytes = [
        0, 0, 0, 0, // Empty length
        1, // Section id: State
        0, 0, 0, 8, // Section len: ?
        41, 54, 12, 21, 32, 2, 0, 0, // State
    ];
    assert_written(state, events, &expected_bytes)
}

const ADDRESS_1: Address = Address {
    address_type: AddressType::ZkContract,
    identifier: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
};

const STATE_AND_EVENTS_SMALL_STATE: SmallState = SmallState { field: 59 };

const STATE_AND_EVENTS_EXPECTED_BYTES: [u8; 65] = [
    0, 0, 0, 0, // Empty length
    // State
    1, // Section id: State
    0, 0, 0, 1,  // Section len: 1
    59, // State data
    // Events
    2, // Section id: Events
    0, 0, 0, 50, // Section len
    0, 0, 0, 1, // #EventGroup
    0, // No callback
    0, // Cost: None
    0, 0, 0, 1, // #Events
    3, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, // Event address
    0, 0, 0, 4, 65, 66, 67, 99, // Event payload
    0,  // From original sender?: no
    1, 0, 0, 0, 0, 0, 0, 0, 0x20, // cost: Some(0x20)
    0,    // No return data
];

#[test]
#[allow(deprecated)]
#[should_panic(expected = "Sending events from original sender is not supported")]
pub fn using_from_original_panics() {
    let mut e = EventGroup::builder();

    let _ = e
        .call(ADDRESS_1, Shortname::from_be_bytes(&[65]).unwrap())
        .from_original_sender();
}

#[test]
pub fn state_and_events_builder() {
    let mut e = EventGroup::builder();
    e.call(ADDRESS_1, Shortname::from_be_bytes(&[65]).unwrap())
        .argument(66u8)
        .argument(67u8)
        .argument(99u8)
        .with_cost(0x20)
        .done();

    let events = vec![e.build()];
    assert_written(
        STATE_AND_EVENTS_SMALL_STATE,
        events,
        &STATE_AND_EVENTS_EXPECTED_BYTES,
    )
}
