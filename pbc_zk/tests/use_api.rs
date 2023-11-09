use std::collections::HashSet;

use pbc_zk::{Sbi32, SecretVarId};

#[test]
fn set_and_get() {
    let secrets = vec![pbc_zk::api::SecretVarWithId {
        id: SecretVarId::new(55),
        value: vec![3, 0, 0, 0],
        metadata: vec![9, 0, 0, 0],
    }];

    unsafe {
        pbc_zk::api::set_secrets_with_ids(secrets);
    }

    assert_eq!(
        pbc_zk::load_sbi::<Sbi32>(SecretVarId::new(55)),
        Sbi32::from(3)
    );
    assert_eq!(pbc_zk::load_metadata::<i32>(SecretVarId::new(55)), 9i32);
}

#[allow(deprecated)]
#[test]
fn num_secret_variables_test() {
    let secrets = vec![
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(32),
            metadata: 12i32,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(32),
            metadata: 12i32,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(32),
            metadata: 12i32,
        },
    ];
    unsafe {
        pbc_zk::api::set_secrets_of_single_type(secrets);
    }

    assert_eq!(pbc_zk::num_secret_variables(), 3);
}

#[test]
fn from_to_i8_specific() {
    let secrets = vec![
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(32),
            metadata: 12i32,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(32),
            metadata: 12i32,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(32),
            metadata: 12i32,
        },
    ];
    unsafe {
        pbc_zk::api::set_secrets_of_single_type(secrets);
    }

    let ids: HashSet<u32> = pbc_zk::secret_variable_ids().map(|x| x.raw_id).collect();
    assert_eq!(ids, HashSet::from([1, 2, 3]));
}
