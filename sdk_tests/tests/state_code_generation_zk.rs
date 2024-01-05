use pbc_contract_codegen::init;
use pbc_contract_codegen::state;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::zk::ZkState;

#[state]
pub struct Something {
    num: u8,
}

#[init(zk = true)]
pub fn initialize(_ctx: ContractContext, _zk_state: ZkState<u8>) -> Something {
    Something { num: 0 }
}

#[test]
#[allow(clippy::unit_cmp)]
pub fn smoke_test_versions() {
    assert_eq!(__PBC_VERSION_BINDER_11_0_0, ());
    assert_eq!(__PBC_VERSION_CLIENT_5_4_0, ());
}
