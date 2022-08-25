#[test]
fn state_macro_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/state-macro-fail/*.rs");
}
