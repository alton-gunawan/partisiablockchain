use pbc_contract_codegen::action;

pub fn main() {}

type ContractState = u64;

#[action(shortname = 0x9090909090909090909000)]
pub fn very_big_shortname(
    _context: pbc_contract_common::context::ContractContext,
    state: ContractState,
) -> ContractState {
    state
}
