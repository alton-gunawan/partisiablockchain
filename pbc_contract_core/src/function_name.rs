//! Function name utility.

use sha2::{Digest, Sha256};

#[cfg(feature = "abi")]
use pbc_traits::WriteRPC;

#[cfg(feature = "abi")]
use read_write_rpc_derive::ReadRPC;
#[cfg(feature = "abi")]
use read_write_rpc_derive::WriteRPC;

#[cfg(feature = "abi")]
use crate::abi::AbiSerialize;

use crate::shortname::Shortname;

/// A small struct that automatically calculates the shortname of a function.
///
/// Serialized with the ABI format.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "abi", derive(ReadRPC, WriteRPC))]
pub struct FunctionName {
    name: String,
    shortname: Shortname,
}

impl FunctionName {
    /// Create a new instance with the specified name. The shortname is calculated if None is
    /// supplied.
    pub fn new(name: String, shortname: Option<Shortname>) -> FunctionName {
        FunctionName::create_from_str(&name, shortname)
    }

    /// Create a new instance with the specified name as a str. The shortname is calculated eagerly.
    pub fn create_from_str(name: &str, shortname_override: Option<Shortname>) -> FunctionName {
        let shortname = if let Some(value) = shortname_override {
            value
        } else {
            name_to_shortname(name)
        };

        FunctionName {
            name: name.to_string(),
            shortname,
        }
    }

    /// Gets the Shortname
    pub fn shortname(&self) -> &Shortname {
        &self.shortname
    }
}

/// Denotes the kind of the ABI function hook.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[cfg_attr(feature = "abi", derive(ReadRPC, WriteRPC))]
#[repr(u8)]
pub enum FunctionKind {
    /// Kind for `init` hook.
    Init = 0x01,
    /// Kind for `action` hook.
    Action = 0x02,
    /// Kind for `callback` hook.
    Callback = 0x03,
    /// Kind for `zk_on_secret_input` hook.
    #[deprecated(note = "Use ZkSecretInputWithExplicitType instead")]
    ZkSecretInput = 0x10,
    /// Kind for `zk_on_variable_inputted` hook.
    ZkVarInputted = 0x11,
    /// Kind for `zk_on_variable_rejected` hook.
    ZkVarRejected = 0x12,
    /// Kind for `zk_on_compute_complete` hook.
    ZkComputeComplete = 0x13,
    /// Kind for `zk_on_variable_opened` hook.
    ZkVarOpened = 0x14,
    /// Kind for `zk_on_user_variable_opened` hook.
    ZkUserVarOpened = 0x15,
    /// Kind for `zk_on_attestation_complete` hook.
    ZkAttestationComplete = 0x16,
    /// Kind for `zk_on_secret_input` hook.
    ZkSecretInputWithExplicitType = 0x17,
}

#[cfg(feature = "abi")]
impl AbiSerialize for FunctionName {
    fn serialize_abi<T: std::io::Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.name.rpc_write_to(writer)?;
        self.shortname.rpc_write_to(writer)
    }
}

/// Create a shortname from the given function name.
/// The shortname consists of the first 4 bytes of the SHA256 hash of the name.
fn name_to_shortname(raw_name: &str) -> Shortname {
    let mut digest = Sha256::new();
    Digest::update(&mut digest, raw_name.as_bytes());
    let output = digest.finalize();
    let first_four = output.chunks(4).next().unwrap();
    let shortname_u32 = u32::from_be_bytes(first_four.try_into().unwrap());
    Shortname::from_u32(shortname_u32)
}

#[cfg(all(test, feature = "abi"))]
mod test_abi_serialization {
    use super::FunctionName;
    use crate::shortname::Shortname;

    fn interesting_shortname_values() -> Vec<(u32, Vec<u8>)> {
        vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7F]),
            (128, vec![0x80, 0x01]),
            (256, vec![0x80, 0x02]),
            (1000, vec![0xe8, 0x07]),
            (586977299, vec![0x93, 0xA0, 0xF2, 0x97, 0x02]),
        ]
    }

    #[test]
    fn shortname_as_u32() {
        for (i, shortname_bytes) in interesting_shortname_values() {
            let shortname = Shortname::from_be_bytes(&shortname_bytes).unwrap();
            assert_eq!(Shortname::from_u32(i), shortname);
            assert_eq!(shortname.as_u32(), i);
        }
    }

    #[test]
    fn u32_as_shortname_as_u32() {
        for (i, _) in interesting_shortname_values() {
            let shortname = Shortname::from_u32(i);
            assert_eq!(i, shortname.as_u32());
        }
    }

    #[test]
    fn u32_as_shortname_bytes_as_u32() {
        for (shortname_value, shortname_bytes) in interesting_shortname_values() {
            let parsed = Shortname::from_be_bytes(&shortname_bytes).unwrap();
            assert_eq!(shortname_bytes, parsed.bytes());
            assert_eq!(shortname_value, parsed.as_u32());
        }
    }

    #[test]
    fn invalid_shortnames() {
        let invalid_shortname_bytes = [
            vec![0x00, 0x00],
            vec![0x00, 0x01],
            vec![0x00, 0x7F],
            vec![0x00, 0x80, 0x01],
            vec![0x80, 0x02, 0x00],
            vec![0x80],
            vec![0x80, 0x00], // Technically valid LEB128, but not normalized
            vec![0x80, 0x80, 0x80, 0x80, 0x32], // Too large for u32
            vec![0x93, 0xA0, 0xF2, 0x97, 0x32], // Too large for u32
        ];
        for bytes in invalid_shortname_bytes {
            let result = Shortname::from_be_bytes(&bytes);
            assert!(result.is_err(), "Must not succeed for bytes: {:?}", &bytes);
        }
    }

    #[test]
    fn function_name_new_shortname() {
        let func_name = FunctionName::new("on_erc721_received".to_string(), None);
        let expected_bytes: Vec<u8> = vec![0x83, 0x81, 0xAE, 0xCB, 0x0D];
        assert_eq!(func_name.shortname.bytes(), expected_bytes);
        let other_shortname = Shortname::from_u32(0);
        let func_name2 = FunctionName::new("on_erc721_received".to_string(), Some(other_shortname));
        let expected_shortname = Shortname::from_u32(0);
        assert_eq!(func_name2.shortname, expected_shortname);
    }
}
