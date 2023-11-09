#![no_main]
use pbc_contract_codegen::{action};

pub fn main() {}

#[action]
pub fn action_with_wildcard_state_id(
    context: pbc_contract_common::context::ContractContext,
    5: u64,
) {}
