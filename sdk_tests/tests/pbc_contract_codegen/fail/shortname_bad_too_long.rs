use pbc_contract_codegen::action;

pub fn main() {}

type ContractState = u64;

#[action(shortname = 0x99AABBCCEEFF55)]
pub fn very_big_shortname(
    _context: pbc_contract_common::context::ContractContext,
    state: ContractState,
) -> ContractState {
    state
}
