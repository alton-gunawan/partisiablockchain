use std::io::{Read, Write};

use pbc_traits::ReadWriteRPC;
use read_write_rpc_derive::ReadWriteRPC;

use crate::address::Address;
use crate::Hash;

/// The contract context encapsulates the blockchain state and relevant information
/// for the callee.
///
/// Serialized with the RPC format.
#[repr(C)]
#[derive(Eq, PartialEq, Debug, ReadWriteRPC)]
pub struct ContractContext {
    /// The address of the contract being called.
    pub contract_address: Address,

    /// The sender of the transaction.
    pub sender: Address,

    /// The block time.
    pub block_time: i64,

    /// The block production time in millis UTC.
    pub block_production_time: i64,

    /// The hash of the current transaction.
    pub current_transaction: Hash,

    /// The hash of the parent transaction, if available.
    pub original_transaction: Hash,
}

/// This is the additional context object that all callbacks receive as a parameter.
/// It includes the execution status of the transactions sent by the event that registered this function as a callback.
pub struct CallbackContext {
    /// Whether or not the callback was a success
    pub success: bool,
    /// The list of execution results for all the transactions spawned by the original event.
    /// These are sorted in sent order.
    pub results: Vec<ExecutionResult>,
}

/// Due to the implementation details of the code generation `rpc_read_from` is required for CallbackContext.
impl ReadWriteRPC for CallbackContext {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        let success = bool::rpc_read_from(reader);
        let results = ReadWriteRPC::rpc_read_from(reader);
        CallbackContext { success, results }
    }

    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.success.rpc_write_to(writer)?;
        self.results.rpc_write_to(writer)
    }
}

/// An execution result containing the succes flag of a transaction.
pub struct ExecutionResult {
    /// Denotes whether the transaction executed succesfully
    pub succeeded: bool,
}

/// Needed since this struct is nested in [`CallbackContext`].
impl ReadWriteRPC for ExecutionResult {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        let succeeded = bool::rpc_read_from(reader);

        ExecutionResult { succeeded }
    }

    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.succeeded.rpc_write_to(writer)
    }
}
