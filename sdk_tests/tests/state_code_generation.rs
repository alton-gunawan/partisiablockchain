use pbc_contract_codegen::state;

#[state]
pub struct Something {
    _num: u8,
}

#[test]
#[allow(clippy::unit_cmp)]
pub fn smoke_test_versions() {
    #[cfg(feature = "zk")]
    assert_eq!(__PBC_VERSION_BINDER_9_3_0, ());
    #[cfg(not(feature = "zk"))]
    assert_eq!(__PBC_VERSION_BINDER_9_0_0, ());
    assert_eq!(__PBC_VERSION_CLIENT_5_2_0, ());
}
