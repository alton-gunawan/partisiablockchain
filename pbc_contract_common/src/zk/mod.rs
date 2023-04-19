//! Contains definitions for use in Zk (Zero-Knowledge) Contracts
//!
//! These should be used in conjunction with the Zk macros in `pbc_contract_codegen`.

use std::io::{Read, Write};

use create_type_spec_derive::CreateTypeSpecInternal;
use pbc_traits::WriteInt;
use pbc_traits::{ReadRPC, ReadWriteState, WriteRPC};
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::address::Address;
use crate::signature::Signature;

/// Identifier for a secret variable.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[repr(transparent)]
#[derive(
    PartialEq, Eq, ReadRPC, WriteRPC, ReadWriteState, Debug, Clone, Copy, CreateTypeSpecInternal,
)]
#[non_exhaustive]
pub struct SecretVarId {
    raw_id: u32,
}

impl SecretVarId {
    /// Creates new secret var id
    pub const fn new(raw_id: u32) -> Self {
        Self { raw_id }
    }
}

/// Identifier for a secret input (variable).
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
type SecretInputId = SecretVarId;

/// Identifier for an attested piece of data.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[repr(transparent)]
#[derive(PartialEq, Eq, ReadRPC, WriteRPC, Debug, Clone, Copy)]
#[non_exhaustive]
pub struct AttestationId {
    raw_id: u32,
}

impl AttestationId {
    /// Creates new attestation id
    #[allow(dead_code)]
    pub(crate) const fn new(raw_id: u32) -> Self {
        Self { raw_id }
    }
}

/// An attested piece of data.
///
/// # Invariants
///
/// Cannot be manually created; must be retrieved from state.
#[derive(Debug, ReadRPC, WriteRPC)]
#[non_exhaustive]
pub struct DataAttestation {
    /// The id of this data attestation.
    pub attestation_id: AttestationId,
    /// Signatures that have attested for this data.
    pub signatures: Vec<Signature>,
    /// The attested data itself.
    pub data: Vec<u8>,
}

/// Status of the associated ZK computation. Life cycle:
///
/// - [`Waiting`](Self::Waiting) => [`Calculating`](Self::Calculating) (by [`ZkStateChange::StartComputation`])
/// - [`Calculating`](Self::Calculating) => [`Output`](Self::Output) (automatically)
/// - [`Calculating`](Self::Calculating) => [`MaliciousBehaviour`](Self::MaliciousBehaviour) (automatically)
/// - [`Output`](Self::Output) => [`Waiting`](Self::Waiting) (by [`ZkStateChange::OutputComplete`])
/// - [`Output`](Self::Output) => [`Done`](Self::Done) (by [`ZkStateChange::ContractDone`])
///
/// Note: It is only possible to use [`ZkStateChange::ContractDone`] when in [`Output`](Self::Output).
/// It is not possible "to change your mind", once a [`ZkStateChange::OutputComplete`] action have
/// been sent, and an entirely new Zk computation must be completed.
///
/// Cannot be manually created; must be retrieved from state.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, ReadRPC, WriteRPC)]
pub enum CalculationStatus {
    /// Nodes are idling, ready for input. Must be manually moved to [`Calculating`](Self::Calculating) with the
    /// [`ZkStateChange::StartComputation`] action.
    Waiting = 0,

    /// Nodes are performing computations, due to a previous [`ZkStateChange::StartComputation`] action.
    ///
    /// When completed, the contract enters the commitment phase. The commitment phase enables fair
    /// open, i.e. all nodes agree on the output since their commitments match (in a replicated
    /// sharing). After checking the commitments the contract enters the output phase where the nodes send
    /// their output.
    ///
    /// The ZK nodes will automatically transition to either [`Output`](Self::Output) or [`MaliciousBehaviour`](Self::MaliciousBehaviour)
    /// after haveing been transitioned to [`Calculating`](Self::Calculating).
    Calculating = 1,

    /// Notes are done with computation, having commited to computation output on the chain. This
    /// phase allows opening variables on chain.
    Output = 2,

    /// Some part of the protocol isn't done correctly; acts as an error case.
    MaliciousBehaviour = 3,

    /// MVP is complete, and nodes can be released.
    Done = 4,
}

/// Stores public information about a secret variable.
///
/// - `<MetadataT>`: Additional data stored along with each variable.
#[repr(C)]
#[derive(Debug)]
#[non_exhaustive]
pub struct ZkClosed<MetadataT: ReadWriteState> {
    /// Id of the secret variable
    pub variable_id: SecretVarId,
    /// Which address owns the variable
    pub owner: Address,
    /// Whether the variable is sealed
    pub is_sealed: bool,
    /// Customizable metadata.
    pub metadata: MetadataT,
    /// Data, but only if published
    pub data: Option<Vec<u8>>,
}

impl<MetadataT: ReadWriteState> ReadRPC for ZkClosed<MetadataT> {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        Self {
            variable_id: SecretVarId::rpc_read_from(reader),
            owner: Address::rpc_read_from(reader),
            is_sealed: bool::rpc_read_from(reader),
            metadata: MetadataT::state_read_from(reader),
            data: <Option<Vec<u8>>>::rpc_read_from(reader),
        }
    }
}

impl<MetadataT: ReadWriteState> WriteRPC for ZkClosed<MetadataT> {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.variable_id.rpc_write_to(writer)?;
        self.owner.rpc_write_to(writer)?;
        self.is_sealed.rpc_write_to(writer)?;
        self.metadata.state_write_to(writer)?;
        self.data.rpc_write_to(writer)
    }
}

/// Stores and tracks state changes
///
/// - `<SecretMetadataT>`: Public state stored along with each variable.
#[repr(C)]
#[derive(Debug, ReadRPC, WriteRPC)]
#[non_exhaustive]
pub struct ZkState<SecretVarMetadataT: ReadWriteState> {
    /// The MPC's current state.
    pub calculation_state: CalculationStatus,
    /// Variables that are in the process of being input.
    pub pending_inputs: Vec<ZkClosed<SecretVarMetadataT>>,
    /// Secret variables that have been commited to.
    pub secret_variables: Vec<ZkClosed<SecretVarMetadataT>>,
    /// Attested data
    pub data_attestations: Vec<DataAttestation>,
    /// Reserved 1
    pub reserved_1: u32,
    /// Reserved 2
    pub reserved_2: u32,
}

impl<SecretVarMetadataT: ReadWriteState> ZkState<SecretVarMetadataT> {
    /// Utility method for finding pending input with given id
    pub fn get_pending_input(&self, id: SecretInputId) -> Option<&ZkClosed<SecretVarMetadataT>> {
        self.pending_inputs.iter().find(|x| x.variable_id == id)
    }

    /// Utility method for finding input with given id.
    pub fn get_pending_inputs_for(&self, owner: Address) -> Vec<&ZkClosed<SecretVarMetadataT>> {
        self.pending_inputs
            .iter()
            .filter(|x| x.owner == owner)
            .collect()
    }

    /// Utility method for finding variable with given id
    pub fn get_variable(&self, id: SecretVarId) -> Option<&ZkClosed<SecretVarMetadataT>> {
        self.secret_variables.iter().find(|x| x.variable_id == id)
    }

    /// Utility method for finding variables for the given owner
    pub fn get_variables_for(&self, owner: Address) -> Vec<&ZkClosed<SecretVarMetadataT>> {
        self.secret_variables
            .iter()
            .filter(|x| x.owner == owner)
            .collect()
    }

    /// Utility method for finding attestation by attestation id
    pub fn get_attestation(&self, attestation_id: AttestationId) -> Option<&DataAttestation> {
        self.data_attestations
            .iter()
            .find(|x| x.attestation_id == attestation_id)
    }
}

/// Contains initialization information about Zk variables. Exclusively needed for the
/// `zk_on_secret_input` hook.
///
/// `<MetadataT>` is the type of the piece of public information associated with the variable.
#[repr(C)]
#[derive(Debug)]
pub struct ZkInputDef<MetadataT: ReadWriteState> {
    /// The bit lengths expected of the variable, and the number of subvariables wanted.
    pub expected_bit_lengths: Vec<u32>,
    /// Whether or not the variable should be sealed.
    pub seal: bool,
    /// A piece of public contract-defined information associated with each variable.
    pub metadata: MetadataT,
}

impl<MetadataT: ReadWriteState> ReadRPC for ZkInputDef<MetadataT> {
    fn rpc_read_from<T: Read>(reader: &mut T) -> Self {
        Self {
            expected_bit_lengths: <_>::rpc_read_from(reader),
            seal: <_>::rpc_read_from(reader),
            metadata: <_>::state_read_from(reader),
        }
    }
}

impl<MetadataT: ReadWriteState> WriteRPC for ZkInputDef<MetadataT> {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.expected_bit_lengths.rpc_write_to(writer)?;
        self.seal.rpc_write_to(writer)?;
        self.metadata.state_write_to(writer)
    }
}

/// Represents individual state changes in the ZkState.
#[derive(Debug)]
pub enum ZkStateChange {
    /// Starts Zk computation.
    ///
    /// The direct constructor is cumbersome, use [`ZkStateChange::start_computation`] instead.
    ///
    /// # Invariants
    ///
    /// - Must only occur when [`ZkState::calculation_state`] is [`CalculationStatus::Waiting`].
    /// - Only one [`StartComputation`](Self::StartComputation) is allowed per transaction.
    #[non_exhaustive]
    StartComputation {
        /// Metadata associated which each output variable. Assumes each piece of metadata have
        /// been serialized manually.
        output_variable_metadata: Vec<Vec<u8>>,
        /// Public variables to be given to the ZK computation, as function inputs. Assumes each piece of metadata have
        /// been serialized manually.
        input_arguments: Vec<Vec<u8>>,
    },

    /// Deletes pending input for the current user.
    ///
    /// # Invariants
    /// - Variable must not be open.
    /// - Variable must be owned by contract caller.
    /// - Can occur in any [`ZkState::calculation_state`].
    DeletePendingInput {
        /// Input that should be deleted.
        variable: SecretInputId,
    },

    /// Transfers variable owned by current user to [`new_owner`](ZkStateChange::TransferVariable::new_owner).
    ///
    /// # Invariants
    /// - Variable must not be open.
    /// - Variable must be owned by contract caller.
    /// - Can occur in any [`ZkState::calculation_state`].
    TransferVariable {
        /// Variable to transfer
        variable: SecretVarId,
        /// New owner of variable
        new_owner: Address,
    },

    /// Deletes given contract variable.
    ///
    /// # Invariants
    /// - Variable must not be open.
    /// - Variable must be owned by contract caller.
    /// - Can occur in any [`ZkState::calculation_state`].
    DeleteVariable {
        /// Variable to delete
        variable: SecretVarId,
    },

    /// Opens variables for the current user.
    ///
    /// # Invariants
    /// - Variables must not be open.
    /// - Variables must be owned by contract caller.
    /// - Must only occur when [`ZkState::calculation_state`] is [`CalculationStatus::Output`].
    /// - There must be no pending inputs.
    OpenVariables {
        /// Variables that should be opened
        variables: Vec<SecretVarId>,
    },

    /// Changes [`ZkState::calculation_state`](ZkState) back to [`CalculationStatus::Waiting`], deleting any given variables.
    ///
    /// # Invariants
    /// - All variables are allowed, including user variables and contract variables.
    /// - Must only occur when [`ZkState::calculation_state`] is [`CalculationStatus::Output`]
    OutputComplete {
        /// Variables that should be deleted
        variables_to_delete: Vec<SecretVarId>,
    },

    /// Closes ZK computation; no further zero-knowledge can be done.
    ///
    /// # Invariants
    /// Must only occur when [`ZkState::calculation_state`] is [`CalculationStatus::Output`].
    ContractDone,

    /// Requests ZK nodes to sign/attest this piece of data. This can occur at any time.
    Attest {
        /// The piece of data to attest.
        data_to_attest: Vec<u8>,
    },
}

impl ZkStateChange {
    /// Convenience function for creating instances of [`Self::StartComputation`], automatically
    /// serializing metadata.
    ///
    /// Arguments:
    /// - `output_variable_metadata`: Vector of pieces of metadata to associate with each output
    ///   variable.
    ///
    /// # Invariants
    /// - The argument `output_variable_metadata` must have the same number of elements as is
    ///   outputted by the zk computation.
    pub fn start_computation<T: ReadWriteState>(output_variable_metadata: Vec<T>) -> Self {
        ZkStateChange::start_computation_with_inputs::<T, bool>(output_variable_metadata, vec![])
    }

    /// Convenience function for creating instances of [`Self::StartComputation`], automatically
    /// serializing metadata.
    ///
    /// Arguments:
    /// - `output_variable_metadata`: Vector of pieces of metadata to associate with each output
    ///   variable.
    /// - `input_arguments`: Vector of pieces of public input to be given to the ZK computation.
    ///
    /// # Invariants
    /// - The argument `output_variable_metadata` must have the same number of elements as is
    ///   outputted by the zk computation.
    /// - The argument `input_arguments` must have the same number of elements as the ZK
    ///   computation have input arguments, and these must be of the same types.
    pub fn start_computation_with_inputs<T: ReadWriteState, A: ReadWriteState>(
        output_variable_metadata: Vec<T>,
        input_arguments: Vec<A>,
    ) -> Self {
        let output_variable_metadata: Vec<Vec<u8>> = output_variable_metadata
            .iter()
            .map(|x| {
                let mut buf = Vec::new();
                x.state_write_to(&mut buf).unwrap();
                buf
            })
            .collect();

        let input_arguments = input_arguments
            .iter()
            .map(|x| {
                let mut buf = Vec::new();
                x.state_write_to(&mut buf).unwrap();
                buf
            })
            .collect();

        Self::StartComputation {
            output_variable_metadata,
            input_arguments,
        }
    }

    const DISCRIMINANT_DELETE_PENDING_VARIABLE: u8 = 0x01;
    const DISCRIMINANT_TRANSFER_VARIABLE: u8 = 0x02;
    const DISCRIMINANT_DELETE_VARIABLE: u8 = 0x03;
    const DISCRIMINANT_OPEN_VARIABLES: u8 = 0x04;
    const DISCRIMINANT_OUTPUT_COMPLETE: u8 = 0x05;
    const DISCRIMINANT_CONTRACT_DONE: u8 = 0x06;
    const DISCRIMINANT_ATTEST: u8 = 0x07;
    const DISCRIMINANT_START_COMPUTATION_WITH_INPUTS: u8 = 0x08;
}

impl WriteRPC for ZkStateChange {
    fn rpc_write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        match self {
            Self::StartComputation {
                output_variable_metadata,
                input_arguments,
            } => {
                writer.write_u8(Self::DISCRIMINANT_START_COMPUTATION_WITH_INPUTS)?;
                output_variable_metadata.rpc_write_to(writer)?;
                input_arguments.rpc_write_to(writer)
            }
            Self::DeletePendingInput { variable } => {
                writer.write_u8(Self::DISCRIMINANT_DELETE_PENDING_VARIABLE)?;
                variable.rpc_write_to(writer)
            }
            Self::TransferVariable {
                variable,
                new_owner,
            } => {
                writer.write_u8(Self::DISCRIMINANT_TRANSFER_VARIABLE)?;
                variable.rpc_write_to(writer)?;
                new_owner.rpc_write_to(writer)
            }
            Self::DeleteVariable { variable } => {
                writer.write_u8(Self::DISCRIMINANT_DELETE_VARIABLE)?;
                variable.rpc_write_to(writer)
            }
            Self::OpenVariables { variables } => {
                writer.write_u8(Self::DISCRIMINANT_OPEN_VARIABLES)?;
                variables.rpc_write_to(writer)
            }
            Self::OutputComplete {
                variables_to_delete,
            } => {
                writer.write_u8(Self::DISCRIMINANT_OUTPUT_COMPLETE)?;
                variables_to_delete.rpc_write_to(writer)
            }
            Self::ContractDone => writer.write_u8(Self::DISCRIMINANT_CONTRACT_DONE),
            Self::Attest { data_to_attest } => {
                writer.write_u8(Self::DISCRIMINANT_ATTEST)?;
                data_to_attest.rpc_write_to(writer)
            }
        }
    }
}
