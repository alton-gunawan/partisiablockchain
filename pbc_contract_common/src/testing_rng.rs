use crate::address::{Address, AddressType};
use crate::context::ContractContext;

pub trait InstanceGenerator {
    fn generate(rng: &mut Rng) -> Self;
}

impl InstanceGenerator for AddressType {
    fn generate(rng: &mut Rng) -> Self {
        match rng.get_u32() & 0x3 {
            0 => AddressType::Account,
            1 => AddressType::SystemContract,
            2 => AddressType::PublicContract,
            3 => AddressType::ZkContract,
            _ => panic!("Impossible"),
        }
    }
}

impl <const N: usize> InstanceGenerator for [u8; N] {
    fn generate(rng: &mut Rng) -> Self {
        let mut x = [0; N];
        for idx in 0..N {
            x[idx] = rng.get_u32() as u8;
        }
        x
    }
}

impl InstanceGenerator for Address {
    fn generate(rng: &mut Rng) -> Self {
        let address_type = AddressType::generate(rng);
        let identifier = <[u8; 20]>::generate(rng);
        Address {
            address_type, identifier
        }
    }
}

impl InstanceGenerator for ContractContext  {
    fn generate(rng: &mut Rng) -> Self {
        let contract_address = Address::generate(rng);
        let sender = Address::generate(rng);
        let block_time = rng.get_u32() as i64;
        let block_production_time = rng.get_u32() as i64;
        let current_transaction = <[u8; 32]>::generate(rng);
        let original_transaction = <[u8; 32]>::generate(rng);
        ContractContext {
            contract_address,
            sender,
            block_time,
            block_production_time,
            current_transaction,
            original_transaction,
        }
    }
}
