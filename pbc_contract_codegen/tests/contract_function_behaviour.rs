#[cfg(feature = "abi")]
use pbc_contract_common::abi::AbiSerialize;

/// Creates a bunch of segment variants, possibly duplicating segments or leaving them out.
///
/// For example for segments `[X, Y, Z]`, it will produce variants:
///
/// - []
/// - [X], [Y], [Z]
/// - [X, X, Y, Z]
/// - [X, X, Y, Y, Z, Z]
/// - ....
fn all_variants(segments: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut variants = vec![];
    let mut variant_idx = 0usize;
    while variant_idx < 3u32.pow(segments.len() as u32) as usize {
        let mut segment_counter = variant_idx;
        let mut variant = vec![];
        for &seg in segments {
            // Add between 0 and 2
            for _ in 0..(segment_counter % 3) {
                variant.extend(seg);
            }
            segment_counter /= 3;
        }

        variants.push(variant);
        variant_idx += 1;
    }
    variants
}

fn good_variant(segments: &[&[u8]]) -> Vec<u8> {
    segments.iter().copied().flatten().copied().collect()
}

fn failing_variants(segments: &[&[u8]]) -> Vec<Vec<u8>> {
    let good_variant = good_variant(segments);
    all_variants(segments)
        .into_iter()
        .filter(|x| x.len() != good_variant.len())
        .collect()
}

fn test_contract_function_with_variants(
    call: extern "C" fn(*mut u8, usize) -> u64,
    segments: &[&[u8]],
) {
    let mut good_variant = good_variant(segments);
    let bad_variants: Vec<Vec<u8>> = failing_variants(segments);

    // Good case
    call(good_variant.as_mut_ptr(), good_variant.len());

    // Bad cases
    for variant in bad_variants {
        let result = std::panic::catch_unwind(|| {
            let mut input_buf = variant.clone();
            call(input_buf.as_mut_ptr(), input_buf.len());
        });
        assert!(
            result.is_err(),
            "Succeeded for input bytes, when it should fail: {variant:?}",
        );
    }
}

fn rpc_self<T: pbc_traits::WriteRPC>(v: T) -> Vec<u8> {
    let mut buf = vec![];
    v.rpc_write_to(&mut buf).unwrap();
    buf
}

type ContractState = u64;

#[cfg(feature = "abi")]
fn assert_abi_serializable<
    K,
    V,
    AbiGenFn: FnOnce(&std::collections::BTreeMap<K, V>) -> pbc_contract_common::abi::FnAbi,
    const N: usize,
>(
    abi_gen_fn: AbiGenFn,
    expected_bytes: [u8; N],
) {
    let lut = std::collections::BTreeMap::new();
    let abi = abi_gen_fn(&lut);
    let mut abi_bytes = vec![];
    abi.serialize_abi(&mut abi_bytes).unwrap();
    assert_eq!(abi_bytes, expected_bytes.to_vec());
}

/// Identical between ZK and non-ZK contracts.
#[cfg(feature = "abi")]
const EXPECTED_DO_THING_ABI_BYTES: [u8; 27] = [
    0x02, // Function kind: Action
    0, 0, 0, 8, // Name length
    100, 111, 95, 116, 104, 105, 110, 103,  // Name
    0x05, // Shortname
    0, 0, 0, 1, // Number arguments
    0, 0, 0, 4, // Argument 0 Name Length
    97, 114, 103, 49,   // Argument 0 Name
    0x02, // Field 0 type ordinal: u16
];

#[cfg(not(feature = "zk"))]
mod nonzk {
    use super::*;
    use pbc_contract_codegen::{action, callback, init};
    use pbc_contract_common::address::Shortname;
    use pbc_contract_common::context::{CallbackContext, ContractContext};
    use pbc_contract_common::events::EventGroup;
    use pbc_contract_common::test_examples::*;

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
}

#[cfg(feature = "zk")]
mod zk {
    use super::*;
    use pbc_contract_codegen::*;
    use pbc_contract_common::address::Shortname;
    use pbc_contract_common::context::{CallbackContext, ContractContext};
    use pbc_contract_common::events::EventGroup;
    use pbc_contract_common::test_examples::*;
    use pbc_contract_common::zk::*;

    type ZkMetadata = ExampleZkMetadata;

    #[action(shortname = 0x05)]
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

    #[callback(shortname = 0x04)]
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
        test_contract_function_with_variants(
            __pbc_autogen__do_zk_on_secret_input_wrapped,
            &segments,
        );
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
                100, 111, 95, 122, 107, 95, 111, 110, 95, 115, 101, 99, 114, 101, 116, 95, 105,
                110, 112, 117, 116,  // Name
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
                100, 111, 95, 122, 107, 95, 111, 110, 95, 117, 115, 101, 114, 95, 118, 97, 114,
                105, 97, 98, 108, 101, 115, 95, 111, 112, 101, 110, 101, 100, // Name
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
}
