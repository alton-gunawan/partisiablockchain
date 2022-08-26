#![no_main]
use pbc_contract_codegen::action;

pub fn main() {}

#[action]
pub fn action(_context: pbc_contract_common::context::ContractContext, _x: u64) {}
