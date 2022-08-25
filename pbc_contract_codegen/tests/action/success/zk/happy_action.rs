use pbc_contract_codegen::action;

pub fn main() {}

#[action]
fn action(
    _context: pbc_contract_common::context::ContractContext,
    state: u64,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    state
}
