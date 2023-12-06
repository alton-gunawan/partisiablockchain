use pbc_contract_common::address::{Address, AddressType};
use pbc_contract_common::sorted_vec_map::SortedVecMap;
use std::collections::HashMap;

const EXAMPLE_ADDRESS: Address = Address {
    address_type: AddressType::PublicContract,
    identifier: [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0x55, 0xAA, 0xDD, 0xFF,
    ],
};

#[test]
pub fn upper_hex() {
    assert_eq!(
        format!("{:X}", EXAMPLE_ADDRESS),
        "02000102030405060708090A0B0C0D0E0F55AADDFF"
    );
}

#[test]
pub fn lower_hex() {
    assert_eq!(
        format!("{:x}", EXAMPLE_ADDRESS),
        "02000102030405060708090a0b0c0d0e0f55aaddff"
    );
}

#[test]
pub fn display() {
    assert_eq!(
        format!("{}", EXAMPLE_ADDRESS),
        format!("{:X}", EXAMPLE_ADDRESS)
    );
}

#[test]
pub fn debug() {
    assert_eq!(
        format!("{:?}", EXAMPLE_ADDRESS),
        "Address { address_type: PublicContract, identifier: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 85, 170, 221, 255] }"
    );
}

#[test]
pub fn clone_address() {
    let addr: Address = EXAMPLE_ADDRESS;
    assert_eq!(addr, addr.clone());
}

#[test]
pub fn clone_address_type() {
    let addr: AddressType = AddressType::Account;
    assert_eq!(addr, addr.clone());
}

#[test]
#[allow(clippy::nonminimal_bool)]
pub fn partial_ord() {
    let addr: Address = EXAMPLE_ADDRESS;
    assert!(addr <= addr);
    assert!(addr >= addr);
    assert!(!(addr < addr));
    assert!(!(addr > addr));
}

#[test]
pub fn sortedvecmap() {
    let mut map = SortedVecMap::new();
    map.insert(EXAMPLE_ADDRESS, "Hello World");
    map.insert(EXAMPLE_ADDRESS, "Hello World 2");
    assert_eq!(map.get(&EXAMPLE_ADDRESS), Some(&"Hello World 2"));
}

#[test]
pub fn hashmap() {
    let mut map = HashMap::new();
    map.insert(EXAMPLE_ADDRESS, "Hello World");
    map.insert(EXAMPLE_ADDRESS, "Hello World 2");
    assert_eq!(map.get(&EXAMPLE_ADDRESS), Some(&"Hello World 2"));
}
