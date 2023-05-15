use pbc_contract_codegen::callback;
use pbc_contract_codegen::init;

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[callback(shortname = 0x02, zk = true)]
fn callback(
    _context: pbc_contract_common::context::ContractContext,
    _callback: pbc_contract_common::context::CallbackContext,
    state: u64,
    arg1: u64,
) -> u64 {
    state.wrapping_add(arg1)
}
