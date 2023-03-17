#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
enum MyEnum {
    #[discriminant(0)]
    Good {},
    #[discriminant(1)]
    Bad {},
    #[discriminant(1)]
    Neutral {},
}