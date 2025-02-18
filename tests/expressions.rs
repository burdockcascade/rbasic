use rbasic::evaluate;

#[test]
fn test_addition() {
    let script = "return 1 + 2";
    let result = evaluate(script, None);
    let expected = 3;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_subtraction() {
    let script = "return 1 - 2";
    let result = evaluate(script, None);
    let expected = -1;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_multiplication() {
    let script = "return 2 * 3";
    let result = evaluate(script, None);
    let expected = 6;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_division() {
    let script = "return 6 / 3";
    let result = evaluate(script, None);
    let expected = 2;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_modulus() {
    let script = "return 7 % 3";
    let result = evaluate(script, None);
    let expected = 1;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_not() {
    let script = "return not true";
    let result = evaluate(script, None);
    let expected = false;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_and() {
    let script = "return true and false";
    let result = evaluate(script, None);
    let expected = false;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_or() {
    let script = "return true or false";
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_less_than() {
    let script = "return 1 < 2";
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_less_than_or_equal() {
    let script = "return 1 <= 1";
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_greater_than() {
    let script = "return 2 > 1";
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_greater_than_or_equal() {
    let script = "return 1 >= 1";
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_equal() {
    let script = "return 1 == 1";
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_not_equal() {
    let script = "return 1 != 2";
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_string_concatenation() {
    let script = r#"return "Hello, " + "world!""#;
    let result = evaluate(script, None);
    let expected = "Hello, world!";
    let value: String = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_string_comparison() {
    let script = r#"return "Hello" == "Hello""#;
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_string_comparison_not_equal() {
    let script = r#"return "Hello" != "World""#;
    let result = evaluate(script, None);
    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);
}

// test a simple expression script
#[test]
fn test_simple_expression() {

    let script = "return 1 + 2 * 3";
    let result = evaluate(script, None);

    let expected = 7;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);

}

#[test]
fn test_simple_expression_with_parentheses() {

    let script = "return (1 + 2) * 3";
    let result = evaluate(script, None);

    let expected = 9;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);

}

#[test]
fn test_simple_expression_with_parentheses_and_variables() {

    let script = r#"
        var a = 1;
        var b = 2;
        return (a + b) * 3
    "#;
    let result = evaluate(script, None);

    let expected = 9;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);

}

#[test]
fn test_simple_expression_with_function_and_expression_as_argument() {

    let script = r#"
        function add(a, b)
            return a + b
        end
        return add(1, 2 * 3)
    "#;
    let result = evaluate(script, None);

    let expected = 7;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);

}

#[test]
fn test_complex_expression_with_function_and_expression_as_argument() {

    let script = r#"
        function add(a, b)
            return a + b
        end
        return add(1, 2 * (3 + 4))
    "#;
    let result = evaluate(script, None);

    let expected = 15;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);

}

#[test]
fn test_very_complex_expression_with_function_and_expression_as_argument() {

    let script = r#"
        function add(a, b)
            return a + b
        end
        return add(1, 2 * (3 + 4) + 5 * 6)
    "#;
    let result = evaluate(script, None);

    let expected = 45;
    let value: i64 = result.unwrap().into();
    assert_eq!(value, expected);

}

#[test]
fn test_super_complex_expression_with_function_and_expression_as_argument() {

    let script = r#"
        return ((5 + 3 * 2) / 4 > 1 and not (7 - 2 <= 3) or 2 == 2) and (not (3 > 5) and (10 - 2 * 3) / 2 == 2 or 6 != 3 + 3)
    "#;
    let result = evaluate(script, None);

    let expected = true;
    let value: bool = result.unwrap().into();
    assert_eq!(value, expected);

}