#[allow(deprecated)]
#[test]
fn from_to_i8_specific() {
    let secrets = vec![
        pbc_zk::api::SecretVarInput {
            value: pbc_zk::Sbi32::from(32),
            metadata: 12i32,
        },
        pbc_zk::api::SecretVarInput {
            value: pbc_zk::Sbi32::from(32),
            metadata: 12i32,
        },
        pbc_zk::api::SecretVarInput {
            value: pbc_zk::Sbi32::from(32),
            metadata: 12i32,
        },
    ];
    unsafe {
        pbc_zk::api::set_secrets_of_single_type(secrets);
    }

    assert_eq!(pbc_zk::num_secret_variables(), 3);

    let ids: Vec<i32> = pbc_zk::secret_variable_ids().collect();
    assert_eq!(ids, vec![1, 2, 3]);
}
