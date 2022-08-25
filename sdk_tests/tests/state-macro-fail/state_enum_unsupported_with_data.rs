#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
#[repr(C)]
enum MyOptionU32 {
    SomeU32(u32),
    NoInt,
}
