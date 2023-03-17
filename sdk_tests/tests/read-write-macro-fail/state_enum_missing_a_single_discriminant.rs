#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
#[repr(C)]
enum Rating {
    Variant0 = 0,
    Variant1 = 1,
    Variant2 = 2,
    Variant3 = 3,
    Variant4 = 4,
    Variant5,
    Variant6 = 6,
    Variant7 = 7,
    Variant8 = 8,
}
