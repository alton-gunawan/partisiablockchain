mod nonzk {
    use pbc_contract_codegen::init;
    use pbc_contract_common::context::ContractContext;
    use pbc_contract_common::events::EventGroup;
    use pbc_contract_common::test_examples::*;
    use pbc_traits::WriteRPC;

    #[init]
    fn initialize_with_zero_args(context: ContractContext) -> (u32, Vec<EventGroup>) {
        (context.block_time as u32, vec![])
    }

    #[test]
    fn happy_case_zero_args() {
        let mut input_buf = Vec::new();
        EXAMPLE_CONTEXT.rpc_write_to(&mut input_buf).unwrap();
        // No RPC arguments
        __pbc_autogen__initialize_with_zero_args_wrapped(input_buf.as_mut_ptr(), input_buf.len());
    }

    #[test]
    fn rpc_too_long() {
        let result = std::panic::catch_unwind(|| {
            let mut input_buf = Vec::new();
            EXAMPLE_CONTEXT.rpc_write_to(&mut input_buf).unwrap();
            EXAMPLE_CONTEXT.rpc_write_to(&mut input_buf).unwrap();
            // No RPC arguments
            __pbc_autogen__initialize_with_zero_args_wrapped(
                input_buf.as_mut_ptr(),
                input_buf.len(),
            );
        });
        assert!(result.is_err());
    }
}
