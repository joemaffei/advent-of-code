# Advent of Code 2025 - xmas Language Experiment

This repository contains an experimental implementation of a custom programming language called **xmas**, designed specifically for solving [Advent of Code](https://adventofcode.com/) challenges.

## What is This?

This is a learning experiment that combines two goals:

1. **Building a Programming Language**: Implementing a complete language from scratch in Rust, including:
   - Lexer (tokenizer) - converts source code into tokens
   - Parser - builds an Abstract Syntax Tree (AST) from tokens
   - Interpreter - executes the AST to run programs

2. **Solving Advent of Code**: Using the custom language to solve AoC 2025 puzzles, testing the language's expressiveness and practicality.

## The xmas Language

**xmas** is a terse, list-based language designed for algorithmic problem-solving. Its key design principles:

- **Everything is a list**: All data structures are fundamentally lists
- **Minimal syntax**: Designed to be concise and readable
- **AoC-focused**: Built-in support for common AoC patterns (2D input parsing, array operations, etc.)

### Key Features

- **Data Types**: Lists, strings (lists of characters), integers, booleans
- **Array Operations**: Indexing, slicing, concatenation, 2D array support
- **Functions**: First-class functions with closures
- **Control Flow**: Conditionals (`if`), loops (`for`), built-in functions (`len`)
- **Input Handling**: Built-in `input` variable as a 2D character array
- **Range Literals**: `[0..5]` creates `[0, 1, 2, 3, 4, 5]`
- **Type Conversion**: `~` operator for converting between types

### Example

```xmas
// Sum all numbers in an array
sum(arr) = {
    _ = for(n of arr, { _ = _ + n }, 0)
}

// Count occurrences of a value
count(arr, value) = {
    _ = for(x of arr, { _ = _ + if(x == value, 1, 0) }, 0)
}

// Process input lines
numLines = len(input)[0]
indices = [0..numLines - 1]
result = for(i of indices, {
    line = input[i]
    // process line...
}, initialValue)
```

## Project Structure

```
.
├── xmas-language/          # The language implementation (Rust)
│   ├── src/               # Source code
│   │   ├── lexer.rs      # Tokenizer
│   │   ├── parser.rs      # AST builder
│   │   ├── interpreter.rs # Executor
│   │   └── ...
│   ├── tests/             # Test suite
│   ├── examples/          # Example programs
│   └── SPECIFICATIONS.md  # Language specification
│
└── aoc-2025/              # Advent of Code solutions
    └── day-01/
        ├── part1.xmas     # Solution in xmas language
        ├── part2.xmas     # Solution in xmas language
        └── input.txt      # Puzzle input
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)

### Building the Language

```bash
cd xmas-language
cargo build --release
```

The executable will be at `xmas-language/target/release/xmas`.

### Running a Program

```bash
cd xmas-language
cargo run -- examples/hello.xmas
```

Or using the built executable:

```bash
./target/release/xmas examples/hello.xmas
```

### Running Tests

```bash
cd xmas-language
cargo test
```

### Running AoC Solutions

```bash
cd xmas-language
cargo run -- ../aoc-2025/day-01/part1.xmas < ../aoc-2025/day-01/input.txt
```

## Language Documentation

- **Full Specification**: See `xmas-language/SPECIFICATIONS_v0.3.md`
- **Implementation Details**: See `xmas-language/README.md`
- **Usage Guide**: See `xmas-language/USAGE.md`

## Current Status

### Language Implementation

✅ **Complete**:
- Lexer (tokenizer)
- Parser (AST builder)
- Interpreter (executor)
- Comprehensive test suite (40+ tests)

✅ **Language Features**:
- Variables and assignments
- Functions and closures
- Conditionals and loops
- Array operations (indexing, slicing, concatenation)
- 2D array support
- Built-in functions (`len`, `if`, `for`)
- Type conversion
- Range literals
- Booleans and logical operators
- Integer division
- `.rows()` method for 2D arrays
- String concatenation

### AoC Solutions

- ✅ Day 1, Part 1: Safe Dial Password
- ✅ Day 1, Part 2: Safe Dial Password (Method 0x434C49434B)

## Motivation

This project serves multiple purposes:

1. **Learning**: Understanding how programming languages work under the hood
2. **Practice**: Applying Rust skills to a non-trivial project
3. **Fun**: Creating something unique and solving puzzles with it
4. **Challenge**: Building a language that's both expressive and practical for AoC

## Design Decisions

- **Rust**: Chosen for safety, performance, and pattern matching (great for AST handling)
- **List-based**: Inspired by functional languages, simplifies data manipulation
- **AoC-focused**: Built-in `input` handling and array operations for common patterns
- **Interpreted**: Easier to implement and iterate on than a compiler

## Future Enhancements

Potential improvements (not currently planned):

- Better error messages with line/column numbers
- Standard library of utility functions
- REPL (interactive mode)
- Performance optimizations
- Additional built-in functions
- Float support
- File I/O operations

## License

This is a personal learning project. Feel free to use it as a reference or starting point for your own experiments.

## Resources

- [Advent of Code 2025](https://adventofcode.com/2025)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Crafting Interpreters](https://craftinginterpreters.com/) - Excellent resource for building interpreters

---

**Note**: This is an experimental project. The language specification and implementation may change as I learn more and encounter new requirements from AoC puzzles.
