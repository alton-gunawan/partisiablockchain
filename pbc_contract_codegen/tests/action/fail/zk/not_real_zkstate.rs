
use pbc_contract_codegen::action;

pub fn main() {}

struct ZkState { }

impl pbc_traits::ReadWriteRPC for ZkState {
    fn rpc_read_from<T: std::io::Read>(_reader: &mut T) -> Self {
        unimplemented!()
    }
    fn rpc_write_to<T: std::io::Write>(&self, _writer: &mut T) -> std::io::Result<()> {
        unimplemented!()
    }
}

#[action]
fn action(_context: pbc_contract_common::context::ContractContext, state: u64, _zk_state: ZkState) -> u64 { state }
