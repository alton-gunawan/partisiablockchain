use pbc_contract_codegen::init;
use pbc_contract_codegen::zk_on_variable_inputted;

type ContractState = u32;
type Metadata = u32;

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[zk_on_variable_inputted]
fn zk_on_variable_inputted(
    _context: pbc_contract_common::context::ContractContext,
    state: ContractState,
    _zk_state: ZkState<Metadata>,
    _variable_id: SecretVarId,
    rpc_arg: u32, // <-- Not allowed for zk_on_variable_inputted
) -> ContractState {
    state.wrapping_add(rpc_arg)
}
