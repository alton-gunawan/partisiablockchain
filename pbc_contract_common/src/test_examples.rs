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

/// Generator of Signatures examples
fn example_signature_rng(rng: &mut Rng) -> Signature {
    Signature {
        recovery_id: rng.get_u8(),
        value_r: rng.get_bytearray(),
        value_s: rng.get_bytearray(),
    }
}

/// Generator of Signatures examples
pub fn example_signature() -> Signature {
    let mut rng = Rng::new(312);
    example_signature_rng(&mut rng)
}

/// Generator of example data attestations.
pub fn example_data_attestation() -> zk::DataAttestation {
    let mut rng = Rng::new(312);
    zk::DataAttestation {
        attestation_id: zk::AttestationId::new(1),
        data: vec![1, 2, 3, 4, 5, 6, 7, 8],
        signatures: vec![
            Some(example_signature_rng(&mut rng)),
            Some(example_signature_rng(&mut rng)),
            Some(example_signature_rng(&mut rng)),
            Some(example_signature_rng(&mut rng)),
        ],
    }
}

/// Generate example event subscription.
pub fn example_subscription() -> zk::evm_event::EventSubscription {
    let mut rng = Rng::new(312);
    zk::evm_event::EventSubscription {
        subscription_id: zk::evm_event::EventSubscriptionId::new(1),
        is_active: true,
        chain_id: "POLYGON".to_string(),
        contract_address: rng.get_bytearray(),
        from_block: U256 {
            bytes: rng.get_bytearray(),
        },
        topics: vec![
            vec![rng.get_bytearray()],
            vec![],
            vec![rng.get_bytearray(), rng.get_bytearray()],
        ],
    }
}

/// Generate example external event log.
pub fn example_external_event() -> zk::evm_event::ExternalEvent {
    let mut rng = Rng::new(312);
    zk::evm_event::ExternalEvent {
        subscription_id: zk::evm_event::EventSubscriptionId::new(1),
        event_id: zk::evm_event::ExternalEventId::new(1),
        data: vec![rng.get_u8(), rng.get_u8(), rng.get_u8(), rng.get_u8()],
        topics: vec![
            rng.get_bytearray(),
            rng.get_bytearray(),
            rng.get_bytearray(),
            rng.get_bytearray(),
        ],
    }
}

/// Generator of example callback contexts
/// Deserializes to:
/// `
/// zk::ZkState {
///     calculation_state: zk::CalculationStatus::Waiting,
///     pending_inputs: AvlTreeMap::with_id(-1),
///     secret_variables: AvlTreeMap::with_id(-2),
///     data_attestations: AvlTreeMap::with_id(-3),
///     event_subscriptions: AvlTreeMap::with_id(-4),
///     external_events: AvlTreeMap::with_id(-5),
/// }
/// `
pub fn example_zk_state_bytes() -> Vec<u8> {
    vec![
        0, 255, 255, 255, 255, 255, 255, 255, 254, 255, 255, 255, 253, 255, 255, 255, 252, 255,
        255, 255, 251,
    ]
}

/// Generate example secret var id
pub fn example_secret_var_id() -> zk::SecretVarId {
    zk::SecretVarId::new(1)
}

/// Generate example attestation id
pub fn example_attestation_id() -> zk::AttestationId {
    zk::AttestationId::new(1)
}

/// Generate example event subscription id
pub fn example_event_subscription_id() -> zk::evm_event::EventSubscriptionId {
    zk::evm_event::EventSubscriptionId::new(1)
}

/// Generate example external event id
pub fn example_external_event_id() -> zk::evm_event::ExternalEventId {
    zk::evm_event::ExternalEventId::new(1)
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
