#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
enum EnumWithMissingDiscriminant {
    #[discriminant]
    Variant { some_field: u8 },
}