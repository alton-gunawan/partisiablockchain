#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
union MyUnion {
    f1: u32,
    f2: u32,
}