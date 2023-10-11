#![cfg(feature = "test_lib")]

use pbc_contract_codegen::{action, callback, init};
use pbc_contract_common::address::Shortname;
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::test_examples::*;
#[cfg(feature = "abi")]
use sdk_tests::test_contract_behaviour::{assert_abi_serializable, EXPECTED_DO_THING_ABI_BYTES};
use sdk_tests::test_contract_behaviour::{rpc_self, test_contract_function_with_variants};

type ContractState = u64;

#[init]
fn initialize(_context: ContractContext, arg1: u16) -> (ContractState, Vec<EventGroup>) {
    (arg1 as ContractState, vec![])
}

#[action(shortname = 0x05)]
fn do_thing(
    _context: ContractContext,
    state: ContractState,
    arg1: u16,
) -> (ContractState, Vec<EventGroup>) {
    let mut e = EventGroup::builder();
    e.call(
        EXAMPLE_ADDRESS_1,
        Shortname::from_be_bytes(&[0x09]).unwrap(),
    )
    .argument(5u8)
    .argument(1u8)
    .argument(9u8)
    .done();
    e.call(
        EXAMPLE_ADDRESS_1,
        Shortname::from_be_bytes(&[0x09]).unwrap(),
    )
    .argument(5u8)
    .argument(7u8)
    .argument(3u8)
    .done();

    (state.wrapping_add(arg1 as ContractState), vec![e.build()])
}

#[action(shortname = 0x93A0F29702)]
fn do_thing_2(_context: ContractContext, state: ContractState, arg1: u16) -> ContractState {
    state.wrapping_add(arg1 as ContractState)
}

#[callback(shortname = 0x00)]
fn call_me_maybe_1(
    _context: ContractContext,
    _callback_context: CallbackContext,
    state: ContractState,
    arg1: u16,
) -> (ContractState, Vec<EventGroup>) {
    (state.wrapping_add(arg1 as ContractState), vec![])
}

#[callback(shortname = 0x02)]
fn call_me_maybe_2(
    _context: ContractContext,
    _callback_context: CallbackContext,
    state: ContractState,
    arg1: u16,
) -> ContractState {
    state.wrapping_add(arg1 as ContractState)
}

#[test]
fn init_behaviour() {
    let segments: [&[u8]; 2] = [
        &rpc_self(EXAMPLE_CONTEXT), // Context
        &[8, 9],                    // RPC: arg1
    ];
    test_contract_function_with_variants(__pbc_autogen__initialize_wrapped, &segments);
}

#[test]
fn action_behaviour() {
    let segments: [&[u8]; 3] = [
        &rpc_self(EXAMPLE_CONTEXT), // Context
        &[1, 2, 3, 4, 1, 2, 3, 4],  // State
        &[8, 9],                    // RPC: arg1
    ];
    test_contract_function_with_variants(__pbc_autogen__do_thing_wrapped, &segments);
    test_contract_function_with_variants(__pbc_autogen__do_thing_2_wrapped, &segments);
}

#[test]
fn callback_behaviour() {
    let segments: [&[u8]; 4] = [
        &rpc_self(EXAMPLE_CONTEXT),            // Context
        &rpc_self(example_callback_context()), // Callback context
        &[1, 2, 3, 4, 1, 2, 3, 4],             // State
        &[8, 9],                               // RPC: arg1
    ];
    test_contract_function_with_variants(__pbc_autogen__call_me_maybe_1_wrapped, &segments);
    test_contract_function_with_variants(__pbc_autogen__call_me_maybe_2_wrapped, &segments);
}

#[cfg(feature = "abi")]
#[test]
fn generated_abi_do_thing() {
    assert_abi_serializable(__abi_fn_do_thing, EXPECTED_DO_THING_ABI_BYTES);
}

#[cfg(feature = "abi")]
#[test]
fn generated_abi_do_thing_2() {
    assert_abi_serializable(
        __abi_fn_do_thing_2,
        [
            0x02, // Function kind: Action
            0, 0, 0, 10, // Name length
            100, 111, 95, 116, 104, 105, 110, 103, 95, 50, // Name
            0x93, 0xA0, 0xF2, 0x97, 0x02, // Shortname
            0, 0, 0, 1, // Number arguments
            0, 0, 0, 4, // Argument 0 Name Length
            97, 114, 103, 49,   // Argument 0 Name
            0x02, // Field 0 type ordinal
        ],
    );
}
