use crate::abi::types::NamedTypeSpec;
use std::collections::BTreeMap;
use std::io::Write;
use std::slice::Iter;

use super::{ContractAbi, FnAbi};

/// Cast a raw function pointer to a: `fn(&BTreeMap<String, u8>) -> T`, for any T.
unsafe fn cast_pointer_unconditionally<T>(ptr: *const ()) -> unsafe fn(&BTreeMap<String, u8>) -> T {
    std::mem::transmute::<*const (), fn(&BTreeMap<String, u8>) -> T>(ptr)
}

/// Serialized with the ABI format.
pub type LookupTable<T> = unsafe fn(&BTreeMap<String, u8>) -> T;

/// Read a raw C-type array of u32 from memory interpreting all items as a function pointer
/// using `cast_pointer_unconditionally`.
unsafe fn read_fn_pointer_array<T>(len: u32, ptr: *const u32) -> Vec<LookupTable<T>> {
    let mut result = Vec::with_capacity(len as usize);
    for i in 0..len {
        let location = ptr.add(i as usize);
        let fn_ptr = std::ptr::read(location) as *const ();
        result.push(cast_pointer_unconditionally(fn_ptr));
    }
    result
}

unsafe fn find_state_index(state_name: String, types: &[NamedTypeSpec]) -> usize {
    let potential_state_indices: Vec<usize> = types
        .iter()
        .enumerate()
        .filter(|(_, type_abi)| type_abi.name == state_name)
        .map(|(idx, _)| idx)
        .collect();

    assert_eq!(
        potential_state_indices.len(),
        1,
        "More than one type named {state_name}",
    );

    potential_state_indices.first().cloned().unwrap()
}

/// Generates the ABI.
///
/// # Safety
///
/// This should only be run by the ABI generation tool.
#[allow(clippy::too_many_arguments)]
pub unsafe fn generate_abi(
    version_binder: [u8; 3],
    version_client: [u8; 3],
    state_name: String,
    fn_len: u32,
    fn_list_ptr: *const u32,
    ty_len: u32,
    ty_list_ptr: *const u32,
) -> u64 {
    let type_suppliers = read_fn_pointer_array::<Vec<NamedTypeSpec>>(ty_len, ty_list_ptr);

    let (lut, types) = generate_types(type_suppliers.iter());

    // Read FnAbi objects enriched with data from LUT
    // Read init
    let actions: Vec<FnAbi> = read_fn_pointer_array::<FnAbi>(fn_len, fn_list_ptr)
        .into_iter()
        .map(|fn_abi_closure| fn_abi_closure(&lut))
        .collect();

    // Determine state type
    let state_index = find_state_index(state_name, &types);
    let state_type = types.get(state_index).unwrap();

    let mut contract = ContractAbi::new(state_type.type_spec.to_vec());
    contract.actions(actions);
    contract.types(types);
    let abi_header_buffer = abi_header_bytes(version_binder, version_client);

    let mut output: Vec<u8> = Vec::new();
    output.write_all(&abi_header_buffer).unwrap();
    contract.serialize_abi(&mut output).unwrap();

    let length = output.len() as u64;
    let pointer = output.as_ptr() as u64;

    std::mem::forget(output);

    (length << 32) | pointer
}

/// Generates the types for the abi given a list of functions that generates the NamedTypeSpecs.
///
/// # Safety
///
/// This should only be run by the ABI generation tool.
pub unsafe fn generate_types(
    iter: Iter<LookupTable<Vec<NamedTypeSpec>>>,
) -> (BTreeMap<String, u8>, Vec<NamedTypeSpec>) {
    // Pass 1: construct the type index lookup table
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();
    let mut index = 0;
    for type_abi_fn in iter.clone() {
        let type_abis = type_abi_fn(&BTreeMap::new());
        for type_abi in type_abis {
            lut.insert(type_abi.type_identifier, index as u8);
            index += 1;
        }
    }

    // Pass 2: Construct enriched TypeAbi objects
    let types = iter.flat_map(|type_abi_fn| type_abi_fn(&lut)).collect();
    (lut, types)
}

/// Create a header for the given version
unsafe fn abi_header_bytes(version_binder: [u8; 3], version_client: [u8; 3]) -> [u8; 12] {
    let mut bytes = [0u8; 12];
    for (i, byte) in "PBCABI".as_bytes().iter().enumerate() {
        bytes[i] = *byte;
    }

    bytes[6] = version_binder[0];
    bytes[7] = version_binder[1];
    bytes[8] = version_binder[2];

    bytes[9] = version_client[0];
    bytes[10] = version_client[1];
    bytes[11] = version_client[2];

    bytes
}
