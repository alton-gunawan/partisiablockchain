use pbc_contract_codegen::action;

pub fn main() {}

type ContractState = u64;

#[action(shortname = 1)]
pub fn not_hex(
    _context: pbc_contract_common::context::ContractContext,
    state: ContractState,
) -> ContractState {
    state
}
