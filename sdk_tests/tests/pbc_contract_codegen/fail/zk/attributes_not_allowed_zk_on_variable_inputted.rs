use pbc_contract_codegen::init;
use pbc_contract_codegen::zk_on_variable_inputted;

pub fn main() {}

struct ContractState {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> ContractState   {
    ContractState  {}
}

#[zk_on_variable_inputted(shortname=0x32)]
fn do_zk_on_variable_inputted(
    _context: pbc_contract_common::context::ContractContext,
    state: ContractState,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
    created_variables: pbc_contract_common::zk::SecretVarId,
) -> ContractState {
        state
}

