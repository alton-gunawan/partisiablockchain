use pbc_contract_codegen::action;
use pbc_contract_common::zk::ZkState as MyZkState;

pub fn main() {}

#[action]
pub fn action(_context: pbc_contract_common::context::ContractContext, state: u64, _zk_state: MyZkState<u32>) -> u64 { state }
