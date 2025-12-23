# xmas Language Specification v0.1

## Overview

A terse, list-based language designed for solving Advent of Code challenges.

## Data Types

- **List**: The fundamental data structure
- **String**: A list of characters
- **Number**: Integer or float
- **Block**: Code enclosed in `{ }`

## Input

- **`input`**: Global 2D character array (lines × characters, newlines excluded)
  - `input` - entire file
  - `input[i]` - line at index i
  - `input[i, n]` - character at position n in line i
  - `input[.., n]` - column n (all characters at position n across lines)

## Array Slicing

- `array[start..end]` - slice from start to end
- `array[start..]` - from start to end of array
- `array[..end]` - from beginning to end
- `array[..]` - entire array

2D arrays use same syntax:

- `input[i, 1..]` - line i from index 1 onward
- `input[i, ..5]` - line i from 0 to 5

## Variables

- **Scope**: Always global
- **Assignment**: `variable = value`

```xmas
a = 1
name = "hello"
```

## Operators

### Arithmetic

JavaScript syntax: `+`, `-`, `*`, `/`, `%`

### Comparison

JavaScript syntax: `<`, `>`, `<=`, `>=`, `==`

- Only `==` for equality (no `===`)

### Type Conversion

- **`~`**: Convert string to number

```xmas
~"123" == 123  // true
num = "456"
~num == 456    // true
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

### Composition

```xmas
addOne(x) = { _ = x + 1 }
addTwo(x) = addOne |> addOne
// addTwo(5) → 7
```

## Conditionals

Function-style syntax:

```xmas
if(condition, trueBlock, falseBlock)
```

Example:

```xmas
if(x == 5, {
  y = 10
}, {
  y = 20
})
```

## Iteration

```xmas
for(variable of array, block, initialValue)
```

- **initialValue**: Can be a value or a block
  - If block: Execute first to compute initial `_`
  - If value: Use directly as initial `_`

### Statement Form (side effects)

```xmas
arr = [1, 2, 3]
for(n of arr, {
  print(n)
}, 0)
```

### Expression Form (accumulation)

```xmas
doubleArr = for(n of arr, {
  _ = _ + [n * 2]
}, [])
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

## Array Concatenation

The `+` operator concatenates arrays:

```xmas
[1, 2] + [3, 4]     // [1, 2, 3, 4]
_ = _ + [newItem]   // append to array
```
