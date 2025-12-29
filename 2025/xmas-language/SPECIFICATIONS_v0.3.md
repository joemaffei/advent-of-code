# xmas Language Specification v0.3

## Overview

A terse, list-based language designed for solving Advent of Code challenges.

## Data Types

- **List**: The fundamental data structure
- **String**: A list of characters
- **Number**: Integer only (floats to be added in future)
- **Boolean**: `true` or `false`
- **Block**: Code enclosed in `{ }`

## Input

- **`input`**: Global 2D character array (lines × characters, newlines excluded)
  - `input` - entire file
  - `input[i]` - line at index i
  - `input[i, n]` - character at position n in line i
  - `input[.., n]` - column n (all characters at position n across lines)

## Array Slicing

- `array[start..end]` - slice from start to end (exclusive of end)
- `array[start..]` - from start to end of array
- `array[..end]` - from beginning to end
- `array[..]` - entire array

2D arrays use same syntax:

- `input[i, 1..]` - line i from index 1 onward
- `input[i, ..5]` - line i from 0 to 5

## Range Literals

- **`[start..end]`**: Creates an array of integers from `start` to `end` (inclusive)

```xmas
[0..5]    // [0, 1, 2, 3, 4, 5]
[5..10]   // [5, 6, 7, 8, 9, 10]
[10..5]   // [10, 9, 8, 7, 6, 5] (reverse order)
```

Range literals are distinct from array slicing:

- `[0..5]` - range literal (creates array)
- `arr[0..5]` - array slicing (extracts elements)

## Variables

- **Scope**: Always global
- **Assignment**: `variable = value`

```xmas
a = 1
name = "hello"
flag = true
```

## Operators

### Arithmetic

JavaScript syntax: `+`, `-`, `*`, `/`, `%`

- All numbers are integers
- `/` performs **integer division** (truncates toward zero)
- `%` is modulo (remainder after integer division)

```xmas
10 / 3 == 3    // integer division
10 % 3 == 1   // modulo
```

### Comparison

JavaScript syntax: `<`, `>`, `<=`, `>=`, `==`

- Comparison operators return **boolean** values (`true` or `false`)
- Only `==` for equality (no `===`)

```xmas
5 < 10 == true
5 == 5 == true
```

### Logical Operators

JavaScript-style logical operators:

- **`&&`**: Logical AND (returns last truthy or first falsy value)
- **`||`**: Logical OR (returns first truthy or last falsy value)
- **`!`**: Logical NOT (returns boolean)

```xmas
true && false == false
true || false == true
!true == false
```

### Type Conversion

- **`~`**: Convert string to number, or boolean to number

```xmas
~"123" == 123  // true
num = "456"
~num == 456    // true

// Boolean conversion: true -> 1, false -> 0
~true == 1     // true
~false == 0    // true

// Useful for converting boolean expressions to numbers
a = 12
a = ~(a < 20)  // 1 (since 12 < 20 is true)
// Equivalent to: a = if(a < 20, 1, 0)
```

### Composition

- **`|>`**: Pipe operator for function composition

## Blocks

Delimited by `{ }`

```xmas
{
  x = 5
  y = 10
}
```

## Functions

### Definition

```xmas
functionName(arg1, arg2) = block
```

### Return Value

- **`_`**: Underscore represents the return value

```xmas
add(a, b) = { _ = a + b }
```

### Shorthand

Single expressions don't require explicit block:

```xmas
addOne(x) = add(x, 1)
// equivalent to: addOne(x) = { _ = add(x, 1) }
```

### Function Composition

```xmas
addOne(x) = { _ = x + 1 }
addTwo(x) = addOne |> addOne
// addTwo(5) → 7
```

## Conditionals

Function-style syntax:

```xmas
if(condition, trueBlock, falseBlock?)
```

- **falseBlock**: Optional third argument. If omitted and condition is false, returns `null` or undefined behavior.

Examples:

```xmas
if(x == 5, {
  y = 10
}, {
  y = 20
})

// Without false block (optional)
if(x > 0, {
  y = x
})
```

## Iteration

```xmas
for(variable of array, block, initialValue?)
```

- **initialValue**: Optional third argument. **Required only when the for loop returns a value** (expression form). Can be a value or a block
  - If block: Execute first to compute initial `_`
  - If value: Use directly as initial `_`
  - If omitted: The for loop can only be used for side effects (statement form)

### Statement Form (side effects)

When used for side effects only, the third argument is optional:

```xmas
arr = [1, 2, 3]
for(n of arr, {
  print(n)
})
// or with a dummy initial value
for(n of arr, {
  print(n)
}, 0)
```

### Expression Form (accumulation)

When the `for` loop returns a value, the third argument **must** be provided. The `for` loop supports accumulation patterns where `_` represents the accumulator:

#### Numeric Accumulation

```xmas
sum = for(n of [1, 2, 3], {
  _ = _ + n
}, 0)
// sum == 6
```

#### Array Building

```xmas
doubled = for(n of [1, 2, 3], {
  _ = _ + [n * 2]
}, [])
// doubled == [2, 4, 6]
```

#### String Building

```xmas
chars = ['h', 'e', 'l', 'l', 'o']
word = for(c of chars, {
  _ = _ + c
}, "")
// word == "hello"
```

#### Complex State Accumulation

```xmas
// Accumulate into an array representing [position, count]
state = for(item of items, {
  newPos = _[0] + item
  newCount = _[1] + 1
  _ = [newPos, newCount]
}, [0, 0])
// state == [finalPosition, totalCount]
```

## Built-in Functions

### len()

Returns length of array:

- **1D array**: Returns integer
- **2D array**: Returns `[lines, columns]`

```xmas
len([1, 2, 3])      // 3
len(input)          // [numLines, numColumns]
len(input[0])       // characters in first line
```

## Array Operations

### Concatenation

The `+` operator concatenates arrays:

```xmas
[1, 2] + [3, 4]     // [1, 2, 3, 4]
_ = _ + [newItem]   // append to array
```

### String Concatenation

Since strings are lists of characters, the `+` operator also concatenates strings:

```xmas
"hello" + " " + "world"  // "hello world"
hello = "Hello"
world = "World"
combined = hello + " " + world  // "Hello World"
```

String concatenation can be used in accumulation patterns:

```xmas
chars = ['h', 'e', 'l', 'l', 'o']
word = for(c of chars, {
  _ = _ + c
}, "")
// word == "hello"
```

### Method Calls

#### `.rows()`

Converts a 2D array into a 1D array where each element is a row (1D array):

```xmas
// input is a 2D array: [['a', 'b'], ['c', 'd']]
rows = input.rows()
// rows is a 1D array: [['a', 'b'], ['c', 'd']]
// Each element of rows is itself a 1D array

// Useful for iterating over lines:
for(line of input.rows(), {
  // line is a 1D array representing one line
  // process line...
}, 0)
```

**Note**: For 2D arrays, `input.rows()` is equivalent to accessing rows individually, but provides a convenient way to iterate over all rows as a 1D array.

## Changelog

### v0.3 (Current)

**New Features:**

- Added `.rows()` method for 2D arrays to convert to 1D array of rows
- Enhanced documentation for string concatenation
- Added comprehensive examples for `for` loop accumulation patterns:
  - Numeric accumulation
  - Array building
  - String building
  - Complex state accumulation
- Extended `~` operator to convert booleans to numbers: `true` → `1`, `false` → `0`

### v0.2

**Breaking Changes:**

- Numbers are now integers only (was: integers or floats)
- Division (`/`) now performs integer division (was: floating-point division)
- Comparison operators now return booleans (was: returned numbers 1.0 or 0.0)

**New Features:**

- Added boolean type with `true` and `false` literals
- Added logical operators: `&&` (AND), `||` (OR), `!` (NOT)
- Added range literals: `[start..end]` creates inclusive integer arrays

### v0.1 (Initial)

- Initial language specification
- Basic data types (numbers as f64, strings, arrays)
- Arithmetic, comparison, and type conversion operators
- Functions, conditionals, and iteration
- Built-in functions (`len`)
- Input handling
