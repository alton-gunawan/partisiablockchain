#![no_main]
use create_type_spec_derive::CreateTypeSpec;

#[derive(CreateTypeSpec)]
enum MyEnum {
    #[discriminant(0)]
    SomeTuple(u8, u8),
}