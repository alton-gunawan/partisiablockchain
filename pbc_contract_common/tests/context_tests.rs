#[cfg(test)]
mod context_tests {
    use pbc_contract_common::context::ExecutionResult;
    use pbc_traits::WriteRPC;

    #[test]
    fn test_get_return_data() {
        let mut e = ExecutionResult {
            succeeded: false,
            return_data: vec![],
        };
        42u64.rpc_write_to(&mut e.return_data).unwrap();
        let return_data_u64 = e.get_return_data::<u64>();
        assert_eq!(return_data_u64, 42);

        let mut e = ExecutionResult {
            succeeded: false,
            return_data: vec![],
        };
        String::from("Hello")
            .rpc_write_to(&mut e.return_data)
            .unwrap();
        let return_data_string = e.get_return_data::<String>();
        assert_eq!(return_data_string, "Hello");

        let mut e = ExecutionResult {
            succeeded: false,
            return_data: vec![],
        };
        true.rpc_write_to(&mut e.return_data).unwrap();
        let return_data_bool = e.get_return_data::<bool>();
        assert!(return_data_bool)
    }
}
