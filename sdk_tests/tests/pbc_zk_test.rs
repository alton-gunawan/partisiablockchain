//! Test for derive `SecretBinary`.

use pbc_zk::{load_sbi, Sbi32, SecretBinary};

#[derive(SecretBinary, Clone)]
#[allow(dead_code)]
struct MyStruct {
    v1: Sbi32,
    v2: Sbi32,
}

impl MyStruct {
    fn identity(self) -> Self {
        self
    }
}

#[allow(dead_code)]
fn derp() -> (MyStruct, MyStruct) {
    let struct1 = MyStruct {
        v1: Sbi32::from(1),
        v2: Sbi32::from(5),
    };
    let struct2 = load_sbi::<MyStruct>(1);
    (struct1, struct2.identity())
}
