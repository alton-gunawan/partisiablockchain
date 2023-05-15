use pbc_contract_codegen::action;
use pbc_contract_codegen::init;

pub fn main() {}

#[init(zk = false)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
) -> u64 {
    0
}

#[action(zk = true)]
fn action(
    _context: pbc_contract_common::context::ContractContext,
    state: u64,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    state
}