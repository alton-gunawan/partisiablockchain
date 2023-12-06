use pbc_zk::{load_metadata, load_sbi, zk_compute, Sbi32, SecretVarId};

#[zk_compute(shortname = 0x33)]
fn bad_computation_1() -> Sbi32 {
    let id = SecretVarId::new(999);
    let metadata: i32 = load_metadata(id);
    Sbi32::from(metadata)
}

#[zk_compute(shortname = 0x34)]
fn bad_computation_2() -> Sbi32 {
    let id = SecretVarId::new(999);
    load_sbi(id)
}

#[test]
#[should_panic]
fn run_example_computation_1() {
    bad_computation_1();
}

#[test]
#[should_panic]
fn run_example_computation_2() {
    bad_computation_2();
}
