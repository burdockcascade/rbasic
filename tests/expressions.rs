use rbasic::evaluate;

// test a simple expression script
#[test]
fn test_simple_expression() {

    let script = "var x = 1 + 2 * 3";
    let result = evaluate(script);

}