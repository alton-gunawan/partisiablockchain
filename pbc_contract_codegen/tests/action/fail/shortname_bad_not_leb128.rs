use pbc_contract_codegen::action;

pub fn main() {}

type ContractState = u64;

#[action(shortname = 0xFF)]
pub fn not_leb128(
    _context: pbc_contract_common::context::ContractContext,
    state: ContractState,
) -> ContractState {
    state
}
