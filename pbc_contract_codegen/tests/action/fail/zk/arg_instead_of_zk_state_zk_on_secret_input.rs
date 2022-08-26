use pbc_contract_codegen::zk_on_secret_input;
use pbc_contract_common::zk::*;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::events::EventGroup;

pub fn main() {}

#[zk_on_secret_input(shortname = 0x04)]
fn do_zk_on_secret_input(
    _context: ContractContext,
    mut state: u32,
    arg1: u32,
) -> (u32, Vec<EventGroup>, ZkInputDef<u32>) {
    state = state.wrapping_add(arg1);
    let def = ZkInputDef {
        expected_bit_lengths: vec![10],
        seal: false,
        metadata: state,
    };
    (state, vec![], def)
}
