//! Test module for builders.

use pbc_contract_common::address::{Address, AddressType, Shortname};
use pbc_contract_common::events::*;

#[test]
fn some_test() {
    let mut e = EventGroup::builder();

    let addr = Address {
        address_type: AddressType::PublicContract,
        identifier: [0; 20],
    };

    e.call(addr, Shortname::from_be_bytes(&[0x33]).unwrap())
        .done();

    e.call(addr, Shortname::from_be_bytes(&[0x79]).unwrap())
        .from_original_sender()
        .argument(99u8)
        .argument(addr)
        .with_cost(100010)
        .done();

    e.build();

    //assert_eq!(derp.events.len(), 2);
    //assert!(derp.callback_payload.is_some());
}
