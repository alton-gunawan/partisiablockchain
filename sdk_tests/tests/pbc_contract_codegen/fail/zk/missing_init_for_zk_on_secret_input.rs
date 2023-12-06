use pbc_contract_codegen::zk_on_secret_input;
use pbc_zk::Sbi32;

pub fn main() {}

#[zk_on_secret_input(shortname = 0x04)]
fn do_zk_on_secret_input(
    _context: pbc_contract_common::context::ContractContext,
    state: u64,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> (
    u64,
    Vec<pbc_contract_common::events::EventGroup>,
    pbc_contract_common::zk::ZkInputDef<u64, Sbi32>,
) {
    let def = pbc_contract_common::zk::ZkInputDef::with_metadata(state);
    (state, vec![], def)
}
