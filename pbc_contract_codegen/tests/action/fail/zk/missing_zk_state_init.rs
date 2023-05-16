use pbc_contract_codegen::init;

pub fn main() {}

#[init(zk = true)]
fn init(
    _context: pbc_contract_common::context::ContractContext,
) -> u64 {
    0
}
