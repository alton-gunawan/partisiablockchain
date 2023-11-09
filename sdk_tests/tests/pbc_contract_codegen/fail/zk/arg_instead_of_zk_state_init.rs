use pbc_contract_codegen::init;

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
    arg1: u64,
) -> u64 {
    arg1
}
