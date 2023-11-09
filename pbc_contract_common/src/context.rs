//! Definitions for commonly used [`ContractContext`] and [`CallbackContext`].
//!
//! The contract contexts contains information about when and why the current invocation is being
//! executed.
//!
//! - [`ContractContext`] is used by all invocations to determine the what and why of the current
//! invocation.
//! - [`CallbackContext`] is used by `#[callback]` invocations to indicate whether the invocation
//! that triggered the callback succeeded, or if whether it resulted in an error. Also includes
//! return data if applicable.

use std::io::{Read, Write};

use pbc_traits::ReadRPC;
use pbc_traits::WriteRPC;
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;

use crate::address::Address;
use crate::Hash;

/// The functional and temporal context that contract invocations are called in.
///
/// Can be through of as the when and why of the current invocation.
///
/// Contains information on the caller, the current time, and various PBC specific informations,
/// like the hash of the transaction, or the hash of the parent transaction.
#[repr(C)]
#[derive(Eq, PartialEq, Debug, ReadRPC, WriteRPC)]
pub struct ContractContext {
    /// The address of the contract being called.
    ///
    /// Primary way for the contract to determine its own address. Will never change for any
    /// specific contract deployment.
    pub contract_address: Address,

    /// The sender of the event that resulted in the currently running invocation.
    ///
    /// In `#[init]` invocations this will be the creator of the contract, and in other invocations
    /// it will be users of the contract, possibly a contract if an invocation have occured.
    pub sender: Address,

    /// The monotonically rising block height of the block this invocation is executed in. Counts the number of blocks that have been produced since genesis.
    ///
    /// Should not be used for time-outs or similar system, as block production periods are
    /// highly variadic, due to being produced on-demand, and being easily manipulated by block
    /// producers.
    pub block_time: i64,

    /// The [Unix time](https://en.wikipedia.org/wiki/Unix_time) in milliseconds of the block this invocation is executed in. Monotonically rising.
    ///
    /// Preferred for time-outs, but beware that block producers may manipulate the value slightly.
    pub block_production_time: i64,

    /// The hash of the event that spawned this invocation.
    ///
    /// Guarenteed unique between all invocations.
    pub current_transaction: Hash,

    /// The hash of the signed transaction that eventually resulted in the creation of this invocation.
    ///
    /// The most important use of this field is to determine the address of a deployed contract.
    pub original_transaction: Hash,
}

/// Additional context for `#[callback]` invocations, indicating the success status and result
/// values of the invocation.
///
/// It includes the [`ExecutionResult`] of the transactions sent by the event that registered this function as a callback.
pub struct CallbackContext {
    /// Whether or not the callback was a success
    pub success: bool,
    /// The list of execution results for all the transactions spawned by the original event.
    /// These are sorted in sent order.
    pub results: Vec<ExecutionResult>,
}

/// Due to the implementation details of the code generation `rpc_read_from` is required for CallbackContext.
impl ReadRPC for CallbackContext {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        let success = bool::rpc_read_from(reader);
        let results = ReadRPC::rpc_read_from(reader);
        CallbackContext { success, results }
    }
}

/// Due to the implementation details of the code generation `rpc_read_from` is required for CallbackContext.
impl WriteRPC for CallbackContext {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.success.rpc_write_to(writer)?;
        self.results.rpc_write_to(writer)
    }
}

/// Execution result of a single event from an event group.
///
/// Primarily retrieved from [`CallbackContext`].
pub struct ExecutionResult {
    /// Denotes whether the event executed successfully, without panicking.
    pub succeeded: bool,
    /// The serialized return data.
    ///
    /// Attached to the event by the called contract using
    /// [`EventGroupBuilder::return_data`](crate::events::EventGroupBuilder::return_data). Use
    /// [`get_return_data`](Self::get_return_data) to deserialize to a specific type.
    pub return_data: Vec<u8>,
}

impl ExecutionResult {
    /// Deserialize the return data to a specific type, using [`ReadRPC`].
    ///
    /// Utility method to avoid the need for manual deserialization over
    /// [`ExecutionResult::return_data`].
    pub fn get_return_data<T: ReadRPC>(&self) -> T {
        let mut return_data_bytes: &[u8] = self.return_data.as_slice();
        T::rpc_read_from(&mut return_data_bytes)
    }
}

/// Needed since this struct is nested in [`CallbackContext`].
impl ReadRPC for ExecutionResult {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        let succeeded = bool::rpc_read_from::<T>(reader);
        let return_data = Vec::<u8>::rpc_read_from::<T>(reader);

        ExecutionResult {
            succeeded,
            return_data,
        }
    }
}

/// Needed since this struct is nested in [`CallbackContext`].
impl WriteRPC for ExecutionResult {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.succeeded.rpc_write_to(writer)?;
        self.return_data.rpc_write_to(writer)
    }
}
