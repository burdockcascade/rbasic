use rbasic::evaluate;

#[test]
fn test_simple_if() {
    let script = r#"
        var a = 1
        if a == 1 then
            a = 2
        end
        return a
    "#;
    let result = evaluate(script, None);

    let expected = 2;
    let value: i64 = result.unwrap().unwrap().into();
    assert_eq!(value, expected);
}