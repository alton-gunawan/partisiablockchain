#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(PartialEq, ReadWriteState)]
#[repr(u8)]
enum MyEnum {
    Good = 0,
    Bad = 1,
    Neutral = 1,
}
