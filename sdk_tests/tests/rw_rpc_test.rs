use pbc_traits::ReadWriteRPC;
use read_write_rpc_derive::ReadWriteRPC;

#[derive(ReadWriteRPC)]
struct SimpleStruct {
    a: u8,
}

#[derive(ReadWriteRPC)]
struct ComplexStruct {
    a: SimpleStruct,
    b: SimpleStruct,
    c: u32,
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
