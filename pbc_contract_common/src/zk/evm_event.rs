//! Definitions specific for external EVM events that Zero-Knowledge contracts can subscribe to.
//!
//! [`EventSubscription`] cannot be manually created, but are handled by the system when adding a
//! new subscription using an [`EvmEventFilter`].
//! [`EvmEventFilter`] are created by using [`EvmEventFilterBuilder`].
//! [`ExternalEvent`] cannot be manually created, but are added by the system.

use crate::U256;
use create_type_spec_derive::CreateTypeSpecInternal;
use read_write_rpc_derive::{ReadRPC, WriteRPC};
use read_write_state_derive::ReadWriteState;

/// An externally-owned EVM account address is the 20 least significant bytes of the Keccak-256 hash value of the public key of an account.
/// A contract EVM address is also 20 bytes, and is derived from the creator's address and nonce.
pub type EvmAddress = [u8; 20];

/// Identifies a concrete EVM compatible blockchain network.
pub type EvmChainId = String;

/// An event topic is an indexed piece of data attached to an event log, or used to filter events
/// logs. A topic is restricted to 32 bytes of data.
pub type EvmEventTopic = [u8; 32];

/// Filter to apply to a subscription on EVM events.
#[derive(Eq, PartialEq, Debug, Clone, Ord, PartialOrd, ReadRPC, WriteRPC, ReadWriteState)]
pub struct EvmEventFilter {
    /// Address of contract that emits events.
    address: EvmAddress,
    /// Block number of the earliest block to receive events from.
    from_block: U256,
    /// Event topics, i.e. event signature and indexed parameters. The list is order dependent.
    topics: Vec<EvmEventTopicFilter>,
}

impl EvmEventFilter {
    /// Create a new filter builder.
    pub fn builder(address: EvmAddress) -> EvmEventFilterBuilder {
        EvmEventFilterBuilder {
            address,
            from_block: None,
            topics: vec![],
        }
    }
}

/// A filter on an EVM event topic is a list of possible matches for the topic.
type EvmEventTopicFilter = Vec<EvmEventTopic>;

/// Event filter builder
pub struct EvmEventFilterBuilder {
    address: EvmAddress,
    from_block: Option<U256>,
    topics: Vec<EvmEventTopicFilter>,
}

impl EvmEventFilterBuilder {
    /// Set earliest block to receive events from.
    pub fn filter_from_block(mut self, block_number: U256) -> Self {
        self.from_block = Some(block_number);
        self
    }

    /// Allow next topic to match on any value.
    pub fn any_match(mut self) -> Self {
        self.topics.push(vec![]);
        self
    }

    /// Next topic should have an exact match on the supplied value.
    pub fn exact_match(mut self, topic: EvmEventTopic) -> Self {
        self.topics.push(vec![topic]);
        self
    }

    /// Next topic can match on any one of the supplied values.
    pub fn one_of_match(mut self, topics: Vec<EvmEventTopic>) -> Self {
        self.topics.push(topics);
        self
    }

    /// Build the filter.
    pub fn build(self) -> EvmEventFilter {
        if self.topics.len() > 4 {
            panic!("Attempted to build EvmEventFilter with more than four topics")
        }
        EvmEventFilter {
            address: self.address,
            from_block: self.from_block.clone().unwrap_or(U256 { bytes: [0; 32] }),
            topics: self.topics,
        }
    }
}

/// Identifier for an EVM event subscription.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[repr(transparent)]
#[derive(PartialEq, Eq, ReadRPC, WriteRPC, Debug, Clone, Copy, CreateTypeSpecInternal)]
#[non_exhaustive]
pub struct EventSubscriptionId {
    raw_id: i32,
}

impl EventSubscriptionId {
    /// Creates new subscription id
    #[allow(dead_code)]
    pub(crate) const fn new(raw_id: i32) -> Self {
        Self { raw_id }
    }
}

/// A subscription to external EVM events.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[derive(Debug, ReadRPC, WriteRPC)]
#[non_exhaustive]
pub struct EventSubscription {
    /// The id of this event subscription.
    pub subscription_id: EventSubscriptionId,
    /// Whether the subscription is active or not.
    pub is_active: bool,
    /// Identifier of the chain that is subscribed to.
    pub chain_id: EvmChainId,
    /// Address of the contract that is subscribed to.
    pub contract_address: EvmAddress,
    /// Block number of the first block the subscription applies to.
    pub from_block: U256,
    /// Topics of the subscription.
    pub topics: Vec<EvmEventTopicFilter>,
}

/// Identifier for an EVM event.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[repr(transparent)]
#[derive(PartialEq, Eq, ReadRPC, WriteRPC, Debug, Clone, Copy, CreateTypeSpecInternal)]
#[non_exhaustive]
pub struct ExternalEventId {
    raw_id: i32,
}

impl ExternalEventId {
    /// Creates new event id
    #[allow(dead_code)]
    pub(crate) const fn new(raw_id: i32) -> Self {
        Self { raw_id }
    }
}

/// An external EVM event.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[derive(Debug, ReadRPC, WriteRPC)]
#[non_exhaustive]
pub struct ExternalEvent {
    /// The id of the event subscription.
    pub subscription_id: EventSubscriptionId,
    /// The id of this event.
    pub event_id: ExternalEventId,
    /// Data included in the log.
    pub data: Vec<u8>,
    /// Topics included in the log,
    pub topics: Vec<EvmEventTopic>,
}
