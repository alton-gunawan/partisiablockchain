#![no_main]
use pbc_contract_codegen::state;

pub fn main() {}

#[state]
enum MyEnum {
    A,
    B,
    C,
}
