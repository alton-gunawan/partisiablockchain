#![cfg(feature = "test_lib")]

use pbc_contract_codegen::*;
use pbc_contract_common::address::Shortname;
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::test_examples::*;
use pbc_contract_common::zk::*;
use sdk_tests::test_contract_behaviour::{
    assert_abi_serializable, rpc_self, test_contract_function_with_variants,
    EXPECTED_DO_THING_ABI_BYTES,
};

type ContractState = u64;

#[init(zk = true)]
fn initialize(_context: ContractContext, _zk_state: ZkState<u64>) -> ContractState {
    0
}

type ZkMetadata = ExampleZkMetadata;

#[action(shortname = 0x05, zk = true)]
fn do_thing(
    _context: ContractContext,
    state: ContractState,
    _zk_state: ZkState<ZkMetadata>,
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
        EXAMPLE_ADDRESS_2,
        Shortname::from_be_bytes(&[0x09]).unwrap(),
    )
    .argument(5u8)
    .argument(7u8)
    .argument(3u8)
    .done();

    (state.wrapping_add(arg1 as ContractState), vec![e.build()])
}

#[callback(shortname = 0x04, zk = true)]
fn call_me_discretely(
    _context: ContractContext,
    callback_context: CallbackContext,
    state: ContractState,
    _zk_state: ZkState<ZkMetadata>,
    arg1: u16,
) -> ContractState {
    state
        .wrapping_add(callback_context.results.len() as ContractState)
        .wrapping_add(arg1 as ContractState)
}

#[zk_on_secret_input(shortname = 0x04)]
fn do_zk_on_secret_input(
    _context: ContractContext,
    mut state: ContractState,
    _zk_state: ZkState<ZkMetadata>,
    arg1: u16,
) -> (ContractState, Vec<EventGroup>, ZkInputDef<ZkMetadata>) {
    state = state.wrapping_add(arg1 as ContractState);
    let def = ZkInputDef {
        expected_bit_lengths: vec![10],
        seal: false,
        metadata: state as ZkMetadata,
    };
    (state, vec![], def)
}

#[zk_on_compute_complete]
fn do_zk_on_compute_complete(
    _context: ContractContext,
    state: ContractState,
    _zk_state: ZkState<ZkMetadata>,
    created_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    (
        state,
        vec![],
        vec![ZkStateChange::OpenVariables {
            variables: created_variables,
        }],
    )
}

#[zk_on_user_variables_opened]
fn do_zk_on_user_variables_opened(
    _context: ContractContext,
    state: ContractState,
    _zk_state: ZkState<ZkMetadata>,
    _opened_variables: Vec<SecretVarId>,
    arg1: u16,
) -> ContractState {
    state.wrapping_add(arg1 as ContractState)
}

#[test]
fn action_behaviour() {
    let segments: [&[u8]; 4] = [
        &rpc_self(EXAMPLE_CONTEXT),    // Context
        &[1, 2, 3, 4, 1, 2, 3, 4],     // State
        &rpc_self(example_zk_state()), // ZkState
        &[8, 9],                       // RPC: arg1
    ];
    test_contract_function_with_variants(__pbc_autogen__do_thing_wrapped, &segments);
}

#[test]
fn callback_behaviour() {
    let segments: [&[u8]; 5] = [
        &rpc_self(EXAMPLE_CONTEXT),            // Context
        &rpc_self(example_callback_context()), // Callback context
        &[1, 2, 3, 4, 1, 2, 3, 4],             // State
        &rpc_self(example_zk_state()),         // ZkState
        &[8, 9],                               // RPC: arg1
    ];
    test_contract_function_with_variants(__pbc_autogen__call_me_discretely_wrapped, &segments);
}

#[test]
fn zk_on_secret_input_behaviour() {
    let segments: [&[u8]; 4] = [
        &rpc_self(EXAMPLE_CONTEXT),    // Context
        &[1, 2, 3, 4, 1, 2, 3, 4],     // State
        &rpc_self(example_zk_state()), // ZkState
        &[8, 9],                       // RPC: arg1
    ];
    test_contract_function_with_variants(__pbc_autogen__do_zk_on_secret_input_wrapped, &segments);
}

#[test]
fn zk_on_compute_complete() {
    let variables = vec![SECRET_VAR_ID_30, SECRET_VAR_ID_31];
    let segments: [&[u8]; 4] = [
        &rpc_self(EXAMPLE_CONTEXT),    // Context
        &[1, 2, 3, 4, 1, 2, 3, 4],     // State
        &rpc_self(example_zk_state()), // ZkState
        &rpc_self(variables),          // RPC: Created variables
    ];
    test_contract_function_with_variants(
        __pbc_autogen__do_zk_on_compute_complete_wrapped,
        &segments,
    );
}

#[test]
fn zk_on_user_variables_opened() {
    let variables = vec![SECRET_VAR_ID_30, SECRET_VAR_ID_31];
    let segments: [&[u8]; 5] = [
        &rpc_self(EXAMPLE_CONTEXT),    // Context
        &[1, 2, 3, 4, 1, 2, 3, 4],     // State
        &rpc_self(example_zk_state()), // ZkState
        &rpc_self(variables),          // RPC: Created variables
        &[9, 2],                       // RPC: arg1
    ];
    test_contract_function_with_variants(
        __pbc_autogen__do_zk_on_user_variables_opened_wrapped,
        &segments,
    );
}

#[cfg(feature = "abi")]
#[test]
fn generated_abi_do_thing() {
    assert_abi_serializable(__abi_fn_do_thing, EXPECTED_DO_THING_ABI_BYTES);
}

#[cfg(feature = "abi")]
#[test]
fn generated_abi_zk_on_secret_input() {
    assert_abi_serializable(
        __abi_fn_do_zk_on_secret_input,
        [
            0x17, // Function kind: ZkSecretInputWithExplicitType
            0, 0, 0, 21, // Name length
            100, 111, 95, 122, 107, 95, 111, 110, 95, 115, 101, 99, 114, 101, 116, 95, 105, 110,
            112, 117, 116,  // Name
            0x04, // Shortname
            0, 0, 0, 1, // Number arguments
            0, 0, 0, 4, // Argument Name Length
            97, 114, 103, 49,   // Argument Name
            0x02, // Field 0 type ordinal: u16
            0, 0, 0, 12, // Secret Argument name length
            115, 101, 99, 114, 101, 116, 95, 105, 110, 112, 117, 116,
            8, // Secret Argument name
        ],
    );
}

#[cfg(feature = "abi")]
#[test]
fn generated_abi_zk_on_user_variables_opened() {
    assert_abi_serializable(
        __abi_fn_do_zk_on_user_variables_opened,
        [
            0x15, // Function kind: ZkUserVarOpened
            0, 0, 0, 30, // Name length
            100, 111, 95, 122, 107, 95, 111, 110, 95, 117, 115, 101, 114, 95, 118, 97, 114, 105,
            97, 98, 108, 101, 115, 95, 111, 112, 101, 110, 101, 100, // Name
            228, 180, 239, 148, 1, // Shortname
            0, 0, 0, 1, // Number arguments
            0, 0, 0, 4, // Argument 0 Name Length
            97, 114, 103, 49,   // Argument 0 Name
            0x02, // Field 0 type ordinal: u16
        ],
    );
}

#[cfg(feature = "abi")]
#[test]
fn generated_abi_zk_on_compute_complete() {
    assert_abi_serializable(
        __abi_fn_do_zk_on_compute_complete,
        [
            0x13, // Function kind: ZkComputeComplete
            0, 0, 0, 25, // Name length
            100, 111, 95, 122, 107, 95, 111, 110, 95, 99, 111, 109, 112, 117, 116, 101, 95, 99,
            111, 109, 112, 108, 101, 116, 101, 199, 250, 239, 173,  // Name
            0x01, // Shortname
            0, 0, 0, 0, // Number arguments
        ],
    );
}
