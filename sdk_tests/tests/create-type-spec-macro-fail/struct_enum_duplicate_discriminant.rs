#![no_main]
use create_type_spec_derive::CreateTypeSpec;

#[derive(CreateTypeSpec)]
enum MyEnum {
    #[discriminant(0)]
    Good {},
    #[discriminant(1)]
    Bad {},
    #[discriminant(1)]
    Neutral {},
}