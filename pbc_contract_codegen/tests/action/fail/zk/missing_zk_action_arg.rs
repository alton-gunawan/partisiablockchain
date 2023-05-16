use pbc_contract_codegen::action;
use pbc_contract_codegen::init;

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[action]
fn action(
    _context: pbc_contract_common::context::ContractContext,
    state: u64,
) -> u64 {
    state
}