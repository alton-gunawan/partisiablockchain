#![cfg(not(feature = "zk"))]
mod nonzk {

    use pbc_contract_codegen::{action, callback};
    use pbc_contract_common::context::ContractContext;
    use pbc_contract_common::events::EventGroup;
    use pbc_contract_common::{address::Address, context::CallbackContext};
    use read_write_state_derive::ReadWriteState;

    #[action]
    pub fn action_with_vec(
        _ctx: ContractContext,
        _state: u8,
        addresses: Vec<Address>,
    ) -> (u8, Vec<EventGroup>) {
        assert_eq!(addresses.len(), 0);
        (1, vec![])
    }

    #[action(shortname = 0xe58e26)]
    pub fn action_with_arrays(
        _ctx: ContractContext,
        _state: u8,
        addresses: [u8; 32],
    ) -> (u8, Vec<EventGroup>) {
        assert_eq!(addresses.len(), 32);
        (1, vec![])
    }

    #[action(shortname = 0x00)]
    pub fn action_with_zero_shortname(
        _ctx: ContractContext,
        _state: u8,
        addresses: [u8; 32],
    ) -> (u8, Vec<EventGroup>) {
        assert_eq!(addresses.len(), 32);
        (1, vec![])
    }

    #[action]
    pub fn action_without_arg(_ctx: ContractContext, state: u8) -> (u8, Vec<EventGroup>) {
        (state, vec![])
    }

    #[derive(ReadWriteState)]
    #[repr(C)]
    pub struct Pair {
        some: u32,
        pair: u32,
    }

    #[action]
    pub fn action_with_mutable_state(
        _ctx: ContractContext,
        mut state: Pair,
    ) -> (Pair, Vec<EventGroup>) {
        state.some += 3;
        state.pair += 7;
        (state, vec![])
    }

    #[callback(shortname = 0x64)]
    pub fn on_callback(
        _ctx: ContractContext,
        _callback_ctx: CallbackContext,
        state: u64,
        _rpc: u8,
    ) -> (u64, Vec<EventGroup>) {
        (state, vec![])
    }

    #[test]
    pub fn callback_generated() {
        let _fn_ptr = __pbc_autogen__on_callback_wrapped;
    }

    #[test]
    #[cfg(feature = "abi")]
    pub fn shortname_is_defined() {
        let _pointer = __abi_fn_action_with_arrays;
        let _pointer2 = __abi_fn_action_with_zero_shortname;
    }
}
