use pbc_contract_codegen::init;
use pbc_contract_codegen::zk_on_secret_input;

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[zk_on_secret_input(shortname = 0x04)]
fn do_zk_on_secret_input(
    _context: ContractContext,
    state: u32,
) -> (u32, Vec<EventGroup>, zk::ZkInputDef<u32>) {
    let def = zk::ZkInputDef {
        expected_bit_lengths: vec![10],
        seal: false,
        metadata: state,
    };
    (state, vec![], def)
}
