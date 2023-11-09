use pbc_contract_codegen::{action, callback, init};

type MyState = Vec<u16>;

#[init]
fn uno(_context: pbc_contract_common::context::ContractContext) -> MyState {
        vec![6, 3, 9, 2]
}

#[action(shortname = 0x01)]
fn dos(_context: pbc_contract_common::context::ContractContext, state: MyState) -> MyState {
    state
}

#[callback(shortname = 0x02)]
fn tres(
    _context: pbc_contract_common::context::ContractContext,
    _callback: pbc_contract_common::context::CallbackContext,
    state: MyState,
) -> MyState {
    state
}

/// Main test: Check that ABI functions are available.
///
/// NOTE: We are just checking that the definitions occur with the expected names. We are not
/// checking semantics.
pub fn main() {
    #[cfg(feature = "abi")]
    {
        // Check that direct ABI constructors exists
        let _abi_fns: Vec<for<'r> fn(&'r std::collections::BTreeMap<String, u8>) -> pbc_contract_common::abi::FnAbi> = vec![
             __abi_fn_uno,
             __abi_fn_dos,
             __abi_fn_tres,
        ];
        // Check that indirect ABI constructors exists
        let _abi_fn_as_fn_ptrs: Vec<unsafe extern "C" fn() -> *const ()> = vec![
             __abi_fn_as_fn_ptr___abi_fn_uno,
             __abi_fn_as_fn_ptr___abi_fn_dos,
             __abi_fn_as_fn_ptr___abi_fn_tres,
        ];
    }
}
