use pbc_zk::{Sbi32, SecretBinary};

pub fn main() {}

#[derive(SecretBinary)]
enum SecretEnum {
    Some { data: Sbi32 },
    None,
}
