use pbc_traits::ReadRPC;
use pbc_traits::WriteRPC;
use read_write_rpc_derive::ReadRPC;
use read_write_rpc_derive::WriteRPC;

#[derive(ReadRPC, WriteRPC)]
struct SimpleStruct {
    a: u8,
}

#[derive(ReadRPC, WriteRPC)]
struct ComplexStruct {
    a: SimpleStruct,
    b: SimpleStruct,
    c: u32,
}

#[derive(ReadRPC, WriteRPC)]
enum AnEnum {
    #[discriminant(0)]
    A { a: u8 },
    #[discriminant(3)]
    B { a: u8, b: u8 },
    #[discriminant(125)]
    C { a: SimpleStruct },
}

#[test]
pub fn derive_for_simple_struct() {
    let simple_struct = SimpleStruct { a: 42 };

    let mut buf: Vec<u8> = Vec::new();
    simple_struct.rpc_write_to(&mut buf).unwrap();
    assert_eq!(&buf, &[42]);

    let mut ctx_reader = std::io::Cursor::new(buf);
    let simple2 = SimpleStruct::rpc_read_from(&mut ctx_reader);
    assert_eq!(simple2.a, 42);
}

#[test]
pub fn derive_for_complex_struct() {
    let simple_struct_1 = SimpleStruct { a: 42 };
    let simple_struct_2 = SimpleStruct { a: 43 };

    let complex = ComplexStruct {
        a: simple_struct_1,
        b: simple_struct_2,
        c: 15432,
    };

    let mut buf: Vec<u8> = Vec::new();
    complex.rpc_write_to(&mut buf).unwrap();
    assert_eq!(&buf, &[42, 43, 0, 0, 60, 72]);

    let mut ctx_reader = std::io::Cursor::new(buf);
    let complex2 = ComplexStruct::rpc_read_from(&mut ctx_reader);
    assert_eq!(complex2.a.a, 42);
    assert_eq!(complex2.b.a, 43);
    assert_eq!(complex2.c, 15432);
}

#[test]
pub fn derive_for_simple_enum() {
    let simple_enum_1 = AnEnum::A { a: 1 };
    let simple_enum_2 = AnEnum::B { a: 1, b: 42 };

    let mut buf_1: Vec<u8> = Vec::new();
    simple_enum_1.rpc_write_to(&mut buf_1).unwrap();
    assert_eq!(&buf_1, &[0, 1]);

    let mut buf_2: Vec<u8> = Vec::new();
    simple_enum_2.rpc_write_to(&mut buf_2).unwrap();
    assert_eq!(&buf_2, &[3, 1, 42]);
}

#[test]
pub fn derive_for_complex_enum() {
    let complex_enum = AnEnum::C {
        a: SimpleStruct { a: 42 },
    };

    let mut buf: Vec<u8> = Vec::new();
    complex_enum.rpc_write_to(&mut buf).unwrap();
    assert_eq!(&buf, &[125, 42]);
}
