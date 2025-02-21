use rbasic::evaluate;

#[test]
fn test_integer_addition() {
    let script = "return 1 + 2";
    let result = evaluate(script, None);

    let expected = 3;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_addition_with_negative() {
    let script = "return 1 + -2";
    let result = evaluate(script, None);

    let expected = -1;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_addition() {
    let script = "return 1.0 + 2.0";
    let result = evaluate(script, None);

    let expected = 3.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_addition_with_negative() {
    let script = "return 1.0 + -2.0";
    let result = evaluate(script, None);

    let expected = -1.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_string_addition() {
    let script = r#"return "Hello, " + "World!""#;
    let result = evaluate(script, None);

    let expected = "Hello, World!";
    let value: String = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_boolean_addition() {
    let script = "return true + false";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_subtraction() {
    let script = "return 1 - 2";
    let result = evaluate(script, None);

    let expected = -1;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_subtraction() {
    let script = "return 1.0 - 2.0";
    let result = evaluate(script, None);

    let expected = -1.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_multiplication() {
    let script = "return 2 * 3";
    let result = evaluate(script, None);

    let expected = 6;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_multiplication() {
    let script = "return 2.0 * 3.0";
    let result = evaluate(script, None);

    let expected = 6.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_division() {
    let script = "return 6 / 3";
    let result = evaluate(script, None);

    let expected = 2;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_division() {
    let script = "return 6.0 / 3.0";
    let result = evaluate(script, None);

    let expected = 2.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_modulo() {
    let script = "return 6 % 3";
    let result = evaluate(script, None);

    let expected = 0;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_modulo() {
    let script = "return 6.0 % 3.0";
    let result = evaluate(script, None);

    let expected = 0.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_exponentiation() {
    let script = "return 2 ^ 3";
    let result = evaluate(script, None);

    let expected = 8;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_exponentiation() {
    let script = "return 2.0 ^ 3.0";
    let result = evaluate(script, None);

    let expected = 8.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_comparison() {
    let script = "return 1 == 1";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_comparison() {
    let script = "return 1.0 == 1.0";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_string_comparison() {
    let script = r#"return "Hello, World!" == "Hello, World!""#;
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_boolean_comparison() {
    let script = "return true == true";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_inequality() {
    let script = "return 1 != 2";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_inequality() {
    let script = "return 1.0 != 2.0";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_string_inequality() {
    let script = r#"return "Hello, World!" != "Hello, World!""#;
    let result = evaluate(script, None);

    let expected = false;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_boolean_inequality() {
    let script = "return true != false";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_less_than() {
    let script = "return 1 < 2";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_less_than() {
    let script = "return 1.0 < 2.0";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_less_than_or_equal() {
    let script = "return 1 <= 1";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_less_than_or_equal() {
    let script = "return 1.0 <= 1.0";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_greater_than() {
    let script = "return 2 > 1";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_greater_than() {
    let script = "return 2.0 > 1.0";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_greater_than_or_equal() {
    let script = "return 1 >= 1";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_greater_than_or_equal() {
    let script = "return 1.0 >= 1.0";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_integer_negation() {
    let script = "return -1";
    let result = evaluate(script, None);

    let expected = -1;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_float_negation() {
    let script = "return -1.0";
    let result = evaluate(script, None);

    let expected = -1.0;
    let value: f64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_boolean_negation() {
    let script = "return !true";
    let result = evaluate(script, None);

    let expected = false;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_boolean_not() {
    let script = "return not true";
    let result = evaluate(script, None);

    let expected = false;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

// Order of Operations

#[test]
fn test_order_of_operations() {
    let script = "return 1 + 2 * 3";
    let result = evaluate(script, None);

    let expected = 7;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_parentheses() {
    let script = "return (1 + 2) * 3";
    let result = evaluate(script, None);

    let expected = 9;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_nested_parentheses() {
    let script = "return (1 + (2 * 3)) * 4";
    let result = evaluate(script, None);

    let expected = 28;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

// Complex Operations

#[test]
fn test_complex_operation() {
    let script = "return ((5 + 3 * 2) / 4 > 1 and not (7 - 2 <= 3) or 2 == 2) and (not (3 > 5) and (10 - 2 * 3) / 2 == 2 or 6 != 3 + 3)";
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}