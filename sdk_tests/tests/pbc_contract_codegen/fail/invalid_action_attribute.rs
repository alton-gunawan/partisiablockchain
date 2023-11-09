#![no_main]

use pbc_contract_codegen::action;
use pbc_contract_common::context::ContractContext;

pub fn main() {}

#[action(cakes = "galore")]
pub fn action(_context: ContractContext, _state: u8, _x: u64) {}
