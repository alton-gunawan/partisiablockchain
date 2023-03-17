#[test]
fn read_write_macro_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/read-write-macro-fail/*.rs");
}

#[test]
fn create_type_spec_macro_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/create-type-spec-macro-fail/*.rs");
}
