#![no_main]
use read_write_state_derive::ReadWriteState;

struct Wheel {
    diameter: i32,
}

#[derive(ReadWriteState)]
struct Car {
    brand: String,
    wheel: Wheel,
}
