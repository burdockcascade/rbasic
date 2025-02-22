use crate::common::test_code_snippet;

mod common;

#[test]
fn test_variable_declaration() {
    test_code_snippet(r#"
        var a = 1
        assert a == 1
    "#);
}

#[test]
fn test_variable_assignment() {
    test_code_snippet(r#"
        var a = 1
        a = 2
        assert a == 2
    "#);
}

#[test]
fn test_variable_reassignment() {
    test_code_snippet(r#"
        var a = 1
        a = 2
        a = 3
        assert a == 3
    "#);
}

#[test]
fn test_variable_reassignment_with_expression() {
    test_code_snippet(r#"
        var a = 1
        a = 2 * 3
        assert a == 6
    "#);
}

#[test]
fn test_variable_reassignment_with_variable() {
    test_code_snippet(r#"
        var a = 1
        var b = 2
        a = b
        assert a == 2
    "#);
}

#[test]
fn test_variable_reassignment_with_variable_and_expression() {
    test_code_snippet(r#"
        var a = 1
        var b = 2
        a = b * 3
        assert a == 6
    "#);
}

#[test]
fn test_variable_reassignment_with_variable_and_expression_and_variable() {
    test_code_snippet(r#"
        var a = 1
        var b = 2
        var c = 3
        a = b * c
        assert a == 6
    "#);
}

// arrays

#[test]
fn test_array_declaration() {
    test_code_snippet(r#"
        var a = [1, 2, 3]
        assert a[0] == 1
        assert a[1] == 2
        assert a[2] == 3
    "#);
}

#[test]
fn test_multi_dimension_array_declaration() {
    test_code_snippet(r#"
        var a = [[1, 2], [3, 4]]
        assert a[0][0] == 1
        assert a[0][1] == 2
        assert a[1][0] == 3
        assert a[1][1] == 4
    "#);
}

#[test]
fn test_array_assignment() {
    test_code_snippet(r#"
        var a = [1, 2, 3]
        assert a[0] == 1
        assert a[1] == 2
        assert a[2] == 3
        a[0] = 4
        a[1] = 5
        a[2] = 6
        assert a[0] == 4
        assert a[1] == 5
        assert a[2] == 6
    "#);
}

// Tables

#[test]
fn test_table_declaration() {
    test_code_snippet(r#"
        var a = {"a": 1, "b": 2, "c": 3}
        assert a["a"] == 1
    "#);
}

