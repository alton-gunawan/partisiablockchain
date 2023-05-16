//! Testing module with example data.
//!
//! Publicly available, as it's depended upon from other modules.
#![allow(dead_code)]

use crate::address::{Address, AddressType};
use crate::context::{CallbackContext, ContractContext, ExecutionResult};
use crate::signature::Signature;
use crate::sorted_vec_map::SortedVecMap;
use crate::zk;
use crate::{BlsPublicKey, BlsSignature, Hash, PublicKey, U256};

/// Example address
pub const EXAMPLE_ADDRESS_1: Address = Address {
    address_type: AddressType::PublicContract,
    identifier: [
        2, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2,
    ],
};

/// Example address
pub const EXAMPLE_ADDRESS_2: Address = Address {
    address_type: AddressType::Account,
    identifier: [
        29, 3, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2,
    ],
};

/// Example Hash
pub const EXAMPLE_HASH_1: Hash = Hash {
    bytes: [
        0, 1, 23, 213, 124, 23, 3, 1, 23, 12, 31, 23, 123, 24, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23,
        2, 3, 23, 2, 3, 23, 2,
    ],
};

/// Example Hash
pub const EXAMPLE_HASH_2: Hash = Hash {
    bytes: [
        124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23,
        2, 3, 23, 2, 3, 23, 2,
    ],
};

/// Example U256
pub const EXAMPLE_U256: U256 = U256 {
    bytes: [
        124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23,
        2, 3, 23, 2, 3, 23, 42,
    ],
};

/// Example PublicKey
pub const EXAMPLE_PUBLIC_KEY: PublicKey = PublicKey {
    bytes: [
        124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23,
        2, 3, 23, 2, 3, 23, 2, 42,
    ],
};

/// Example BlsPublicKey
pub const EXAMPLE_BLS_PUBLIC_KEY: BlsPublicKey = BlsPublicKey {
    bytes: [
        124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23,
        2, 3, 23, 2, 3, 23, 42, 124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3,
        2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 42, 124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13,
        3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 42,
    ],
};

/// Example BlsSignature
pub const EXAMPLE_BLS_SIGNATURE: BlsSignature = BlsSignature {
    bytes: [
        124, 25, 3, 1, 23, 12, 31, 23, 123, 26, 13, 3, 123, 32, 3, 2, 2, 3, 2, 3, 2, 32, 32, 3, 23,
        2, 3, 23, 2, 3, 23, 2, 42, 2, 3, 2, 32, 32, 3, 23, 2, 3, 23, 2, 3, 23, 2, 42,
    ],
};

/// Generator of example sorted vec map
pub fn example_vec_map() -> SortedVecMap<u8, Vec<String>> {
    let mut map: SortedVecMap<u8, Vec<String>> = SortedVecMap::new();
    map.insert(
        1,
        vec!["my".to_string(), "name".to_string(), "is".to_string()],
    );
    map.insert(2, vec!["what".to_string()]);
    map
}

/// Example contract context
pub const EXAMPLE_CONTEXT: ContractContext = ContractContext {
    contract_address: EXAMPLE_ADDRESS_1,
    sender: EXAMPLE_ADDRESS_2,
    block_time: 53,
    block_production_time: 53,
    current_transaction: EXAMPLE_HASH_1,
    original_transaction: EXAMPLE_HASH_2,
};

/// Generator of example callback contexts
pub fn example_callback_context() -> CallbackContext {
    CallbackContext {
        success: true,
        results: vec![
            ExecutionResult {
                succeeded: true,
                return_data: vec![],
            },
            ExecutionResult {
                succeeded: true,
                return_data: vec![],
            },
            ExecutionResult {
                succeeded: true,
                return_data: vec![],
            },
        ],
    }
}

/// Defines the Metadata type used for [`zk::ZkClosed`] example instances
pub type ExampleZkMetadata = u32;

/// Example secret variable id
pub const SECRET_VAR_ID_31: zk::SecretVarId = zk::SecretVarId::new(31);

/// Example secret variable id
pub const SECRET_VAR_ID_30: zk::SecretVarId = zk::SecretVarId::new(30);

/// Example secret variable id
pub const SECRET_VAR_ID_4: zk::SecretVarId = zk::SecretVarId::new(4);

/// Example ZkClosed 1
///
/// Metadata is explicitly NOT palindromic wrt. endianess.
pub const ZK_CLOSED_1: zk::ZkClosed<ExampleZkMetadata> = zk::ZkClosed {
    variable_id: SECRET_VAR_ID_31,
    owner: EXAMPLE_ADDRESS_1,
    is_sealed: false,
    metadata: 0xFF,
    data: None,
};

/// Example ZkClosed 2
///
/// Metadata is explicitly NOT palindromic wrt. endianess.
pub const ZK_CLOSED_2: zk::ZkClosed<ExampleZkMetadata> = zk::ZkClosed {
    variable_id: SECRET_VAR_ID_30,
    owner: EXAMPLE_ADDRESS_2,
    is_sealed: false,
    metadata: 0xFF00,
    data: None,
};

/// Generator of open example ZkClosed
///
/// Metadata and data is explicitly NOT palindromic wrt. endianess.
pub fn zk_closed_open() -> zk::ZkClosed<ExampleZkMetadata> {
    zk::ZkClosed {
        variable_id: SECRET_VAR_ID_4,
        owner: EXAMPLE_ADDRESS_1,
        is_sealed: false,
        metadata: 0xDEADBEEF,
        data: Some(vec![1, 2, 3]),
    }
}

/// Generator of ZkInputDef examples
///
/// Metadata is explicitly NOT palindromic wrt. endianess.
pub fn zk_input_def(seed: u32) -> zk::ZkInputDef<ExampleZkMetadata> {
    assert_ne!(seed, u32::from_be(seed));
    zk::ZkInputDef {
        seal: seed % 2 == 0,
        expected_bit_lengths: (1..(seed % 10 + 2)).collect(),
        metadata: seed,
    }
}

/// Generator of Signatures examples
fn example_signature(rng: &mut Rng) -> Signature {
    Signature {
        recovery_id: rng.get_u8(),
        value_r: rng.get_bytearray(),
        value_s: rng.get_bytearray(),
    }
}

/// Generator of example data attestations.
pub fn example_data_attestation() -> zk::DataAttestation {
    let mut rng = Rng::new(312);
    zk::DataAttestation {
        attestation_id: zk::AttestationId::new(1),
        data: vec![1, 2, 3, 4, 5, 6, 7, 8],
        signatures: vec![
            example_signature(&mut rng),
            example_signature(&mut rng),
            example_signature(&mut rng),
            example_signature(&mut rng),
        ],
    }
}

/// Generator of example callback contexts
pub fn example_zk_state() -> zk::ZkState<ExampleZkMetadata> {
    zk::ZkState {
        calculation_state: zk::CalculationStatus::Waiting,
        pending_inputs: vec![ZK_CLOSED_1],
        secret_variables: vec![ZK_CLOSED_2, zk_closed_open()],
        data_attestations: vec![example_data_attestation()],
        reserved_1: 0,
        reserved_2: 0,
    }
}

/// Simple Linear Congruential Generator RNG for generating example data.
struct Rng {
    state: u32,
}

impl Rng {
    const MODULO: u32 = 2147483648;
    const MULTIPLIER: u32 = 1103515245;
    const INCREMENT: u32 = 12345;

    /// Creates new rng instance from the given seed.
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    /// Retrieves a new u32, and updates Rng internal state.
    pub fn get_u32(&mut self) -> u32 {
        self.state = Rng::MULTIPLIER
            .wrapping_mul(self.state)
            .wrapping_add(Rng::INCREMENT)
            .wrapping_rem_euclid(Rng::MODULO);
        self.state
    }

    /// Retrieves a new u8, and updates Rng internal state.
    pub fn get_u8(&mut self) -> u8 {
        self.get_u32() as u8
    }

    /// Retrieves a bytearray, and updates Rng internal state.
    pub fn get_bytearray<const N: usize>(&mut self) -> [u8; N] {
        let mut out = [0; N];
        for entry in &mut out {
            *entry = self.get_u8();
        }
        out
    }
}
