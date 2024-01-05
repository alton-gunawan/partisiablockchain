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

#[cfg(feature = "abi")]
#[test]
fn trybuild_codegen_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/pbc_contract_codegen/fail/*.rs");
    t.compile_fail("tests/pbc_contract_codegen/fail/zk/*.rs");
    t.compile_fail("tests/pbc_contract_codegen/compile_but_fail_abi/*.rs");
}

#[test]
fn trybuild_codegen() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pbc_contract_codegen/success/*.rs");
    t.pass("tests/pbc_contract_codegen/success/zk/*.rs");
    #[cfg(not(feature = "abi"))]
    t.pass("tests/pbc_contract_codegen/compile_but_fail_abi/*.rs");
}
