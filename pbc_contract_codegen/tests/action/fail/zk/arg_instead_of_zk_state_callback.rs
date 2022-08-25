use pbc_contract_codegen::callback;

pub fn main() {}

#[callback(shortname = 0x02)]
fn callback(
    _context: pbc_contract_common::context::ContractContext,
    _callback: pbc_contract_common::context::CallbackContext,
    state: u64,
    arg1: u64,
) -> u64 {
    state.wrapping_add(arg1)
}
