use pbc_contract_codegen::action;
use pbc_contract_codegen::init;

pub fn main() {}

struct ZkState { }

impl pbc_traits::ReadRPC for ZkState {
    fn rpc_read_from<T: std::io::Read>(_reader: &mut T) -> Self {
        unimplemented!()
    }
}

impl pbc_traits::WriteRPC for ZkState {
    fn rpc_write_to<T: std::io::Write>(&self, _writer: &mut T) -> std::io::Result<()> {
        unimplemented!()
    }
}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u64>,
) -> u64 {
    0
}

#[action(zk = true)]
fn action(_context: pbc_contract_common::context::ContractContext, state: u64, _zk_state: ZkState) -> u64 { state }
