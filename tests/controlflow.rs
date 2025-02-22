use crate::common::test_code_snippet;

mod common;

// If

#[test]
fn test_if_true() {
    test_code_snippet(r#"
        var condition = true
        if condition do
            condition = false
        end
        assert not condition
    "#);
}

// While

#[test]
fn test_while_true() {
    test_code_snippet(r#"
        var condition = true
        while condition do
            condition = false
        end
        assert not condition
    "#);
}

#[test]
fn test_while_counter() {
    test_code_snippet(r#"
        var i = 0
        while i < 3 do
            i = i + 1
        end
        assert i == 3
    "#);
}
