use pbc_contract_codegen::action;

pub fn main() {}

#[action]
fn action(_context: pbc_contract_common::context::ContractContext, state: u64) -> u64 {
    state
}
