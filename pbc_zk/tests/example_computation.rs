use pbc_zk::{load_metadata, load_sbi, secret_variable_ids, zk_compute, Sbi32};

#[zk_compute(shortname = 0x32)]
fn example_computation() -> Sbi32 {
    let mut sum = Sbi32::from(0);
    for id in secret_variable_ids() {
        let metadata = load_metadata::<i32>(id);
        if metadata != 0 {
            let mut value = load_sbi::<Sbi32>(id);
            println!("Derp: {value:?}, {metadata:?}");
            if metadata == 2 {
                value = value * Sbi32::from(2);
            }
            if metadata == 4 {
                value = value << 2;
            }
            sum = sum + value;
            println!("    {sum:?}");
        }
    }
    sum
}

#[test]
fn run_example_computation() {
    let secrets = vec![
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(3),
            metadata: 1,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(3),
            metadata: 0,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(3),
            metadata: 0,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(3),
            metadata: 2,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(6),
            metadata: 4,
        },
        pbc_zk::api::SecretVarInput {
            value: Sbi32::from(9),
            metadata: 2,
        },
    ];
    unsafe {
        pbc_zk::api::set_secrets_of_single_type(secrets);
    }

    let result = example_computation();
    assert_eq!(result, Sbi32::from(51));
}
