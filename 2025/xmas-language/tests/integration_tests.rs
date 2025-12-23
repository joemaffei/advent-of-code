/// Integration tests for xmas language.
///
/// These tests run complete xmas programs from files and verify
/// their output matches expected values.

use std::fs;
use xmas_language::{Lexer, Parser, Interpreter, Value};

/// Run a xmas program file and return the result and interpreter
fn run_xmas_file(file_path: &str) -> Result<(Value, Interpreter), String> {
    let code = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program)?;
    Ok((result, interpreter))
}

/// Format a value for comparison
fn format_value(value: &Value) -> String {
    match value {
        Value::Number(n) => format!("{}", n),
        Value::Boolean(b) => format!("{}", b),
        Value::String(s) => s.clone(),
        Value::Array1D(arr) => {
            let items: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Array2D(arr) => {
            let rows: Vec<String> = arr.iter()
                .map(|row| {
                    let items: Vec<String> = row.iter().map(format_value).collect();
                    format!("[{}]", items.join(", "))
                })
                .collect();
            format!("[{}]", rows.join(", "))
        }
    }
}

#[test]
fn test_01_basic_arithmetic() {
    let (result, interpreter) = run_xmas_file("tests/integration/01_basic_arithmetic.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(8), "Expected sum = 5 + 3 = 8");

    // Verify all arithmetic operations
    assert_eq!(interpreter.variables.get("a"), Some(&Value::Number(5)), "a should be 5");
    assert_eq!(interpreter.variables.get("b"), Some(&Value::Number(3)), "b should be 3");
    assert_eq!(interpreter.variables.get("sum"), Some(&Value::Number(8)), "sum = 5 + 3 = 8");
    assert_eq!(interpreter.variables.get("product"), Some(&Value::Number(15)), "product = 5 * 3 = 15");
    assert_eq!(interpreter.variables.get("difference"), Some(&Value::Number(2)), "difference = 5 - 3 = 2");
    assert_eq!(interpreter.variables.get("quotient"), Some(&Value::Number(1)), "quotient = 5 / 3 = 1 (integer division)");
    assert_eq!(interpreter.variables.get("remainder"), Some(&Value::Number(2)), "remainder = 5 % 3 = 2");
}

#[test]
fn test_02_comparison() {
    let (result, interpreter) = run_xmas_file("tests/integration/02_comparison.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Boolean(true), "Expected eq = (5 == 5) = true");

    // Verify all comparison operations
    assert_eq!(interpreter.variables.get("x"), Some(&Value::Number(5)), "x should be 5");
    assert_eq!(interpreter.variables.get("y"), Some(&Value::Number(3)), "y should be 3");
    assert_eq!(interpreter.variables.get("eq"), Some(&Value::Boolean(true)), "eq = (5 == 5) = true");
    assert_eq!(interpreter.variables.get("ne"), Some(&Value::Boolean(false)), "ne = (5 == 3) = false");
    assert_eq!(interpreter.variables.get("lt"), Some(&Value::Boolean(true)), "lt = (5 < 10) = true");
    assert_eq!(interpreter.variables.get("gt"), Some(&Value::Boolean(true)), "gt = (5 > 2) = true");
    assert_eq!(interpreter.variables.get("lte"), Some(&Value::Boolean(true)), "lte = (5 <= 5) = true");
    assert_eq!(interpreter.variables.get("gte"), Some(&Value::Boolean(true)), "gte = (5 >= 5) = true");
}

#[test]
fn test_03_arrays() {
    let (result, interpreter) = run_xmas_file("tests/integration/03_arrays.xmas").unwrap();

    // Check final output (concatenated array)
    if let Value::Array1D(arr) = &result {
        assert_eq!(arr.len(), 4, "Expected concatenated array [1, 2, 3, 4]");
        assert_eq!(arr[0], Value::Number(1));
        assert_eq!(arr[1], Value::Number(2));
        assert_eq!(arr[2], Value::Number(3));
        assert_eq!(arr[3], Value::Number(4));
    } else {
        panic!("Expected Array1D, got {:?}", result);
    }

    // Verify all array operations
    if let Some(Value::Array1D(arr)) = interpreter.variables.get("arr") {
        assert_eq!(arr.len(), 5, "arr should have 5 elements");
        assert_eq!(arr[0], Value::Number(1));
        assert_eq!(arr[4], Value::Number(5));
    } else {
        panic!("arr should be an array");
    }

    assert_eq!(interpreter.variables.get("first"), Some(&Value::Number(1)), "first = arr[0] = 1");
    assert_eq!(interpreter.variables.get("last"), Some(&Value::Number(5)), "last = arr[4] = 5");

    if let Some(Value::Array1D(slice)) = interpreter.variables.get("slice") {
        assert_eq!(slice.len(), 3, "slice should have 3 elements [2, 3, 4]");
        assert_eq!(slice[0], Value::Number(2));
        assert_eq!(slice[2], Value::Number(4));
    } else {
        panic!("slice should be an array");
    }
}

#[test]
fn test_04_functions() {
    let (result, interpreter) = run_xmas_file("tests/integration/04_functions.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(12), "Expected multiply(3, 4) = 12");

    // Verify both function calls worked
    assert_eq!(interpreter.variables.get("result"), Some(&Value::Number(30)), "result = add(10, 20) = 30");
    assert_eq!(interpreter.variables.get("result2"), Some(&Value::Number(12)), "result2 = multiply(3, 4) = 12");
}

#[test]
fn test_05_conditionals() {
    let (result, interpreter) = run_xmas_file("tests/integration/05_conditionals.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(200), "Expected result2 = 200 (x == 5 is false)");

    // Verify both conditional branches
    assert_eq!(interpreter.variables.get("x"), Some(&Value::Number(3)), "x should be 3");
    assert_eq!(interpreter.variables.get("result1"), Some(&Value::Number(100)), "result1 = if(x == 3, 100, 200) = 100 (true branch)");
    assert_eq!(interpreter.variables.get("result2"), Some(&Value::Number(200)), "result2 = if(x == 5, 100, 200) = 200 (false branch)");
}

#[test]
fn test_06_loops() {
    let (result, interpreter) = run_xmas_file("tests/integration/06_loops.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(120), "Expected product = 1 * 2 * 3 * 4 * 5 = 120");

    // Verify both loop operations
    assert_eq!(interpreter.variables.get("sum"), Some(&Value::Number(15)), "sum = for(n of arr, {{ _ = _ + n }}, 0) = 15");
    assert_eq!(interpreter.variables.get("product"), Some(&Value::Number(120)), "product = for(n of arr, {{ _ = _ * n }}, 1) = 120");
}

#[test]
fn test_07_strings() {
    let (result, interpreter) = run_xmas_file("tests/integration/07_strings.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(11), "Expected len('Hello World') = 11");

    // Verify string operations
    assert_eq!(interpreter.variables.get("hello"), Some(&Value::String("Hello".to_string())), "hello should be 'Hello'");
    assert_eq!(interpreter.variables.get("world"), Some(&Value::String("World".to_string())), "world should be 'World'");
    assert_eq!(interpreter.variables.get("combined"), Some(&Value::String("Hello World".to_string())), "combined should be 'Hello World'");
    assert_eq!(interpreter.variables.get("length"), Some(&Value::Number(11)), "length = len('Hello World') = 11");
}

#[test]
fn test_08_type_conversion() {
    let (result, interpreter) = run_xmas_file("tests/integration/08_type_conversion.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(246), "Expected ~'123' * 2 = 246");

    // Verify type conversion
    assert_eq!(interpreter.variables.get("str_num"), Some(&Value::String("123".to_string())), "str_num should be '123'");
    assert_eq!(interpreter.variables.get("num"), Some(&Value::Number(123)), "num = ~'123' = 123");
    assert_eq!(interpreter.variables.get("doubled"), Some(&Value::Number(246)), "doubled = 123 * 2 = 246");
}

#[test]
fn test_09_builtin_len() {
    let (result, interpreter) = run_xmas_file("tests/integration/09_builtin_len.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(5), "Expected len([1,2,3,4,5]) = 5");

    // Verify len() works on both arrays and strings
    if let Some(Value::Array1D(arr)) = interpreter.variables.get("arr1d") {
        assert_eq!(arr.len(), 5, "arr1d should have 5 elements");
    } else {
        panic!("arr1d should be an array");
    }
    assert_eq!(interpreter.variables.get("str"), Some(&Value::String("hello".to_string())), "str should be 'hello'");
    assert_eq!(interpreter.variables.get("len1d"), Some(&Value::Number(5)), "len1d = len([1,2,3,4,5]) = 5");
    assert_eq!(interpreter.variables.get("len_str"), Some(&Value::Number(5)), "len_str = len('hello') = 5");
}

#[test]
fn test_10_array_slicing() {
    let (result, interpreter) = run_xmas_file("tests/integration/10_array_slicing.xmas").unwrap();

    // Check final output (slice1)
    if let Value::Array1D(arr) = &result {
        assert_eq!(arr.len(), 3, "Expected slice [1, 2, 3]");
        assert_eq!(arr[0], Value::Number(1));
        assert_eq!(arr[1], Value::Number(2));
        assert_eq!(arr[2], Value::Number(3));
    } else {
        panic!("Expected Array1D, got {:?}", result);
    }

    // Verify all slicing operations
    if let Some(Value::Array1D(slice1)) = interpreter.variables.get("slice1") {
        assert_eq!(slice1.len(), 3, "slice1 = arr[1..4] should have 3 elements");
        assert_eq!(slice1[0], Value::Number(1));
        assert_eq!(slice1[2], Value::Number(3));
    } else {
        panic!("slice1 should be an array");
    }

    if let Some(Value::Array1D(slice2)) = interpreter.variables.get("slice2") {
        assert_eq!(slice2.len(), 4, "slice2 = arr[2..] should have 4 elements [2,3,4,5]");
        assert_eq!(slice2[0], Value::Number(2));
        assert_eq!(slice2[3], Value::Number(5));
    } else {
        panic!("slice2 should be an array");
    }

    if let Some(Value::Array1D(slice3)) = interpreter.variables.get("slice3") {
        assert_eq!(slice3.len(), 3, "slice3 = arr[..3] should have 3 elements [0,1,2]");
        assert_eq!(slice3[0], Value::Number(0));
        assert_eq!(slice3[2], Value::Number(2));
    } else {
        panic!("slice3 should be an array");
    }
}

#[test]
fn test_11_operator_precedence() {
    let (result, interpreter) = run_xmas_file("tests/integration/11_operator_precedence.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(7), "Expected 1 + 2 * 3 = 7 (not 9)");

    // Verify operator precedence is correct
    assert_eq!(interpreter.variables.get("result"), Some(&Value::Number(7)), "result = 1 + 2 * 3 = 7 (multiplication before addition)");
}

#[test]
fn test_12_nested_functions() {
    let (result, interpreter) = run_xmas_file("tests/integration/12_nested_functions.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(30), "Expected double(add(5, 10)) = double(15) = 30");

    // Verify nested function call worked
    assert_eq!(interpreter.variables.get("result"), Some(&Value::Number(30)), "result = double(add(5, 10)) = double(15) = 30");
}

#[test]
fn test_13_complex_expression() {
    let (result, interpreter) = run_xmas_file("tests/integration/13_complex_expression.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(25), "Expected if(10 > 5, 10*2, 5*2) + 5 = 20 + 5 = 25");

    // Verify complex expression evaluation
    assert_eq!(interpreter.variables.get("x"), Some(&Value::Number(10)), "x should be 10");
    assert_eq!(interpreter.variables.get("y"), Some(&Value::Number(5)), "y should be 5");
    assert_eq!(interpreter.variables.get("result"), Some(&Value::Number(25)), "result = if(10 > 5, 10*2, 5*2) + 5 = 25");
}

#[test]
fn test_14_return_value() {
    let (result, _interpreter) = run_xmas_file("tests/integration/14_return_value.xmas").unwrap();

    // Check final output
    // After setting _ = 42, reading _ should return 42
    assert_eq!(result, Value::Number(42), "Expected _ = 42, then _ returns 42");

    // Verify return value was set
    // Note: We can't directly check the return_value field, but we can verify
    // that reading _ after setting it works
}

#[test]
fn test_15_function_shorthand() {
    let (result, interpreter) = run_xmas_file("tests/integration/15_function_shorthand.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(6), "Expected addOne(5) = 6");

    // Verify function shorthand (expression body) works
    assert_eq!(interpreter.variables.get("result"), Some(&Value::Number(6)), "result = addOne(5) = 6");
}

#[test]
fn test_16_array_indexing() {
    let (result, interpreter) = run_xmas_file("tests/integration/16_array_indexing.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(30), "Expected arr[2] = 30");

    // Verify all indexing operations
    if let Some(Value::Array1D(arr)) = interpreter.variables.get("arr") {
        assert_eq!(arr.len(), 5, "arr should have 5 elements");
    } else {
        panic!("arr should be an array");
    }
    assert_eq!(interpreter.variables.get("first"), Some(&Value::Number(10)), "first = arr[0] = 10");
    assert_eq!(interpreter.variables.get("middle"), Some(&Value::Number(30)), "middle = arr[2] = 30");
    assert_eq!(interpreter.variables.get("last"), Some(&Value::Number(50)), "last = arr[4] = 50");
}

#[test]
fn test_17_string_indexing() {
    let (result, interpreter) = run_xmas_file("tests/integration/17_string_indexing.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::String("l".to_string()), "Expected str[2] = 'l'");

    // Verify string indexing operations
    assert_eq!(interpreter.variables.get("str"), Some(&Value::String("hello".to_string())), "str should be 'hello'");
    assert_eq!(interpreter.variables.get("first_char"), Some(&Value::String("h".to_string())), "first_char = str[0] = 'h'");
    assert_eq!(interpreter.variables.get("third_char"), Some(&Value::String("l".to_string())), "third_char = str[2] = 'l'");
}

#[test]
fn test_18_multiple_statements() {
    let (result, interpreter) = run_xmas_file("tests/integration/18_multiple_statements.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(6), "Expected a + b + c = 1 + 2 + 3 = 6");

    // Verify all variables were set correctly
    assert_eq!(interpreter.variables.get("a"), Some(&Value::Number(1)), "a should be 1");
    assert_eq!(interpreter.variables.get("b"), Some(&Value::Number(2)), "b should be 2");
    assert_eq!(interpreter.variables.get("c"), Some(&Value::Number(3)), "c should be 3");
    assert_eq!(interpreter.variables.get("sum"), Some(&Value::Number(6)), "sum = a + b + c = 6");
}

#[test]
fn test_19_blocks() {
    let (result, interpreter) = run_xmas_file("tests/integration/19_blocks.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Number(15), "Expected block result = 5 + 10 = 15");

    // Verify block executed correctly
    // Note: Variables inside blocks are in local scope, so we can't check them
    // But we can verify the result
    assert_eq!(interpreter.variables.get("result"), Some(&Value::Number(15)), "result = block {{ x = 5; y = 10; _ = x + y }} = 15");
}

#[test]
fn test_20_comparison_chaining() {
    let (result, interpreter) = run_xmas_file("tests/integration/20_comparison_chaining.xmas").unwrap();

    // Check final output
    assert_eq!(result, Value::Boolean(true), "Expected x < z = 5 < 15 = true");

    // Verify all comparison operations
    assert_eq!(interpreter.variables.get("x"), Some(&Value::Number(5)), "x should be 5");
    assert_eq!(interpreter.variables.get("y"), Some(&Value::Number(10)), "y should be 10");
    assert_eq!(interpreter.variables.get("z"), Some(&Value::Number(15)), "z should be 15");
    assert_eq!(interpreter.variables.get("result1"), Some(&Value::Boolean(true)), "result1 = x < y = 5 < 10 = true");
    assert_eq!(interpreter.variables.get("result2"), Some(&Value::Boolean(true)), "result2 = y < z = 10 < 15 = true");
    assert_eq!(interpreter.variables.get("result3"), Some(&Value::Boolean(true)), "result3 = x < z = 5 < 15 = true");
}

#[test]
fn test_21_booleans() {
    let (result, interpreter) = run_xmas_file("tests/integration/21_booleans.xmas").unwrap();

    assert_eq!(result, Value::Boolean(true), "Expected w = (true == true) = true");

    assert_eq!(interpreter.variables.get("x"), Some(&Value::Boolean(true)), "x should be true");
    assert_eq!(interpreter.variables.get("y"), Some(&Value::Boolean(false)), "y should be false");
    assert_eq!(interpreter.variables.get("z"), Some(&Value::Boolean(false)), "z = (true == false) = false");
    assert_eq!(interpreter.variables.get("w"), Some(&Value::Boolean(true)), "w = (true == true) = true");
}

#[test]
fn test_22_logical_operators() {
    let (result, interpreter) = run_xmas_file("tests/integration/22_logical_operators.xmas").unwrap();

    assert_eq!(result, Value::Number(0), "Expected j = (0 || 0) = 0");

    assert_eq!(interpreter.variables.get("a"), Some(&Value::Boolean(false)), "a = true && false = false");
    assert_eq!(interpreter.variables.get("b"), Some(&Value::Boolean(true)), "b = true && true = true");
    assert_eq!(interpreter.variables.get("c"), Some(&Value::Boolean(true)), "c = false || true = true");
    assert_eq!(interpreter.variables.get("d"), Some(&Value::Boolean(false)), "d = false || false = false");
    assert_eq!(interpreter.variables.get("e"), Some(&Value::Boolean(false)), "e = !true = false");
    assert_eq!(interpreter.variables.get("f"), Some(&Value::Boolean(true)), "f = !false = true");
    assert_eq!(interpreter.variables.get("g"), Some(&Value::Number(10)), "g = 5 && 10 = 10 (last truthy)");
    assert_eq!(interpreter.variables.get("h"), Some(&Value::Number(0)), "h = 0 && 10 = 0 (first falsy)");
    assert_eq!(interpreter.variables.get("i"), Some(&Value::Number(10)), "i = 0 || 10 = 10 (first truthy)");
    assert_eq!(interpreter.variables.get("j"), Some(&Value::Number(0)), "j = 0 || 0 = 0 (last falsy)");
}

#[test]
fn test_23_range_literals() {
    let (result, interpreter) = run_xmas_file("tests/integration/23_range_literals.xmas").unwrap();

    if let Value::Array1D(arr) = result {
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], Value::Number(3));
    } else {
        panic!("Expected Array1D");
    }

    // Check range1 = [0..5]
    if let Some(Value::Array1D(arr)) = interpreter.variables.get("range1") {
        assert_eq!(arr.len(), 6);
        assert_eq!(arr, &vec![
            Value::Number(0),
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
            Value::Number(4),
            Value::Number(5),
        ]);
    } else {
        panic!("Expected range1 to be Array1D");
    }

    // Check range2 = [10..15]
    if let Some(Value::Array1D(arr)) = interpreter.variables.get("range2") {
        assert_eq!(arr.len(), 6);
        assert_eq!(arr[0], Value::Number(10));
        assert_eq!(arr[5], Value::Number(15));
    } else {
        panic!("Expected range2 to be Array1D");
    }

    // Check range3 = [5..0] (reverse)
    if let Some(Value::Array1D(arr)) = interpreter.variables.get("range3") {
        assert_eq!(arr.len(), 6);
        assert_eq!(arr, &vec![
            Value::Number(5),
            Value::Number(4),
            Value::Number(3),
            Value::Number(2),
            Value::Number(1),
            Value::Number(0),
        ]);
    } else {
        panic!("Expected range3 to be Array1D");
    }

    // Check single = [3..3]
    if let Some(Value::Array1D(arr)) = interpreter.variables.get("single") {
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], Value::Number(3));
    } else {
        panic!("Expected single to be Array1D");
    }
}

#[test]
fn test_24_integer_division() {
    let (result, interpreter) = run_xmas_file("tests/integration/24_integer_division.xmas").unwrap();

    assert_eq!(result, Value::Number(4), "Expected d = 20 / 5 = 4");

    assert_eq!(interpreter.variables.get("a"), Some(&Value::Number(3)), "a = 10 / 3 = 3 (integer division)");
    assert_eq!(interpreter.variables.get("b"), Some(&Value::Number(3)), "b = 7 / 2 = 3 (integer division)");
    assert_eq!(interpreter.variables.get("c"), Some(&Value::Number(3)), "c = 15 / 4 = 3 (integer division)");
    assert_eq!(interpreter.variables.get("d"), Some(&Value::Number(4)), "d = 20 / 5 = 4");
}
