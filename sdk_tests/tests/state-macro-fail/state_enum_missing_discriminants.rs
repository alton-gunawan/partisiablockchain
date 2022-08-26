#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
#[repr(C)]
enum Rating {
    Good, Bad, Neutral,
}
