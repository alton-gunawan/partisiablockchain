//! Partisia Blockchain SDK Common Crate
//!
//! Defines common types and methods used in PBC smart contracts.
extern crate quote;

use pbc_traits::WriteInt;

pub use function_name::{FunctionKind, FunctionName};
use pbc_external::log_external;
use pbc_traits::{ReadWriteRPC, ReadWriteState};
#[cfg(feature = "abi")]
pub use raw_ptr::RawPtr;

use crate::events::EventGroup;

/// The ABI traits and glue code.
#[cfg(feature = "abi")]
pub mod abi;
/// The address module.
pub mod address;
/// The contract context module.
pub mod context;
pub mod events;
pub mod signature;
#[cfg(any(feature = "zk", doc))]
pub mod zk;

#[cfg(any(test, doc, feature = "test_examples"))]
pub mod test_examples;

mod function_name;
#[cfg(feature = "abi")]
mod raw_ptr;

/// The hash type is simply a 32 byte array
pub type Hash = [u8; 32];

/// The shortname for the init method of a contract.
const FN_INIT_SHORTNAME: u32 = 0xFFFFFFFF;

/// The shortname for the init method of a contract.
pub fn fn_init_shortname() -> address::Shortname {
    address::Shortname::from_u32(FN_INIT_SHORTNAME)
}

/// Write an ABI object to the given pointer.
/// This method uses unsafe code.
#[cfg(feature = "abi")]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn abi_to_ptr<T: ReadWriteState>(abi: T, pointer: *mut u8) -> u32 {
    let mut raw = RawPtr::new(pointer);
    abi.state_write_to(&mut raw).unwrap();
    raw.get_offset()
}

fn write_u32_be_at_idx(buffer: &mut [u8], idx: usize, value: u32) -> std::io::Result<()> {
    let mut value_buffer = Vec::with_capacity(4);
    value_buffer.write_u32_be(value)?;
    buffer[idx..(4 + idx)].clone_from_slice(&value_buffer[..4]);
    Ok(())
}

mod result_section_type_id {
    pub const STATE: u8 = 0x01;
    pub const EVENTS: u8 = 0x02;

    #[cfg(feature = "zk")]
    pub const ZK_STATE_CHANGE: u8 = 0x11;
    #[cfg(feature = "zk")]
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
    /// | id: u32 | len: u32 | data: $len bytes |
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
    #[cfg(any(feature = "zk", doc))]
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
    #[cfg(any(feature = "zk", doc))]
    pub fn write_zk_state_change(&mut self, changes: Vec<zk::ZkStateChange>) {
        self.write_section(result_section_type_id::ZK_STATE_CHANGE, |buf| {
            changes.rpc_write_to(buf)
        })
        .unwrap();
    }
}

fn raw_log(message: &str) {
    let string = message.to_string();
    let len = string.len();
    unsafe {
        log_external(string.as_ptr() as i64, len as i32);
    }
}

/// Log a message to the blockchain standard out.
pub fn info(string: String) {
    raw_log(&string);
}

/// Encode unsigned 32-bit ints as LEB128.
pub fn to_leb128_bytes(mut value: u32) -> Vec<u8> {
    if value == 0 {
        return vec![0];
    }

    let mut result = Vec::new();
    while value != 0 {
        let lower_seven = value & 0x7f;
        value >>= 7;

        let high_bit = if value != 0 { 0x80 } else { 0 };
        result.push((lower_seven | high_bit) as u8);
    }
    result
}

#[cfg(test)]
mod test {
    use crate::to_leb128_bytes;

    #[test]
    fn leb() {
        assert_eq!(to_leb128_bytes(0), vec![0]);
        assert_eq!(to_leb128_bytes(1), vec![1]);
        assert_eq!(to_leb128_bytes(65), vec![65]);
        assert_eq!(to_leb128_bytes(127), vec![127]);
        assert_eq!(to_leb128_bytes(128), vec![128, 1]);
        assert_eq!(to_leb128_bytes(192), vec![192, 1]);
        assert_eq!(to_leb128_bytes(255), vec![255, 1]);
        assert_eq!(to_leb128_bytes(256), vec![128, 2]);
        assert_eq!(to_leb128_bytes(624485), vec![0xE5, 0x8E, 0x26]);
        assert_eq!(
            to_leb128_bytes(0xFFFFFFFF),
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]
        );
    }
}
