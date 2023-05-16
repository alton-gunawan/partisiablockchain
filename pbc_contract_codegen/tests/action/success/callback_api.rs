use pbc_contract_codegen::{action, callback, init};
use pbc_contract_common::events::EventGroup;

type MyState = Vec<u32>;

#[init]
fn uno(_context: pbc_contract_common::context::ContractContext) -> MyState {
    vec![]
}

#[action(shortname = 0x01)]
fn dos(
    _context: pbc_contract_common::context::ContractContext,
    state: MyState,
) -> (MyState, Vec<EventGroup>) {
    let mut e = EventGroup::builder();
    e.with_callback(SHORTNAME_TRES).argument(9u32).done();
    (state, vec![e.build()])
}

#[callback(shortname = 0x02)]
fn tres(
    _context: pbc_contract_common::context::ContractContext,
    _callback: pbc_contract_common::context::CallbackContext,
    mut state: MyState,
    arg1: u32,
) -> MyState {
    state.push(arg1);
    state
}

fn main() {
    // Do nothing
}
