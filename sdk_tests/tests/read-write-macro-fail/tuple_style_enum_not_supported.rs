#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
enum MyEnum {
    #[discriminant(0)]
    SomeTuple(u8, u8),
}