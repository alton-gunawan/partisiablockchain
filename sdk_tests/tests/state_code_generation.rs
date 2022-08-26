use pbc_contract_codegen::state;

#[state]
pub struct Something {
    _num: u8,
}

#[test]
#[allow(clippy::unit_cmp)]
pub fn smoke_test_versions() {
    assert_eq!(__PBC_VERSION_BINDER_7_0_0, ());
    assert_eq!(__PBC_VERSION_CLIENT_4_1_0, ());
}
