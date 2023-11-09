use pbc_contract_codegen::action;

pub fn main() {}

type ContractState = u64;

#[action(shortname = -0x01)]
pub fn negative_hex(
    _context: pbc_contract_common::context::ContractContext,
    state: ContractState,
) -> ContractState {
    state
}
