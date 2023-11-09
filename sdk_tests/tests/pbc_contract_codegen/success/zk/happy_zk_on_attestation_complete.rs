use pbc_contract_codegen::init;
use pbc_contract_codegen::zk_on_attestation_complete;

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[zk_on_attestation_complete]
fn do_zk_on_attestation_complete(
    _context: pbc_contract_common::context::ContractContext,
    _state: u64,
    zk_state: pbc_contract_common::zk::ZkState<u64>,
    attestation_id: pbc_contract_common::zk::AttestationId,
) -> u64 {
    zk_state.get_attestation(attestation_id).unwrap().data.len() as u64
}
