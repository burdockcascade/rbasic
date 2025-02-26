use crate::common::test_code_snippet_ok;

mod common;

#[test]
fn test_integer_addition() {
    test_code_snippet_ok("assert 1 + 2 == 3");
}

#[test]
fn test_integer_addition_with_negative() {
    test_code_snippet_ok("assert 1 + -2 == -1");
}

#[test]
fn test_float_addition() {
    test_code_snippet_ok("assert 1.0 + 2.0 == 3.0");
}

#[test]
fn test_float_addition_with_negative() {
    test_code_snippet_ok("assert 1.0 + -2.0 == -1.0");
}

#[test]
fn test_string_addition() {
    test_code_snippet_ok(r#"assert "Hello, " + "World!" == "Hello, World!""#);
}

#[test]
fn test_boolean_addition() {
    test_code_snippet_ok("assert true + false == false");
}

#[test]
fn test_integer_subtraction() {
    test_code_snippet_ok("assert 1 - 2 == -1");
}

#[test]
fn test_float_subtraction() {
    test_code_snippet_ok("assert 1.0 - 2.0 == -1.0");
}

#[test]
fn test_integer_multiplication() {
    test_code_snippet_ok("assert 2 * 3 == 6");
}

#[test]
fn test_float_multiplication() {
    test_code_snippet_ok("assert 2.0 * 3.0 == 6.0");
}

#[test]
fn test_integer_division() {
    test_code_snippet_ok("assert 6 / 3 == 2");
}

#[test]
fn test_float_division() {
    test_code_snippet_ok("assert 6.0 / 3.0 == 2.0");
}

#[test]
fn test_integer_modulo() {
    test_code_snippet_ok("assert 6 % 3 == 0");
}

#[test]
fn test_float_modulo() {
    test_code_snippet_ok("assert 6.0 % 3.0 == 0.0");
}

#[test]
fn test_integer_exponentiation() {
    test_code_snippet_ok("assert 2 ^ 3 == 8");
}

#[test]
fn test_float_exponentiation() {
    test_code_snippet_ok("assert 2.0 ^ 3.0 == 8.0");
}

#[test]
fn test_integer_comparison() {
    test_code_snippet_ok("assert 1 == 1");
}

#[test]
fn test_float_comparison() {
    test_code_snippet_ok("assert 1.0 == 1.0");
}

#[test]
fn test_string_comparison() {
    test_code_snippet_ok(r#"assert "Hello, World!" == "Hello, World!""#);
}

#[test]
fn test_boolean_comparison() {
    test_code_snippet_ok("assert true == true");
}

#[test]
fn test_integer_inequality() {
    test_code_snippet_ok("assert 1 != 2");
}

#[test]
fn test_float_inequality() {
    test_code_snippet_ok("assert 1.0 != 2.0");
}

#[test]
fn test_string_inequality() {
    test_code_snippet_ok(r#"assert "Hello, World!" != "Goodbye, World!""#);
}

#[test]
fn test_boolean_inequality() {
    test_code_snippet_ok("assert true != false");
}

#[test]
fn test_integer_less_than() {
    test_code_snippet_ok("assert 1 < 2");
}

#[test]
fn test_float_less_than() {
    test_code_snippet_ok("assert 1.0 < 2.0");
}

#[test]
fn test_integer_less_than_or_equal() {
    test_code_snippet_ok("assert 1 <= 1");
}

#[test]
fn test_float_less_than_or_equal() {
    test_code_snippet_ok("assert 1.0 <= 1.0");
}

#[test]
fn test_integer_greater_than() {
    test_code_snippet_ok("assert 2 > 1");
}

#[test]
fn test_float_greater_than() {
    test_code_snippet_ok("assert 2.0 > 1.0");
}

#[test]
fn test_integer_greater_than_or_equal() {
    test_code_snippet_ok("assert 2 >= 2");
}

#[test]
fn test_float_greater_than_or_equal() {
    test_code_snippet_ok("assert 2.0 >= 2.0");
}

#[test]
fn test_boolean_negation() {
    test_code_snippet_ok("assert !false");
}

#[test]
fn test_boolean_not() {
    test_code_snippet_ok("assert not false");
}

// Order of Operations

#[test]
fn test_order_of_operations() {
    test_code_snippet_ok("assert 1 + 2 * 3 == 7");
}

#[test]
fn test_parentheses() {
    test_code_snippet_ok("assert (1 + 2) * 3 == 9");
}

#[test]
fn test_nested_parentheses() {
    test_code_snippet_ok("assert (1 + (2 * 3)) == 7");
}

// Complex Operations

#[test]
fn test_complex_operation() {
    test_code_snippet_ok("assert ((5 + 3 * 2) / 4 > 1 and not (7 - 2 <= 3) or 2 == 2) and (not (3 > 5) and (10 - 2 * 3) / 2 == 2 or 6 != 3 + 3)");
}