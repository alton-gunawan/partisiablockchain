use pbc_contract_codegen::init;
use pbc_contract_codegen::state;
use pbc_contract_common::context::ContractContext;

#[state]
pub struct Something {
    num: u8,
}

#[init]
pub fn initialize(_ctx: ContractContext) -> Something {
    Something { num: 0 }
}

#[test]
#[allow(clippy::unit_cmp)]
pub fn smoke_test_versions() {
    assert_eq!(__PBC_VERSION_BINDER_9_0_0, ());
    assert_eq!(__PBC_VERSION_CLIENT_5_2_0, ());
}
