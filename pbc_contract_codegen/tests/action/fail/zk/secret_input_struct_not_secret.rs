use pbc_contract_codegen::zk_on_secret_input;
use create_type_spec_derive::CreateTypeSpec;

pub fn main() {}

#[derive(CreateTypeSpec)]
struct SecretStruct {
    v1: i32,
    v2: i8,
}

#[zk_on_secret_input(shortname = 0x04, secret_type = "SecretStruct")]
fn do_zk_on_secret_input(
    _context: pbc_contract_common::context::ContractContext,
    state: u64,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> (u64, Vec<pbc_contract_common::events::EventGroup>, pbc_contract_common::zk::ZkInputDef<u64>) {
    let def = pbc_contract_common::zk::ZkInputDef {
        expected_bit_lengths: vec![10],
        seal: false,
        metadata: state,
    };
    (state, vec![], def)
}