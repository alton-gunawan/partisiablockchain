use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_codegen::init;
use pbc_contract_codegen::zk_on_secret_input;
use pbc_zk::{Sbi32, SecretBinary};

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[derive(CreateTypeSpec, SecretBinary)]
struct SecretStructWithIntArray {
    v1: [Sbi32; 10],
}

#[zk_on_secret_input(shortname = 0x04, secret_type = "SecretStructWithIntArray")]
fn do_zk_on_secret_input(
    _context: pbc_contract_common::context::ContractContext,
    state: u64,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> (
    u64,
    Vec<pbc_contract_common::events::EventGroup>,
    pbc_contract_common::zk::ZkInputDef<u64, SecretStructWithIntArray>,
) {
    let def = pbc_contract_common::zk::ZkInputDef::with_metadata(state);
    (state, vec![], def)
}
