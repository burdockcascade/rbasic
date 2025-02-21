use rbasic::evaluate;

#[test]
fn test_simple_function() {
    let script = r#"
        function add(a, b)
            return a + b
        end
        return add(1, 2)
    "#;
    let result = evaluate(script, None);

    let expected = 3;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_simple_function_with_variable() {
    let script = r#"
        var a = 1;
        var b = 2;
        function add(a, b)
            return a + b
        end
        return add(a, b)
    "#;
    let result = evaluate(script, None);

    let expected = 3;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_simple_function_with_expression_as_argument() {
    let script = r#"
        function add(a, b)
            return a + b
        end
        return add(1, 2 * 3)
    "#;
    let result = evaluate(script, None);

    let expected = 7;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_simple_function_with_expression_as_argument_and_variable() {
    let script = r#"
        var a = 1;
        var b = 2;
        function add(a, b)
            return a + b
        end
        return add(a, b * 3)
    "#;
    let result = evaluate(script, None);

    let expected = 7;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_simple_function_with_expression_as_argument_and_variable_and_return() {
    let script = r#"
        var a = 1;
        var b = 2;
        function add(a, b)
            return a + b
        end
        var result = add(a, b * 3);
        return result
    "#;
    let result = evaluate(script, None);

    let expected = 7;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_single_function_call_with_no_return() {
    let script = r#"
        function add(a, b)
            return a + b
        end
        add(1, 2)
    "#;
    let result = evaluate(script, None);
    assert!(result.unwrap().is_none());
}