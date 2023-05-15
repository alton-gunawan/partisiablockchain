use crate::events::EventGroup;
use pbc_traits::WriteInt;
use pbc_traits::{ReadWriteState, WriteRPC};

use crate::zk;

fn write_u32_be_at_idx(buffer: &mut [u8], idx: usize, value: u32) -> std::io::Result<()> {
    let mut value_buffer = Vec::with_capacity(4);
    value_buffer.write_u32_be(value)?;
    buffer[idx..(4 + idx)].clone_from_slice(&value_buffer[..4]);
    Ok(())
}

mod result_section_type_id {
    pub const STATE: u8 = 0x01;
    pub const EVENTS: u8 = 0x02;
    pub const ZK_STATE_CHANGE: u8 = 0x11;
    pub const ZK_INPUT_DEF: u8 = 0x12;
}

/// PBC internal object for serializing results to buffer, in a format understood by the blockchain
/// binder. Wraps buffer data, providing easy section serialization methods.
///
/// **Contracts should not use this struct directly.**
///
/// Usage protocol:
/// - Initialize with [`new`](Self::new)
/// - Write sections, in order: (Calls are allowed to be absent.)
///   * [`write_events`](Self::write_events)
///   * [`write_state`](Self::write_state)
/// - Finalize with [`finalize_result_buffer`](Self::finalize_result_buffer)
#[non_exhaustive]
pub struct ContractResultBuffer {
    /// Stores the actual buffer data
    pub data: Vec<u8>,

    /// Stores section id of the next allowed section
    pub next_allowed_section_id: u8,
}

#[allow(clippy::new_without_default)]
impl ContractResultBuffer {
    /// Allocates a vector and writes the result tuple according to what the blockchain binder expects.
    ///
    /// This will only write the buffer itself, it will not forget it.
    ///
    /// Should be used in conjunction with [`Self::finalize_result_buffer`], which will place the buffer as
    /// expected by the blockchain binder, and produce some output to locate it.
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(10240);
        // Preallocate 4 bytes for the length of the rest of the result.
        data.write_u32_be(0).unwrap();

        Self {
            data,
            next_allowed_section_id: 0,
        }
    }

    /// Write section with format:
    ///
    /// ```ignore
    /// | id: u8 | len: u32 | data: $len bytes |
    /// ```
    ///
    /// Note that we don't know the length of bytes beforehand, so initially we insert a placeholder
    /// length of zero at the length position, and replace it later on.
    #[inline]
    fn write_section<F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>>(
        &mut self,
        section_id: u8,
        section_data_writer: F,
    ) -> std::io::Result<()> {
        // Check that this section id is allowed to be written
        assert!(self.next_allowed_section_id <= section_id, "Duplicated or incorrectly ordered sections. Tried to write section with id 0x{:02x}, but expected section id of at least 0x{:02x}", section_id, self.next_allowed_section_id );
        self.next_allowed_section_id = section_id + 1;

        // Write id
        self.data.write_u8(section_id)?;

        // Write placeholder length, and keep track of where we wrote it
        let buf_length_idx = self.data.len();
        self.data.write_u32_be(0)?;

        // Write section data, using the supplied function
        section_data_writer(&mut self.data)?;

        // Determine actual length of data, and replace the placeholder.
        let data_length = (self.data.len() - buf_length_idx - 4) as u32;
        write_u32_be_at_idx(&mut self.data, buf_length_idx, data_length)
    }

    /// Writes the state to the output buffer
    ///
    /// See [`Self`] documentation for order of operations.
    pub fn write_state<S: ReadWriteState>(&mut self, state: S) {
        if std::mem::size_of::<S>() == 0 {
            return;
        }
        self.write_section(result_section_type_id::STATE, |buf| {
            state.state_write_to(buf)
        })
        .unwrap();
    }

    /// Writes a vector of events to the output buffer.
    ///
    /// See [`Self`] documentation for order of operations.
    pub fn write_events(&mut self, events: Vec<EventGroup>) {
        if events.is_empty() {
            return;
        }
        self.write_section(result_section_type_id::EVENTS, |buf| {
            events.rpc_write_to(buf)
        })
        .unwrap();
    }

    /// Places [`Self`] as is expected by the blockchain, and produces a value so the blockchain
    /// can locate the buffer result.
    ///
    /// See [`Self`] documentation for order of operations.
    ///
    /// # Safety
    ///
    /// This writes the result and forgets the buffer so it should only be called
    /// as the last part of the transaction.
    pub unsafe fn finalize_result_buffer(self) -> u64 {
        let mut buf = self.data;

        // The buffer has preallocated an u64 for the length of the payload.
        let payload_len = (buf.len() - std::mem::size_of::<u32>()) as u32;

        let len_bytes = payload_len.to_be_bytes();
        for (i, byte) in len_bytes.iter().enumerate() {
            buf[i] = *byte;
        }

        let ptr = buf.as_ptr();

        std::mem::forget(buf);

        ptr as u64
    }

    /// Writes an instance of [`zk::ZkInputDef`] to the output buffer.
    pub fn write_zk_input_def_result<MetadataT: ReadWriteState>(
        &mut self,
        declaration: zk::ZkInputDef<MetadataT>,
    ) {
        self.write_section(result_section_type_id::ZK_INPUT_DEF, |buf| {
            declaration.rpc_write_to(buf)
        })
        .unwrap();
    }

    /// Writes a vector of [`zk::ZkStateChange`] to the output buffer.
    pub fn write_zk_state_change(&mut self, changes: Vec<zk::ZkStateChange>) {
        self.write_section(result_section_type_id::ZK_STATE_CHANGE, |buf| {
            changes.rpc_write_to(buf)
        })
        .unwrap();
    }
}
