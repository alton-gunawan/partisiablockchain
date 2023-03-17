#![no_main]
use create_type_spec_derive::CreateTypeSpec;

#[derive(CreateTypeSpec)]
enum EnumWithMissingDiscriminant {
    #[discriminant(1)]
    Variant { some_field: u8 },
    Variant1 { other: u8 },
}