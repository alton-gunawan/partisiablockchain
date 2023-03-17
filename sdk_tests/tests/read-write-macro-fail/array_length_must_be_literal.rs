#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
struct StructWithArray {
    my_array: [u8; 8*8]
}