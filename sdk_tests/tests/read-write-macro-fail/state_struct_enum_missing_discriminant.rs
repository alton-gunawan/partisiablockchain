#![no_main]
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState)]
enum EnumWithMissingDiscriminant {
    #[discriminant(1)]
    Variant { some_field: u8 },
    Variant1 { other: u8 },
}