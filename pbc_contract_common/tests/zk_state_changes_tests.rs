use pbc_contract_common::shortname::ShortnameZkComputation;
use pbc_contract_common::zk::ZkStateChange;
use pbc_contract_common::ContractResultBuffer;

fn assert_written(actual: Vec<ZkStateChange>, expected: Vec<u8>) {
    let mut gotten_buffer = ContractResultBuffer::new();
    gotten_buffer.write_zk_state_change(actual);
    assert_eq!(gotten_buffer.data, expected);
}

const EXAMPLE_METADATA: u8 = 5;
const EXAMPLE_INPUTS: [u8; 2] = [1, 2];

const SHORTNAME_COMPUTE: ShortnameZkComputation = ShortnameZkComputation::from_u32(0x35);

#[test]
fn start_computation() {
    let state_changes = vec![ZkStateChange::start_computation::<u8>(
        SHORTNAME_COMPUTE,
        vec![EXAMPLE_METADATA],
    )];
    let expected_bytes = vec![
        0, 0, 0, 0,  // Empty length
        17, // Zk state change section id
        0, 0, 0, 22, // Zk state section length
        0, 0, 0, 1, // Number of state changes
        9, // Discriminant: Start computation with shortname
        0, 0, 0, 0x35, // Shortname
        0, 0, 0, 1, // Number of pieces of metadata
        0, 0, 0, 1, // Size of metadata
        5, // Metadata
        0, 0, 0, 0, // Number of input arguments
    ];
    assert_written(state_changes, expected_bytes);
}

#[test]
fn start_computation_with_inputs() {
    let state_changes = vec![ZkStateChange::start_computation_with_inputs::<u8, u8>(
        SHORTNAME_COMPUTE,
        vec![EXAMPLE_METADATA],
        EXAMPLE_INPUTS.to_vec(),
    )];
    let expected_bytes = vec![
        0, 0, 0, 0,  // Empty length
        17, // Zk state change section id
        0, 0, 0, 32, // Zk state section length
        0, 0, 0, 1, // Number of state changes
        9, // Discriminant: Start computation with shortname
        0, 0, 0, 0x35, // Shortname
        0, 0, 0, 1, // Number of pieces of metadata
        0, 0, 0, 1, // Size of metadata
        5, // Metadata
        0, 0, 0, 2, // Number of input arguments
        0, 0, 0, 1, // Size of input 1
        1, // Input 1
        0, 0, 0, 1, // Size of input 2
        2, // Input 1
    ];
    assert_written(state_changes, expected_bytes);
}

#[test]
#[allow(deprecated)]
fn using_output_complete_panics() {
    let state_changes = vec![ZkStateChange::OutputComplete {}];
    let expected_bytes = vec![
        0, 0, 0, 0,  // Empty length
        17, // Zk state change section id
        0, 0, 0, 9, // Zk state section length
        0, 0, 0, 1, // Number of state changes
        3, // Discriminant: Delete variables
        0, 0, 0, 0, // Number to delete
    ];
    assert_written(state_changes, expected_bytes);
}
