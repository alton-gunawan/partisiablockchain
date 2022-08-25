use pbc_traits::ReadWriteRPC;

use crate::address::{Address, AddressType};
use crate::events::Interaction;

fn to_bytes<T: ReadWriteRPC>(val: T) -> Vec<u8> {
    let mut vec = Vec::new();
    val.rpc_write_to(&mut vec).unwrap();
    vec
}

const TEST_ADDRESS: [u8; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

#[test]
pub fn single_interaction_with_cost() {
    let interaction = Interaction {
        dest: Address {
            address_type: AddressType::PublicContract,
            identifier: TEST_ADDRESS,
        },
        cost: Some(42),
        from_original_sender: false,
        payload: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };

    let vec1 = to_bytes(interaction);
    assert_eq!(
        vec1,
        vec![
            2, // Address type
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, // Address ident
            0, 0, 0, 8, // Payload len
            1, 2, 3, 4, 5, 6, 7, 8, // payload
            0, // Send from contract
            1, // Cost exists
            0, 0, 0, 0, 0, 0, 0, 42, // Cost is 42
        ]
    );
}

#[test]
pub fn single_interaction_with_no_cost() {
    let interaction = Interaction {
        dest: Address {
            address_type: AddressType::PublicContract,
            identifier: TEST_ADDRESS,
        },
        cost: None,
        from_original_sender: true,
        payload: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };

    let vec1 = to_bytes(interaction);
    assert_eq!(
        vec1,
        vec![
            2, // Address type
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, // Address ident
            0, 0, 0, 8, // Payload len
            1, 2, 3, 4, 5, 6, 7, 8, // payload
            1, // Send from sender
            0, // Cost does not exist
        ]
    );
}
