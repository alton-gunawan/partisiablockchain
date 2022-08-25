use pbc_contract_codegen::action;

pub fn main() {}

#[action]
fn action(_context: pbc_contract_common::context::ContractContext, state: u32, arg1: u32) -> u32 {
    state.wrapping_add(arg1)
}
