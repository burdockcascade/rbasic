use rbasic::evaluate;

#[test]
fn test_variable_declaration() {
    let script = r#"
        var a = 1
        return a
    "#;
    let result = evaluate(script, None);

    let expected = 1;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_variable_assignment() {
    let script = r#"
        var a = 1
        a = 2
        return a
    "#;
    let result = evaluate(script, None);

    let expected = 2;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_variable_reassignment() {
    let script = r#"
        var a = 1
        a = 2
        a = 3
        return a
    "#;
    let result = evaluate(script, None);

    let expected = 3;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}

#[test]
fn test_variable_reassignment_with_expression() {
    let script = r#"
        var a = 1
        a = 2 * 3
        return a
    "#;
    let result = evaluate(script, None);

    let expected = 6;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}