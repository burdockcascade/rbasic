use rbasic::evaluate;
use crate::common::test_script;

mod common;

#[test]
fn test_simple_function() {
    test_script(r#"
        function add(a, b)
            return a + b
        end
        function main()
            return add(1, 2)
        end
    "#);
}

#[test]
fn test_simple_function_with_variable() {
    test_script(r#"
        function add(a, b)
            return a + b
        end
        function main()
            var a = 1
            var b = 2
            return add(a, b)
        end
    "#);
}

#[test]
fn test_simple_function_with_expression_as_argument() {
    test_script(r#"
        function add(a, b)
            return a + b
        end
        function main()
            return add(1, 2 * 3)
        end
    "#);
}

#[test]
fn test_simple_function_with_expression_as_argument_and_variable() {
    test_script(r#"
        function add(a, b)
            return a + b
        end
        function main()
             var a = 1;
            var b = 2;
            return add(a, b * 3)
        end
    "#);
}

#[test]
fn test_simple_function_with_expression_as_argument_and_variable_and_return() {
    test_script(r#"
        function add(a, b)
            return a + b
        end
        function main()
            var a = 1;
            var b = 2;
            var result = add(a, b * 3);
            return result
        end
    "#);
}

#[test]
fn test_single_function_call_with_no_return() {
    test_script(r#"
        function add(a, b)
            return a + b
        end
        function main()
            add(1, 2)
        end
    "#);
}