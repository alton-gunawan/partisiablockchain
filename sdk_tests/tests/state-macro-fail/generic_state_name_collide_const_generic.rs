#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
struct SomeState<const ReadSomeState: usize> { }
