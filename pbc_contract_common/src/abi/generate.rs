use std::collections::BTreeMap;
use std::io::Write;

use crate::abi::{ContractAbi, FnAbi, TypeAbi};

/// Cast a raw function pointer to a: `fn(&BTreeMap<String, u8>) -> T`, for any T.
unsafe fn cast_pointer_unconditionally<T>(ptr: *const ()) -> unsafe fn(&BTreeMap<String, u8>) -> T {
    std::mem::transmute::<*const (), fn(&BTreeMap<String, u8>) -> T>(ptr)
}

/// Serialized with the ABI format.
type LookupTable<T> = unsafe fn(&BTreeMap<String, u8>) -> T;

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

unsafe fn find_state_index(state_name: String, types: &[TypeAbi]) -> usize {
    let potential_state_indices: Vec<usize> = types
        .iter()
        .enumerate()
        .filter(|(_, type_abi)| type_abi.name == state_name)
        .map(|(idx, _)| idx)
        .collect();

    assert_eq!(
        potential_state_indices.len(),
        1,
        "More than one type named {}",
        state_name
    );

    potential_state_indices.first().cloned().unwrap()
}

/// Generates the ABI.
///
/// # Safety
///
/// This should only be run by the ABI generation tool.
pub unsafe fn generate_abi(
    version_binder: [u8; 3],
    version_client: [u8; 3],
    state_name: String,
    fn_len: u32,
    fn_list_ptr: *const u32,
    ty_len: u32,
    ty_list_ptr: *const u32,
) -> u64 {
    let mut lut: BTreeMap<String, u8> = BTreeMap::new();

    let type_suppliers = read_fn_pointer_array::<TypeAbi>(ty_len, ty_list_ptr);
    // Pass 1: construct the type index lookup table
    for (index, type_abi_fn) in type_suppliers.clone().into_iter().enumerate() {
        let type_abi = type_abi_fn(&BTreeMap::new());
        lut.insert(type_abi.type_identifier.clone(), index as u8);
    }

    // Pass 2: Construct enriched TypeAbi objects
    let types: Vec<TypeAbi> = type_suppliers
        .into_iter()
        .map(|type_abi_fn| type_abi_fn(&lut))
        .collect();

    // Pass 2: Read FnAbi objects enriched with data from LUT
    // Pass 2: Read init
    let actions: Vec<FnAbi> = read_fn_pointer_array::<FnAbi>(fn_len, fn_list_ptr)
        .into_iter()
        .map(|fn_abi_closure| fn_abi_closure(&lut))
        .collect();

    // Pass 2: Determine state type
    let state_index = find_state_index(state_name, &types);
    let state_type = types.get(state_index).unwrap();

    let mut contract = ContractAbi::new(state_type.type_spec.clone());
    contract.actions(actions);
    contract.types(types);

    let mut output: Vec<u8> = Vec::new();
    output
        .write_all(&abi_header_bytes(version_binder, version_client))
        .unwrap();
    contract.serialize_abi(&mut output).unwrap();

    let length = output.len() as u64;
    let pointer = output.as_ptr() as u64;

    std::mem::forget(output);

    (length << 32) | pointer
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
