use rbasic::evaluate;

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