//! Definitions for RPC calls between contracts
//!
//! # Motivation
//!
//! Partisia Blockchain's contract interaction model sandboxes each contract, and allows RPC
//! calls as the primary form of interaction. As each transaction is entirely isolated, RPCs can
//! only occur "between" action calls.
//!
//! Abstractly, for example:
//!
//! - X calls Alice in transaction 1: Alice determine it needs some information from Bob, and exits
//!   while telling the blockchain: "Call Bob for me, I want a reply, and let me pay for the reply"
//! - Alice calls Bob in transaction 2: Bob performs it's computation, and exists with "Call Alice for
//!   me, she said she'd pay for this reply".
//! - Bob calls Alice in transaction 3: Alice got the necessary information to perform her own
//!   computation...
//!
//! # Implementation
//!
//! To accommodate the model, the SDK requires each `action` annotated function to return
//! a (possibly empty) `Vec` of `EventGroup`s, which represents the "Call X for me" information.
//!
//! Each `EventGroup` consists of one or more interactions (representing "Call X for me",) with the
//! possiblity of callbacks (representing "I want a reply".) All interactions in an `EventGroup`
//! shares gas costs uniformly.
//!
//! # Example usage
//!
//! ```rust
//! # use pbc_contract_common::events::*;
//! # use pbc_contract_common::address::{Address, AddressType, Shortname, ShortnameCallback};
//! # let token_contract = Address { address_type: AddressType::PublicContract, identifier: [0; 20] };
//! # let first_argument = 0u16;
//! # let second_argument = 0i32;
//! # let some_argument = 0i32;
//! # const SHORTNAME_CALLBACK: ShortnameCallback = ShortnameCallback::from_u32(0x55);
//! let SHORTNAME_TOKEN_TRANSFER: Shortname = Shortname::from_be_bytes(&[0x42]).unwrap();
//!
//! let mut e = EventGroup::builder();
//!
//! e.call(token_contract, SHORTNAME_TOKEN_TRANSFER)
//!  .argument(first_argument)
//!  .argument(second_argument)
//!  .with_cost(10000)
//!  .done();
//!
//! e.with_callback(SHORTNAME_CALLBACK)
//!  .argument(some_argument)
//!  .with_cost(9999)
//!  .done();
//!
//! let event_group: EventGroup = e.build();
//! ```

use pbc_traits::ReadWriteRPC;
use read_write_rpc_derive::ReadWriteRPC;

use crate::address::{Address, Shortname, ShortnameCallback};

/// Represents the gas cost of various interactions.
pub type GasCost = u64;

/// An interaction is what is sent from an event.
///
/// It consists of:
///
/// - `dest` - the address of the receiver
/// - `payload` - the raw payload to send to the receiver
/// - `from_contract` - whether to send the interaction from the contract or from the sender of the original transaction
/// - `cost` - the max cost of the interaction.
///
/// Serialized with the RPC format.
#[derive(ReadWriteRPC, Eq, PartialEq, Debug)]
pub struct Interaction {
    dest: Address,
    payload: Vec<u8>,
    from_original_sender: bool,
    cost: Option<GasCost>,
}

/// A callback is a simple interaction that is sent *after* all sent events have been processed
/// by a node on the chain.
///
/// - `payload` - the raw RPC you want to receive
/// - `cost` - the max cost of the callback. If set to `None` the max cost is automatically set from the remaining gas.
///
/// Serialized with the RPC format.
#[derive(ReadWriteRPC, Eq, PartialEq, Debug)]
pub struct Callback {
    payload: Vec<u8>,
    cost: Option<GasCost>,
}

/// The event group is a struct holding a list of events to send to other contracts and
/// an optional callback RPC.
///
/// See docs for `Interaction`.
///
/// Serialized with the RPC format.
#[derive(ReadWriteRPC, Eq, PartialEq, Debug)]
pub struct EventGroup {
    callback_payload: Option<Vec<u8>>,
    callback_cost: Option<GasCost>,
    events: Vec<Interaction>,
}

impl Default for EventGroup {
    #[allow(deprecated)]
    fn default() -> Self {
        EventGroup::new()
    }
}

impl EventGroup {
    /// Produce new [`EventGroupBuilder`] with zero interactions.
    #[deprecated(
        note = "prefer using builder pattern, as it is more flexible. See EventGroup::builder"
    )]
    pub fn new() -> EventGroup {
        EventGroup {
            events: Vec::new(),
            callback_payload: None,
            callback_cost: None,
        }
    }

    /// Retrieves new [`EventGroupBuilder`] for constructing an event group, using builder pattern.
    pub fn builder() -> EventGroupBuilder {
        EventGroupBuilder {
            callback: None,
            interactions: vec![],
        }
    }

    /// Send an interaction with this current contract as sender.
    ///
    /// Params:
    /// - `dest`: Address of contract to call.
    /// - `payload`: Payload for the recipient contract.
    /// - `cost`: How much gas to dedicate to the callback. If `None` the cost is automatically set
    ///   from the remaining gas.
    #[deprecated(
        note = "prefer using builder pattern, as it is more flexible. See EventGroup::builder, EventGroupBuilder::call"
    )]
    pub fn send_from_contract(&mut self, dest: &Address, payload: Vec<u8>, cost: Option<GasCost>) {
        self.events.push(Interaction {
            dest: *dest,
            payload,
            from_original_sender: false,
            cost,
        })
    }

    /// Send an interaction with the original sender as sender.
    ///
    /// Params:
    /// - `dest`: Address of contract to call.
    /// - `payload`: Payload for the recipient contract.
    /// - `cost`: How much gas to dedicate to the callback. If `None` the cost is automatically set
    ///   from the remaining gas.
    #[deprecated(
        note = "prefer using builder pattern, as it is more flexible. See EventGroup::builder, EventGroupBuilder::call"
    )]
    pub fn send_from_original_sender(
        &mut self,
        dest: &Address,
        payload: Vec<u8>,
        cost: Option<GasCost>,
    ) {
        self.events.push(Interaction {
            dest: *dest,
            payload,
            from_original_sender: true,
            cost,
        })
    }

    /// Register a callback on this event group.
    ///
    /// Params:
    /// - `payload`: Data to accompany the callback once it occurs.
    /// - `cost`: How much gas to dedicate to the callback. If `None` the cost is automatically set
    ///   from the remaining gas.
    #[deprecated(
        note = "prefer using builder pattern, as it is more flexible. See EventGroup::builder, EventGroupBuilder::with_callback"
    )]
    pub fn register_callback(&mut self, payload: Vec<u8>, cost: Option<GasCost>) {
        self.callback_payload = Some(payload);
        self.callback_cost = cost;
    }
}

/// Builder object for [`EventGroup`].
#[must_use]
#[derive(Debug)]
pub struct EventGroupBuilder {
    callback: Option<Callback>,
    interactions: Vec<Interaction>,
}

/// Builder object for [`Interaction`]; produced by [`EventGroupBuilder::call`].
#[must_use]
#[derive(Debug)]
pub struct InteractionBuilder<'a> {
    dest: Address,
    payload: Vec<u8>,
    from_original_sender: bool,
    cost: Option<GasCost>,
    parent: &'a mut EventGroupBuilder,
}

/// Builder object for [`Callback`]; produced by [`EventGroupBuilder::with_callback`].
#[must_use]
#[derive(Debug)]
pub struct CallbackBuilder<'a> {
    payload: Vec<u8>,
    cost: Option<GasCost>,
    parent: &'a mut EventGroupBuilder,
}

impl EventGroupBuilder {
    /// Register new call with the given event group. [`InteractionBuilder::done`] must
    /// be called on the produced [`InteractionBuilder`] for it to be added to this
    /// [`EventGroupBuilder`].
    pub fn call(&mut self, dest: Address, shortname: Shortname) -> InteractionBuilder {
        InteractionBuilder {
            dest,
            payload: shortname.bytes(),
            from_original_sender: false,
            cost: None,
            parent: self,
        }
    }

    /// Register new callback with the given event group.
    pub fn with_callback(&mut self, shortname: ShortnameCallback) -> CallbackBuilder {
        CallbackBuilder {
            payload: shortname.shortname.bytes(),
            cost: None,
            parent: self,
        }
    }

    /// Build new [`EventGroup`].
    pub fn build(self) -> EventGroup {
        // Determine callback attributes
        let (callback_payload, callback_cost) = match self.callback {
            Some(x) => (Some(x.payload), x.cost),
            None => (None, None),
        };

        //
        EventGroup {
            callback_payload,
            callback_cost,
            events: self.interactions,
        }
    }
}

impl InteractionBuilder<'_> {
    /// Register that interaction should be treated as coming from the sender of the interaction
    /// this code is running in.
    ///
    /// Idempotent.
    pub fn from_original_sender(mut self) -> Self {
        self.from_original_sender = true;
        self
    }

    /// Register new argument for this interaction. This argument is automatically serialized. It
    /// is the programmer's responsibility to check that this type is correct, wrt. target
    /// contract's ABI.
    ///
    /// Can be called repeatedly.
    pub fn argument<T: ReadWriteRPC>(mut self, arg: T) -> Self {
        arg.rpc_write_to(&mut self.payload).unwrap();
        self
    }

    /// Register that this interaction should cost at most this much gas.
    ///
    /// Idempotent.
    pub fn with_cost(mut self, cost: GasCost) -> Self {
        self.cost = Some(cost);
        self
    }

    /// Build interaction and add it to the [`EventGroupBuilder`].
    pub fn done(self) {
        self.parent.interactions.push(Interaction {
            dest: self.dest,
            payload: self.payload,
            from_original_sender: self.from_original_sender,
            cost: self.cost,
        })
    }
}

impl CallbackBuilder<'_> {
    /// Register new argument for this interaction. This argument is automatically serialized. It
    /// is the programmer's responsibility to check that this type is correct, wrt. target
    /// contract's ABI.
    ///
    /// Can be called repeatedly.
    pub fn argument<T: ReadWriteRPC>(mut self, arg: T) -> Self {
        arg.rpc_write_to(&mut self.payload).unwrap();
        self
    }

    /// Register that this interaction should cost at most this much gas.
    ///
    /// Idempotent.
    pub fn with_cost(mut self, cost: GasCost) -> Self {
        self.cost = Some(cost);
        self
    }

    /// Build interaction and add it to the [`EventGroupBuilder`].
    pub fn done(self) {
        self.parent.callback = Some(Callback {
            payload: self.payload,
            cost: self.cost,
        });
    }
}

#[cfg(test)]
#[path = "../unit_tests/event_serialization.rs"]
mod event_serialization;
