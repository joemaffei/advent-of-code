# Integration Tests

This directory contains integration tests for the xmas language. Each test is a complete xmas program that exercises specific language features.

## Running the Tests

```bash
# Run all integration tests
cargo test --test integration_tests

# Run a specific test
cargo test --test integration_tests test_01_basic_arithmetic

# Run with output
cargo test --test integration_tests -- --nocapture
```

## Test Files

### 01_basic_arithmetic.xmas
Tests basic arithmetic operations: `+`, `-`, `*`, `/`, `%`
- Expected output: `8` (sum of 5 + 3)

### 02_comparison.xmas
Tests comparison operators: `==`, `<`, `>`, `<=`, `>=`
- Expected output: `1` (true, because 5 == 5)

### 03_arrays.xmas
Tests array operations: literals, indexing, slicing, concatenation
- Expected output: `[1, 2, 3, 4]` (concatenated arrays)

### 04_functions.xmas
Tests function definition and calling
- Expected output: `12` (multiply(3, 4))

### 05_conditionals.xmas
Tests `if` conditional expressions
- Expected output: `200` (false branch when x != 5)

### 06_loops.xmas
Tests `for` loops with accumulation
- Expected output: `120` (product of [1,2,3,4,5])

### 07_strings.xmas
Tests string operations and concatenation
- Expected output: `11` (length of "Hello World")

### 08_type_conversion.xmas
Tests the `~` operator for type conversion:
- String-to-number conversion: `~"123"` → `123`
- Boolean-to-number conversion: `~true` → `1`, `~false` → `0`
- Expected output: `246` (~"123" * 2)

### 09_builtin_len.xmas
Tests the built-in `len()` function
- Expected output: `5` (length of array [1,2,3,4,5])

### 10_array_slicing.xmas
Tests array slicing with ranges
- Expected output: `[1, 2, 3]` (slice from index 1 to 4)

### 11_operator_precedence.xmas
Tests that operator precedence is correct
- Expected output: `7` (1 + 2 * 3 = 7, not 9)

### 12_nested_functions.xmas
Tests nested function calls
- Expected output: `30` (double(add(5, 10)))

### 13_complex_expression.xmas
Tests complex nested expressions with conditionals
- Expected output: `25` (if(10 > 5, 10*2, 5*2) + 5)

### 14_return_value.xmas
Tests the return value (`_`) mechanism
- Expected output: `42` (after setting _ = 42)

### 15_function_shorthand.xmas
Tests function definitions with expression bodies (shorthand)
- Expected output: `6` (addOne(5))

### 16_array_indexing.xmas
Tests array element access by index
- Expected output: `30` (arr[2])

### 17_string_indexing.xmas
Tests string character access by index
- Expected output: `"l"` (str[2] from "hello")

### 18_multiple_statements.xmas
Tests programs with multiple statements
- Expected output: `6` (sum of 1 + 2 + 3)

### 19_blocks.xmas
Tests block expressions
- Expected output: `15` (block that computes 5 + 10)

### 20_comparison_chaining.xmas
Tests multiple comparison operations
- Expected output: `1` (true, because 5 < 15)

## Test Structure

Each test file:
1. Contains xmas code that exercises specific features
2. Has a final expression that produces the expected output
3. Is automatically run by the test suite
4. Verifies that the output matches the expected value
5. **Verifies all intermediate operations** - The test suite checks all variables and operations, not just the final output

### Comprehensive Verification

The integration tests don't just check the final output - they verify **all operations** in each test file:

- **01_basic_arithmetic.xmas**: Verifies all 5 arithmetic operations (+, -, *, /, %)
- **02_comparison.xmas**: Verifies all 6 comparison operations (==, !=, <, >, <=, >=)
- **03_arrays.xmas**: Verifies array creation, indexing, slicing, and concatenation
- **04_functions.xmas**: Verifies both function definitions and calls
- **05_conditionals.xmas**: Verifies both true and false branches
- **06_loops.xmas**: Verifies both sum and product accumulation
- And so on for all tests...

This ensures that every operation in the test file actually works, not just the final expression.

## Adding New Tests

To add a new test:

1. Create a new `.xmas` file in `tests/integration/`
2. Name it with a number prefix (e.g., `21_new_feature.xmas`)
3. Write xmas code that tests the feature
4. Add a test function in `tests/integration_tests.rs`:

```rust
#[test]
fn test_21_new_feature() {
    let result = run_xmas_file("tests/integration/21_new_feature.xmas").unwrap();
    assert_eq!(result, Value::Number(42.0), "Expected output: 42");
}
```

## Test Coverage

These integration tests cover:
- ✅ All arithmetic operators
- ✅ All comparison operators
- ✅ Array operations (literals, indexing, slicing, concatenation)
- ✅ String operations
- ✅ Function definitions and calls
- ✅ Control flow (if, for)
- ✅ Built-in functions (len)
- ✅ Type conversion
- ✅ Operator precedence
- ✅ Blocks and return values
- ✅ Complex nested expressions
