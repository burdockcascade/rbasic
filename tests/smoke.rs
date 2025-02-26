use crate::common::test_code_snippet_ok;

mod common;

#[test]
fn smoke_test() {
    // This is a smoke test to ensure that the code compiles and runs
    test_code_snippet_ok("assert true");
}