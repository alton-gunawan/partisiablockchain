use pbc_contract_codegen::callback;
use pbc_contract_codegen::init;

pub fn main() {}

#[init(zk = false)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
) -> u64 {
    0
}

#[callback(shortname = 0x21, zk = true)]
fn callback(
    _context: pbc_contract_common::context::ContractContext,
    _callback_context: pbc_contract_common::context::CallbackContext,
    state: u64,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    state
}