# Running xmas Code

## Quick Start

### 1. Build the xmas interpreter

```bash
cd xmas-language
cargo build --release
```

This creates the `xmas` executable in `target/release/xmas`.

### 2. Run a xmas program file

```bash
# Using cargo run (development)
cargo run --bin xmas -- program.xmas

# Using the built executable (production)
./target/release/xmas program.xmas
```

### 3. Run with input file

For programs that use the `input` variable:

```bash
cargo run --bin xmas -- program.xmas -i input.txt
# or
./target/release/xmas program.xmas -i input.txt
```

### 4. Run from stdin

You can also pipe code directly:

```bash
echo 'x = 5\nx * 2' | cargo run --bin xmas
```

## Examples

### Example 1: Hello World

Create `hello.xmas`:

```xmas
message = "Hello, xmas language!"
message
```

Run it:

```bash
cargo run --bin xmas -- hello.xmas
# Output: Hello, xmas language!
```

### Example 2: Sum Array

Create `sum.xmas`:

```xmas
arr = [1, 2, 3, 4, 5]
sum = for(n of arr, { _ = _ + n }, 0)
sum
```

Run it:

```bash
cargo run --bin xmas -- sum.xmas
# Output: 15
```

### Example 3: Using Input

Create `process.xmas`:

```xmas
first_line = input[0]
len(first_line)
```

Create `input.txt`:

```
abc
def
```

Run it:

```bash
cargo run --bin xmas -- process.xmas -i input.txt
# Output: 3
```

## Command Line Options

```
xmas [file.xmas] [-i|--input input.txt] [-d|--debug]

Arguments:
  file.xmas          Path to xmas source file (or read from stdin if omitted)
  -i, --input FILE   Path to input file for the `input` variable
  -d, --debug        Enable debug mode (prints execution trace to stderr)
```

### Debug Mode

The `-d` or `--debug` flag enables verbose debug output that shows:

- **Variable assignments**: When variables are assigned or updated
- **Operations**: Arithmetic and logical operations with before/after values
- **Conditionals**: `if` statement conditions and their results
- **Loops**: `for` loop iterations showing the current element

Debug output is printed to `stderr`, so it won't interfere with program output on `stdout`.

Example:

```bash
cargo run --bin xmas -- program.xmas -i input.txt --debug
```

Debug output looks like:
```
DEBUG: x: 0 → 5
DEBUG:   if (x == 5): true
DEBUG:   for n: 1
DEBUG:   for n: 2
DEBUG:   for n: 3
```

## Installation (Optional)

To install xmas globally:

```bash
cargo install --path .
```

Then you can run xmas from anywhere:

```bash
xmas program.xmas
```

## Programmatic Usage

You can also use xmas as a library in Rust:

```rust
use xmas_language::{Lexer, Parser, Interpreter};

let code = "x = 5\nx * 2";
let mut lexer = Lexer::new(code);
let tokens = lexer.tokenize();
let mut parser = Parser::new(tokens);
let program = parser.parse()?;
let mut interpreter = Interpreter::new();
let result = interpreter.interpret(&program)?;
println!("Result: {:?}", result);
```

## Tips

- The last expression in your program will be printed as output
- Use `_ = value` to set the return value explicitly
- Comments start with `//`
- Arrays are 0-indexed
- The `input` variable is a 2D character array (lines × characters)
