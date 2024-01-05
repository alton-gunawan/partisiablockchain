use std::io::Write;
use pbc_contract_codegen::init;
use pbc_traits::WriteRPC;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::test_examples::{EXAMPLE_CONTEXT,example_zk_state_bytes};

fn main() {}

struct ContractState { }

impl pbc_traits::ReadWriteState for ContractState {
    const SERIALIZABLE_BY_COPY: bool = true;
    fn state_read_from<T: std::io::Read>(_reader: &mut T) -> Self { Self {} }
    fn state_write_to<T: std::io::Write>(&self, _writer: &mut T) -> std::io::Result<()> { std::io::Result::Ok(()) }
}

#[init(zk = true)]
fn do_zk_init(
    _context: ContractContext,
    _zk_state: pbc_contract_common::zk::ZkState<u32>,
    _arg1: u32,
) -> ContractState {
    ContractState {}
}

fn setup_buffers() -> Vec<u8> {
    let mut input_buf = Vec::new();
    EXAMPLE_CONTEXT.rpc_write_to(&mut input_buf).unwrap();
    // ZkState Argument
    input_buf.write(&example_zk_state_bytes()).unwrap();
    // RPC Argument
    0x01020304u32.rpc_write_to(&mut input_buf).unwrap();
    input_buf
}

fn test_interface() {
    let mut input_buf = setup_buffers();
    __pbc_autogen__do_zk_init_wrapped(
        input_buf.as_mut_ptr(),
        input_buf.len(),
    );
}
