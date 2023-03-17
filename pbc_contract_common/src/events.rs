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
//! It is furthermore possible to provide some data to a calling interaction.
//! To provide said data, see the corresponding example usage down below. <br>
//! Note: Creating an `EventGroup` holding both a callback and some return data is not allowed and will fail.
//! # Example usage: Interaction with callback
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
//! # Example usage: Return data
//! ```rust
//! # use pbc_contract_common::address::Shortname;
//! # use pbc_contract_common::events::EventGroup;
//! let SHORTNAME_RETURN_SOME_DATA: Shortname = Shortname::from_be_bytes(&[0x43]).unwrap();
//!
//! let mut e = EventGroup::builder();
//!
//! e.return_data("Hello World".to_string());
//!
//! let event_group: EventGroup = e.build();
//! ```

use pbc_traits::WriteRPC;

use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;

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
#[derive(ReadRPC, WriteRPC, Eq, PartialEq, Debug)]
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
#[derive(ReadRPC, WriteRPC, Eq, PartialEq, Debug)]
pub struct Callback {
    payload: Vec<u8>,
    cost: Option<GasCost>,
}

/// Data to return to either a client or a contract.
///
/// - `data` - the raw RPC you want to return
/// - `cost` - the max cost of the callback. If set to `None` the max cost is automatically set from the remaining gas.
///
/// Serialized with the RPC format.
#[derive(ReadRPC, WriteRPC, Eq, PartialEq, Debug)]
pub struct ReturnData {
    data: Vec<u8>,
}

impl ReturnData {
    /// Getter for test assertions
    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

/// The event group is a struct holding a list of events to send to other contracts and
/// an optional callback RPC.
///
/// See docs for `Interaction`.
///
/// Serialized with the RPC format.
#[derive(ReadRPC, WriteRPC, Eq, PartialEq, Debug)]
pub struct EventGroup {
    callback_payload: Option<Vec<u8>>,
    callback_cost: Option<GasCost>,
    events: Vec<Interaction>,
    return_data: Option<ReturnData>,
}

impl Default for EventGroup {
    fn default() -> Self {
        EventGroup::builder().build()
    }
}

impl EventGroup {
    /// Retrieves new [`EventGroupBuilder`] for constructing an event group, using builder pattern.
    pub fn builder() -> EventGroupBuilder {
        EventGroupBuilder {
            callback: None,
            interactions: vec![],
            return_data: None,
        }
    }

    /// Getter for test assertions
    pub fn return_data(self) -> Option<ReturnData> {
        self.return_data
    }
}

/// Builder object for [`EventGroup`].
#[must_use]
#[derive(Debug)]
pub struct EventGroupBuilder {
    callback: Option<Callback>,
    interactions: Vec<Interaction>,
    return_data: Option<ReturnData>,
}

/// Builder object for [`Interaction`]; produced by [`EventGroupBuilder::call`].
#[must_use]
#[derive(Debug)]
pub struct InteractionBuilder<'a> {
    dest: Address,
    payload: Vec<u8>,
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
            cost: None,
            parent: self,
        }
    }

    /// Register a new call with no payload with this [`EventGroupBuilder`].
    /// This is used to ping a contract to check if it is alive.
    pub fn ping(&mut self, dest: Address, cost: Option<GasCost>) {
        self.interactions.push(Interaction {
            dest,
            payload: vec![],
            from_original_sender: false,
            cost,
        })
    }

    /// Register new callback with the given event group.
    pub fn with_callback(&mut self, shortname: ShortnameCallback) -> CallbackBuilder {
        CallbackBuilder {
            payload: shortname.shortname.bytes(),
            cost: None,
            parent: self,
        }
    }

    /// Register new return data with the given event group. Only a single type can be returned at a time.
    /// If called multiple times, overwrites previous return data.
    pub fn return_data<T: WriteRPC>(&mut self, return_value: T) {
        let mut buffer: Vec<u8> = vec![];
        return_value.rpc_write_to(&mut buffer).unwrap();
        self.return_data = Some(ReturnData { data: buffer });
    }

    /// Build new [`EventGroup`].
    pub fn build(self) -> EventGroup {
        // Determine callback attributes
        let (callback_payload, callback_cost) = match self.callback {
            Some(x) => {
                if self.interactions.is_empty() {
                    panic!("Attempted to build EventGroup with callback but no associated interactions")
                } else {
                    (Some(x.payload), x.cost)
                }
            }
            None => (None, None),
        };

        let return_data = match self.return_data {
            Some(data) => {
                if callback_payload.is_none() && callback_cost.is_none() {
                    Some(data)
                } else {
                    panic!("Attempted to build EventGroup with both callback and return data")
                }
            }
            None => None,
        };

        EventGroup {
            callback_payload,
            callback_cost,
            events: self.interactions,
            return_data,
        }
    }
}

impl InteractionBuilder<'_> {
    /// Register that interaction should be treated as coming from the sender of the interaction
    /// this code is running in. This functionality has been removed.
    ///
    #[deprecated(note = "Sending events from original sender is not supported.")]
    pub fn from_original_sender(self) -> Self {
        panic!("Sending events from original sender is not supported")
    }

    /// Register new argument for this interaction. This argument is automatically serialized. It
    /// is the programmer's responsibility to check that this type is correct, wrt. target
    /// contract's ABI.
    ///
    /// Can be called repeatedly.
    pub fn argument<T: WriteRPC>(mut self, arg: T) -> Self {
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
            from_original_sender: false, // disabled
            cost: self.cost,
        })
    }
}

impl CallbackBuilder<'_> {
    /// Register new argument for this callback. This argument is automatically serialized. It
    /// is the programmer's responsibility to check that this type is correct, wrt. target
    /// contract's ABI.
    ///
    /// Can be called repeatedly.
    pub fn argument<T: WriteRPC>(mut self, arg: T) -> Self {
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
