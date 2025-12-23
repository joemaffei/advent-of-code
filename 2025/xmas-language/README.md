# xmas Language - Lexer Documentation

Welcome! This guide will help you understand the lexer (tokenizer) for the xmas programming language, even if you're completely new to Rust.

## Table of Contents

1. [What is a Lexer?](#what-is-a-lexer)
2. [Rust Basics for Beginners](#rust-basics-for-beginners)
3. [Project Structure](#project-structure)
4. [How the Lexer Works](#how-the-lexer-works)
5. [Running the Code](#running-the-code)
6. [Testing](#testing)

## What is a Lexer?

A **lexer** (also called a **tokenizer**) is the first step in understanding a programming language. Think of it like reading a book:

- When you read, you don't see individual letters - you see **words**
- A lexer does the same thing: it reads code character by character and groups them into **tokens** (the "words" of programming)

**Example:**

```
Code:  x = 5
Tokens: [Identifier("x"), Equals, Number(5)]
```

The lexer breaks down `x = 5` into three meaningful pieces:

1. `x` - an identifier (variable name)
2. `=` - the equals operator
3. `5` - a number

## Rust Basics for Beginners

If you're new to Rust, here are the key concepts you'll see in this code:

### 1. Variables and Mutability

```rust
let x = 5;        // Immutable (can't change)
let mut y = 10;   // Mutable (can change)
```

- `let` creates a variable
- `mut` means the variable can be changed
- Variables are immutable by default (Rust's safety feature)

### 2. Types

Rust is a **statically typed** language - every value has a type:

```rust
let number: i32 = 42;           // Integer
let text: String = "hello".to_string();  // String
let floating: f64 = 3.14;        // Float (decimal number)
```

### 3. Enums (Enumerations)

An enum is like a "choice" type - it can be one of several options:

```rust
enum Token {
    Number(f64),      // A number token with a value
    Plus,             // Just the plus operator
    Identifier(String), // An identifier with a name
}
```

Think of it like a multiple-choice question where each option can carry data.

### 4. Structs

A struct groups related data together:

```rust
struct Lexer {
    source: Vec<char>,  // The code to tokenize
    current: usize,     // Current position
}
```

Like a container that holds multiple pieces of information.

### 5. Methods (Functions on Types)

Methods are functions that belong to a type:

```rust
impl Lexer {
    fn new(source: &str) -> Self {
        // Creates a new Lexer
    }

    fn next_token(&mut self) -> Token {
        // Gets the next token
    }
}
```

- `&self` - borrows (reads) the struct
- `&mut self` - mutably borrows (can modify) the struct
- `Self` - refers to the type (Lexer in this case)

### 6. Match Expressions

`match` is like a super-powered if/else - it checks which case matches:

```rust
match character {
    '+' => Token::Plus,
    '-' => Token::Minus,
    '0'..='9' => self.read_number(),
    _ => Token::Unknown,  // Default case
}
```

### 7. Vectors (Vec)

A vector is like an array that can grow:

```rust
let mut tokens = Vec::new();  // Create empty vector
tokens.push(Token::Plus);     // Add item
let first = tokens[0];        // Get first item
```

### 8. Strings

Rust has two string types:

- `&str` - string slice (reference, can't change)
- `String` - owned string (can change)

```rust
let text: &str = "hello";           // String slice
let owned: String = "world".to_string();  // Owned string
```

### 9. Option and Result

Rust uses these for error handling:

```rust
Option<T>  // Can be Some(value) or None
Result<T, E>  // Can be Ok(value) or Err(error)
```

### 10. Testing

Tests go in a `#[cfg(test)]` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(expected, actual);
    }
}
```

## Project Structure

```
xmas-language/
├── Cargo.toml          # Project configuration (like package.json)
├── README.md           # This file
├── SPECIFICATIONS.md   # Language specification
└── src/
    ├── lib.rs         # Library entry point (exports modules)
    ├── main.rs        # Executable entry point (test program)
    └── lexer.rs       # The lexer implementation
```

### Cargo.toml Explained

This is Rust's project file (like `package.json` in Node.js):

```toml
[package]
name = "xmas-language"    # Project name
version = "0.1.0"         # Version number
edition = "2021"          # Rust edition (language version)

[lib]                     # Library configuration
name = "xmas_language"    # Name used in code (hyphens become underscores)
path = "src/lib.rs"       # Where the library code is

[[bin]]                   # Executable configuration
name = "xmas"             # Name of the executable
path = "src/main.rs"      # Where the main code is
```

## How the Lexer Works

### Step 1: Create a Lexer

```rust
let mut lexer = Lexer::new("x = 5");
```

This creates a new lexer with the source code. The lexer stores:

- The source code as a vector of characters
- Current position in the code
- Line and column numbers (for error messages)

### Step 2: Read Tokens

The lexer reads the code character by character:

1. **Skip whitespace** - spaces and tabs are ignored
2. **Check current character** - what is it?
3. **Match to token type** - is it a number? operator? keyword?
4. **Return the token** - give back the token found

### Token Types

The lexer recognizes these token types:

#### Keywords

- `if`, `for`, `of`, `input`, `len`

#### Operators

- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `<`, `>`, `<=`, `>=`, `==`
- Special: `~` (type conversion), `|>` (pipe)

#### Literals

- **Numbers**: `5`, `3.14`, `-10`
- **Strings**: `"hello"`, `"world"`
- **Identifiers**: `x`, `myVariable`, `function_name`

#### Punctuation

- `(`, `)`, `{`, `}`, `[`, `]`, `,`, `=`, `.`, `..`, `_`

#### Special

- `//` comments
- Newlines (`\n`)

### Example: Tokenizing `x = 5`

```
Position 0: 'x'
  → It's a letter, so read identifier
  → Result: Identifier("x")

Position 1: ' ' (space)
  → Skip whitespace

Position 2: '='
  → It's an equals sign
  → Check next character: ' ' (not '=')
  → Result: Equals

Position 3: ' ' (space)
  → Skip whitespace

Position 4: '5'
  → It's a digit, so read number
  → Result: Number(5.0)
```

### Key Functions

#### `next_token()`

Gets the next token from the source code. This is the main function.

#### `advance()`

Moves forward one character and returns it. Like reading the next letter.

#### `peek()`

Looks at the current character without moving forward. Like "peeking" ahead.

#### `match_char(expected)`

Checks if the next character matches, and if so, consumes it. Used for two-character operators like `==`, `<=`, `|>`.

#### `read_number()`

Reads digits (and optionally a decimal point) to form a number token.

#### `read_string()`

Reads characters between quotes to form a string token. Handles escape sequences like `\n`.

#### `read_identifier_or_keyword()`

Reads letters, digits, and underscores. Then checks if it's a keyword or just an identifier.

#### `tokenize()`

Reads all tokens until the end of the file and returns them as a vector.

## Running the Code

### Prerequisites

You need Rust installed. Install it from [rustup.rs](https://rustup.rs/).

### Build and Run

```bash
# Navigate to the project directory
cd xmas-language

# Build the project
cargo build

# Run the test program
cargo run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### What `cargo run` Does

1. Compiles the code (turns Rust into machine code)
2. Runs the `main.rs` program
3. Shows the output

You should see something like:

```
Tokenizing xmas code:
a = 1
b = 2
...

Tokens:
  0: Identifier("a")
  1: Equals
  2: Number(1.0)
  ...
```

## Testing

### Running Tests

```bash
cargo test
```

This runs all tests in the `#[cfg(test)]` modules.

### Test Structure

Each test:

1. Creates a lexer with sample code
2. Calls `next_token()` or `tokenize()`
3. Checks that the result matches what we expect

### Example Test

```rust
#[test]
fn test_assignment() {
    let mut lexer = Lexer::new("a = 1");
    assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token(), Token::Equals);
    assert_eq!(lexer.next_token(), Token::Number(1.0));
}
```

This test:

- Creates a lexer with `"a = 1"`
- Checks that the first token is `Identifier("a")`
- Checks that the second token is `Equals`
- Checks that the third token is `Number(1.0)`

### All Tests

The lexer includes tests for:

- ✅ Simple tokens (parentheses, brackets, operators)
- ✅ Comparison operators (`<`, `<=`, `==`, etc.)
- ✅ Special operators (`~`, `|>`, `..`)
- ✅ Numbers (integers and floats)
- ✅ Strings (with escape sequences)
- ✅ Keywords
- ✅ Identifiers
- ✅ Comments
- ✅ Whitespace handling
- ✅ Newlines
- ✅ Complex expressions (functions, conditionals, loops)
- ✅ Array literals and slicing
- ✅ Input access patterns

## Common Questions

### Q: What does `&mut` mean?

**A:** It means "mutable reference" - you're borrowing the value and can modify it. The `&` means reference (borrow), and `mut` means mutable (can change).

### Q: Why `Vec<char>` instead of `String`?

**A:** We need to access individual characters by index and iterate character by character. `Vec<char>` makes this easier than working with `String` directly.

### Q: What's the difference between `Token::Number(5.0)` and `Token::Number(5)`?

**A:** In Rust, `5` is an integer (`i32`) and `5.0` is a float (`f64`). Our `Token::Number` stores `f64` (floating point) to handle both integers and decimals.

### Q: Why do we skip whitespace?

**A:** Whitespace (spaces, tabs) doesn't affect the meaning of the code in xmas language. `x=5` and `x = 5` mean the same thing. So we ignore whitespace to simplify tokenization.

### Q: What happens with invalid characters?

**A:** Currently, the lexer skips unknown characters and tries the next one. In a full compiler, you'd want to report an error.

## The Parser

The parser is the second phase of the compiler/interpreter. It takes tokens from the lexer and builds an **Abstract Syntax Tree (AST)** - a tree structure that represents the structure of your program.

### What is an AST?

Think of an AST like a sentence diagram:

- The lexer gives us words (tokens)
- The parser groups those words into phrases and sentences (expressions and statements)
- The AST shows the relationships between them

**Example:**

```
Code: x = 1 + 2 * 3

AST:
  Stmt::Assign {
    name: "x",
    value: Expr::Binary {
      left: Expr::Number(1),
      op: Plus,
      right: Expr::Binary {
        left: Expr::Number(2),
        op: Star,
        right: Expr::Number(3)
      }
    }
  }
```

### How the Parser Works

The parser uses **recursive descent parsing** - it starts at the top level and recursively parses smaller parts:

1. **Parse Program** → List of statements
2. **Parse Statement** → Assignment, function definition, or expression
3. **Parse Expression** → With operator precedence
4. **Parse Primary** → Literals, identifiers, function calls, etc.

### Operator Precedence

The parser respects operator precedence (order of operations):

1. **Unary**: `~` (type conversion)
2. **Indexing**: `[]` (array access)
3. **Multiplication/Division**: `*`, `/`, `%`
4. **Addition/Subtraction**: `+`, `-`
5. **Comparison**: `<`, `>`, `<=`, `>=`, `==`
6. **Pipe**: `|>` (function composition)

### AST Structure

The AST has two main types:

#### Expressions (`Expr`)

Things that produce a value:

- `Number(5.0)` - number literal
- `String("hello")` - string literal
- `Identifier("x")` - variable reference
- `Binary { left, op, right }` - binary operation
- `Call { callee, args }` - function call
- `Index { array, index }` - array access
- And more...

#### Statements (`Stmt`)

Things that do something:

- `Assign { name, value }` - variable assignment
- `Return { value }` - return value assignment (`_ = ...`)
- `Function { name, params, body }` - function definition
- `Expr(expr)` - expression statement

### Using the Parser

```rust
use xmas_language::{Lexer, Parser};

let code = "x = 1 + 2";
let mut lexer = Lexer::new(code);
let tokens = lexer.tokenize();
let mut parser = Parser::new(tokens);
let ast = parser.parse().unwrap();
```

### Parser Features

The parser handles all syntax from the xmas language specification:

✅ **Expressions**

- Numbers, strings, identifiers
- Arithmetic operations (`+`, `-`, `*`, `/`, `%`)
- Comparison operations (`<`, `>`, `<=`, `>=`, `==`)
- Unary operators (`~` for type conversion)
- Pipe operator (`|>`)
- Function calls
- Array literals and indexing/slicing
- Blocks (code in `{ }`)
- Parentheses for grouping

✅ **Statements**

- Variable assignments (`x = 5`)
- Return value assignments (`_ = value`)
- Function definitions (`name(params) = body`)
- Expression statements

✅ **Special Constructs**

- Conditionals (`if(condition, trueBlock, falseBlock)`)
- Loops (`for(variable of array, block, initialValue)`)
- Built-in functions (`len(...)`)
- Input access (`input[0]`, `input[0, 5]`, etc.)

### Testing

The parser includes 23 comprehensive unit tests covering:

- All expression types
- All statement types
- Operator precedence
- Complex nested expressions
- Array operations
- Function definitions and calls
- Control flow (if, for)

Run the tests:

```bash
cargo test --lib parser::tests
```

## The Interpreter

The interpreter is the final phase - it takes the AST and actually **executes** the program, evaluating expressions and executing statements.

### What is an Interpreter?

An interpreter:

- Takes the AST (tree structure)
- Evaluates expressions to produce values
- Executes statements to perform actions
- Manages variables and functions
- Handles runtime behavior

Think of it like a calculator that understands your program structure.

### How the Interpreter Works

The interpreter has three main components:

1. **Value Types** - What data can exist at runtime
2. **Environment** - Where variables and functions are stored
3. **Evaluation** - How expressions become values

### Value Types

The xmas language has these runtime value types:

```rust
enum Value {
    Number(f64),           // Numbers: 5, 3.14
    String(String),       // Strings: "hello"
    Array1D(Vec<Value>),  // 1D arrays: [1, 2, 3]
    Array2D(Vec<Vec<Value>>), // 2D arrays: [[1, 2], [3, 4]]
}
```

Since everything is a list in xmas, arrays are fundamental. Strings are conceptually lists of characters.

### Environment

The interpreter maintains:

- **Variables** - Global variable storage (HashMap)
- **Functions** - User-defined functions
- **Input** - The `input` variable (2D character array)
- **Return Value** - The `_` (underscore) return value

### Expression Evaluation

The interpreter evaluates expressions recursively:

1. **Literals** → Direct values
   - `5` → `Value::Number(5.0)`
   - `"hello"` → `Value::String("hello")`

2. **Identifiers** → Look up in environment
   - `x` → Get value from variables HashMap

3. **Operators** → Evaluate operands, apply operation
   - `a + b` → Evaluate `a`, evaluate `b`, add them

4. **Function Calls** → Evaluate arguments, call function
   - `add(1, 2)` → Evaluate args, execute function body

5. **Array Operations** → Index or slice arrays
   - `arr[0]` → Get element at index 0
   - `arr[1..5]` → Get slice from 1 to 5

### Statement Execution

Statements perform actions:

- **Assignment** (`x = 5`) → Store value in variables
- **Return** (`_ = value`) → Set return value
- **Function Definition** → Store function in functions HashMap
- **Expression Statement** → Evaluate expression (for side effects)

### Built-in Functions

The interpreter implements these built-ins:

#### `if(condition, trueBlock, falseBlock)`

Conditional execution:

```xmas
if(x == 5, 10, 20)  // Returns 10 if x == 5, else 20
```

#### `for(variable of array, block, initialValue)`

Iteration with accumulation:

```xmas
for(n of [1, 2, 3], { _ = _ + n }, 0)  // Sums array: 6
```

#### `len(value)`

Get length:

- `len([1, 2, 3])` → `3`
- `len("hello")` → `5`
- `len(input)` → `[numLines, numColumns]` (for 2D arrays)

### Array Operations

#### Indexing

```xmas
arr = [10, 20, 30]
arr[1]  // Returns 20
```

#### Slicing

```xmas
arr = [1, 2, 3, 4, 5]
arr[1..4]  // Returns [2, 3, 4]
arr[1..]   // Returns [2, 3, 4, 5]
arr[..3]   // Returns [1, 2, 3]
```

#### Concatenation

```xmas
[1, 2] + [3, 4]  // Returns [1, 2, 3, 4]
```

### Input Handling

The `input` variable is a 2D character array:

```rust
interpreter.set_input("abc\ndef");
// input[0] → ['a', 'b', 'c']
// input[0, 1] → 'b'
// input[.., 1] → ['b', 'e'] (column 1)
```

### Using the Interpreter

```rust
use xmas_language::{Lexer, Parser, Interpreter};

let code = "x = 5\ny = x + 10";

// Step 1: Tokenize
let mut lexer = Lexer::new(code);
let tokens = lexer.tokenize();

// Step 2: Parse
let mut parser = Parser::new(tokens);
let program = parser.parse()?;

// Step 3: Interpret
let mut interpreter = Interpreter::new();
let result = interpreter.interpret(&program)?;

// Access variables
println!("x = {:?}", interpreter.variables.get("x"));
```

### Interpreter Features

The interpreter handles all language features:

✅ **Expressions**

- All operators (arithmetic, comparison, unary, pipe)
- Function calls
- Array operations (indexing, slicing, concatenation)
- Type conversion (`~`)

✅ **Statements**

- Variable assignments
- Return value assignments
- Function definitions
- Expression statements

✅ **Control Flow**

- Conditionals (`if`)
- Loops (`for`)
- Built-in functions (`len`)

✅ **Data Types**

- Numbers and strings
- 1D and 2D arrays
- Input handling

### Testing

The interpreter includes 18 comprehensive unit tests covering:

- All value types
- All operators
- Array operations
- Function definitions and calls
- Control flow
- Built-in functions
- Input access

Run the tests:

```bash
cargo test --lib interpreter::tests
```

## Complete Pipeline

Now you have a complete xmas language implementation:

```
Source Code
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST
    ↓
[Interpreter] → Result
```

**Example:**

```xmas
x = 1 + 2 * 3
```

1. **Lexer**: `[Identifier("x"), Equals, Number(1), Plus, Number(2), Star, Number(3)]`
2. **Parser**: `Stmt::Assign { name: "x", value: Binary { ... } }`
3. **Interpreter**: `variables["x"] = Number(7.0)`

## Next Steps

The xmas language is now fully functional! Possible enhancements:

1. **Better error messages** - Line/column numbers in errors
2. **Standard library** - More built-in functions
3. **File I/O** - Read/write files
4. **REPL** - Interactive read-eval-print loop
5. **Optimization** - Performance improvements

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Comprehensive Rust tutorial
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust through examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive Rust exercises

## Summary

The lexer is like a translator that converts human-readable code into tokens that the computer can understand. It reads character by character, groups them into meaningful units (tokens), and handles all the syntax of the xmas language.

Key takeaways:

- A lexer breaks code into tokens
- Rust uses types, enums, and structs to organize code
- Tests verify the lexer works correctly
- The lexer handles all syntax from the xmas language specification

Happy coding! 🎄
