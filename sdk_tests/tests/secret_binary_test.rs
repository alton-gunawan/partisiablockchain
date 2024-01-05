use pbc_zk::SecretBinaryFixedSize;
use pbc_zk::{Sbi1, Sbi32, Sbi8, SecretBinary};

#[derive(SecretBinary)]
struct MyStruct1 {}

#[derive(SecretBinary)]
struct MyStruct2 {
    data: Sbi32,
}

#[derive(SecretBinary)]
struct MyStruct3 {
    v1: Sbi32,
    v2: Sbi32,
}

#[derive(SecretBinary)]
struct MyStruct5 {
    data: [MyStruct3; 3],
}

#[derive(SecretBinary)]
struct MyStruct7 {
    data: [Sbi8; 128],
}

#[derive(SecretBinary)]
struct StructWithBit {
    data: Sbi1,
}

#[derive(SecretBinary)]
struct StructWithSeveralBits {
    v1: Sbi1,
    v2: Sbi1,
    v3: Sbi1,
    v4: Sbi1,
    v5: Sbi1,
}

#[test]
fn test() {
    assert_eq!(MyStruct1::BITS, 0);
    assert_eq!(MyStruct2::BITS, 32);
    assert_eq!(MyStruct3::BITS, 64);
    assert_eq!(MyStruct5::BITS, 64 * 3);
    assert_eq!(MyStruct7::BITS, 8 * 128);
    assert_eq!(StructWithBit::BITS, 1);
    assert_eq!(StructWithSeveralBits::BITS, 5);
}
