use pbc_contract_codegen::init;
use pbc_contract_codegen::zk_on_secret_input;
use create_type_spec_derive::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

pub fn main() {}

#[derive(CreateTypeSpec, ReadWriteState)]
struct MyTotallySecretStructTrustMe {
    v1: i32,
    v2: i8,
}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[zk_on_secret_input(shortname = 0x04, secret_type = "MyTotallySecretStructTrustMe")]
fn do_zk_on_secret_input(
    _context: pbc_contract_common::context::ContractContext,
    state: u64,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> (u64, Vec<pbc_contract_common::events::EventGroup>, pbc_contract_common::zk::ZkInputDef<u64, MyTotallySecretStructTrustMe>) {
    let def = pbc_contract_common::zk::ZkInputDef::with_metadata(state);
    (state, vec![], def)
}
