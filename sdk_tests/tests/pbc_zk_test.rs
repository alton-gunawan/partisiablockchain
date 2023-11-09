//! Test for derive `SecretBinary`.

use pbc_zk::{load_sbi, zk_compute, Sbi32, SecretBinary, SecretVarId};
use read_write_state_derive::ReadWriteState;

#[derive(SecretBinary, Clone, Debug)]
#[allow(dead_code)]
struct MyStruct {
    v1: Sbi32,
    v2: Sbi32,
}

impl MyStruct {
    #[allow(dead_code)]
    fn increment_v1(mut self) -> Self {
        self.v1 = self.v1 + Sbi32::from(1);
        self
    }
}

#[zk_compute(shortname = 0x60)]
fn do_nothing_useful_0() {}

#[zk_compute(shortname = 0x61)]
fn do_nothing_useful(my_constant_value: i32) -> MyStruct {
    MyStruct {
        v1: Sbi32::from(1),
        v2: Sbi32::from(my_constant_value),
    }
}

#[zk_compute(shortname = 0x62)]
fn do_nothing_useful_2() -> (MyStruct, MyStruct) {
    let struct2 = load_sbi::<MyStruct>(SecretVarId::new(1));
    let struct3 = struct2.increment_v1();
    (struct3.clone(), struct3)
}

#[zk_compute(shortname = 0x62)]
fn do_nothing_useful_3(id: SecretVarId) -> (MyStruct, MyStruct) {
    let struct2 = load_sbi::<MyStruct>(id);
    let struct3 = struct2.increment_v1();
    (struct3.clone(), struct3)
}

#[derive(ReadWriteState)]
struct MyMetadata {}

#[test]
fn test_zk_compute_original_rust() {
    let struct1 = do_nothing_useful(9);
    assert_eq!(struct1.v1, Sbi32::from(1));
    assert_eq!(struct1.v2, Sbi32::from(9));
}

#[test]
fn test_zk_compute_autogen() {
    let metadata = MyMetadata {};
    let start_compute_0 = do_nothing_useful_0_start();
    let start_compute_1 = do_nothing_useful_start(7, &metadata);
    let start_compute_2 = do_nothing_useful_2_start([&metadata, &metadata]);

    assert_eq!(format!("{start_compute_0:?}"), "StartComputation { function_shortname: ShortnameZkComputation { shortname: Shortname { value: 96 } }, output_variable_metadata: [], input_arguments: [] }");
    assert_eq!(format!("{start_compute_1:?}"), "StartComputation { function_shortname: ShortnameZkComputation { shortname: Shortname { value: 97 } }, output_variable_metadata: [[]], input_arguments: [[7, 0, 0, 0]] }");
    assert_eq!(format!("{start_compute_2:?}"), "StartComputation { function_shortname: ShortnameZkComputation { shortname: Shortname { value: 98 } }, output_variable_metadata: [[], []], input_arguments: [] }");
}
