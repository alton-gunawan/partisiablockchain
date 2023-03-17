#![no_main]
use pbc_traits::ReadWriteState;
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
struct SomeState<ReadSomeState: ReadWriteState> {
    state_data: ReadSomeState,
}
