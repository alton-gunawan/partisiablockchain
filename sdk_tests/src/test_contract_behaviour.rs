//! Module with some common functions used for testing contract function behaviour.

#[cfg(feature = "abi")]
use pbc_contract_common::abi::AbiSerialize;

/// Creates a bunch of segment variants, possibly duplicating segments or leaving them out.
///
/// For example for segments \[X, Y, Z\], it will produce variants:
///
/// - []
/// - \[X\], \[Y\], \[Z\]
/// - \[X, X, Y, Z\]
/// - \[X, X, Y, Y, Z, Z\]
/// - ....
pub fn all_variants(segments: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut variants = vec![];
    let mut variant_idx = 0usize;
    while variant_idx < 3u32.pow(segments.len() as u32) as usize {
        let mut segment_counter = variant_idx;
        let mut variant = vec![];
        for &seg in segments {
            // Add between 0 and 2
            for _ in 0..(segment_counter % 3) {
                variant.extend(seg);
            }
            segment_counter /= 3;
        }

        variants.push(variant);
        variant_idx += 1;
    }
    variants
}

/// Flatten segments to only get good variants.
pub fn good_variant(segments: &[&[u8]]) -> Vec<u8> {
    segments.iter().copied().flatten().copied().collect()
}

/// Filter segments to only get the failing segments.
pub fn failing_variants(segments: &[&[u8]]) -> Vec<Vec<u8>> {
    let good_variant = good_variant(segments);
    all_variants(segments)
        .into_iter()
        .filter(|x| x.len() != good_variant.len())
        .collect()
}

/// Go through each segment and assert that the good variants
pub fn test_contract_function_with_variants(
    call: extern "C" fn(*mut u8, usize) -> u64,
    segments: &[&[u8]],
) {
    let mut good_variant = good_variant(segments);
    let bad_variants: Vec<Vec<u8>> = failing_variants(segments);

    // Good case
    call(good_variant.as_mut_ptr(), good_variant.len());

    // Bad cases
    for variant in bad_variants {
        let result = std::panic::catch_unwind(|| {
            let mut input_buf = variant.clone();
            call(input_buf.as_mut_ptr(), input_buf.len());
        });
        assert!(
            result.is_err(),
            "Succeeded for input bytes, when it should fail: {variant:?}",
        );
    }
}

/// Write rpc to buffer for `v`.
pub fn rpc_self<T: pbc_traits::WriteRPC>(v: T) -> Vec<u8> {
    let mut buf = vec![];
    v.rpc_write_to(&mut buf).unwrap();
    buf
}

/// Check that the abi gen function produces the expected bytes.
#[cfg(feature = "abi")]
pub fn assert_abi_serializable<
    K,
    V,
    AbiGenFn: FnOnce(&std::collections::BTreeMap<K, V>) -> pbc_contract_common::abi::FnAbi,
    const N: usize,
>(
    abi_gen_fn: AbiGenFn,
    expected_bytes: [u8; N],
) {
    let lut = std::collections::BTreeMap::new();
    let abi = abi_gen_fn(&lut);
    let mut abi_bytes = vec![];
    abi.serialize_abi(&mut abi_bytes).unwrap();
    assert_eq!(abi_bytes, expected_bytes.to_vec());
}

/// Identical between ZK and non-ZK contracts.
#[cfg(feature = "abi")]
pub const EXPECTED_DO_THING_ABI_BYTES: [u8; 27] = [
    0x02, // Function kind: Action
    0, 0, 0, 8, // Name length
    100, 111, 95, 116, 104, 105, 110, 103,  // Name
    0x05, // Shortname
    0, 0, 0, 1, // Number arguments
    0, 0, 0, 4, // Argument 0 Name Length
    97, 114, 103, 49,   // Argument 0 Name
    0x02, // Field 0 type ordinal: u16
];
