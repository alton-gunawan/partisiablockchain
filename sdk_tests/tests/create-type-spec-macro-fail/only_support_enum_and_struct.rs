#![no_main]
use create_type_spec_derive::CreateTypeSpec;

#[derive(CreateTypeSpec)]
union MyUnion {
    f1: u32,
    f2: u32,
}