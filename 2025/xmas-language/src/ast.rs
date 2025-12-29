/// Abstract Syntax Tree (AST) for the xmas language.
///
/// The AST represents the structure of a program as a tree of nodes.
/// Think of it like a sentence diagram - it shows how the code is organized.

/// Represents any expression in the language.
///
/// Expressions are things that produce a value, like:
/// - `5` (a number)
/// - `x + y` (addition)
/// - `add(1, 2)` (function call)
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A number literal: `5` (integer)
    Number(i64),

    /// A boolean literal: `true`, `false`
    Boolean(bool),

    /// A string literal: `"hello"`
    String(String),

    /// A variable or function name: `x`, `myVar`
    Identifier(String),

    /// The special `input` variable
    Input,

    /// The return value `_`
    ReturnValue,

    /// Array literal: `[1, 2, 3]`
    Array(Vec<Expr>),

    /// Range literal: `[0..5]` creates `[0, 1, 2, 3, 4, 5]` (inclusive)
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },

    /// Unary operator: `~x` (type conversion)
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    /// Binary operator: `a + b`, `x == y`
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    /// Pipe operator: `f |> g`
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Function call: `add(1, 2)`
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    /// Array indexing: `arr[0]`
    Index {
        array: Box<Expr>,
        index: Vec<IndexExpr>,
    },

    /// Built-in function call: `len(arr)`
    Builtin {
        name: String,
        args: Vec<Expr>,
    },

    /// Method call: `array.rows()`
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },

    /// Block expression: `{ x = 5; y = 10 }`
    Block(Vec<Stmt>),
}

/// Represents a statement (something that does something, not necessarily returns a value).
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// Variable assignment: `x = 5`
    Assign {
        name: String,
        value: Expr,
    },

    /// Assignment operator: `x += 5`, `x -= 5`, etc.
    AssignOp {
        name: String,
        op: BinaryOp,
        value: Expr,
    },

    /// Return value assignment: `_ = x + y` or `_name = x + y`
    Return {
        name: Option<String>,  // None for `_`, Some(name) for `_name`
        value: Expr,
    },

    /// Function definition: `add(a, b) = { _ = a + b }`
    Function {
        name: String,
        params: Vec<String>,
        body: Expr,
    },

    /// Expression statement (function call, etc.)
    Expr(Expr),
}

/// Unary operators (operate on one value).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    /// Type conversion: `~"123"`
    Tilde,
    /// Logical NOT: `!true`
    Bang,
}

/// Binary operators (operate on two values).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Comparison
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    EqualEqual,

    // Logical
    And,      // &&
    Or,       // ||
}

/// Represents an index or slice in array access.
///
/// Examples:
/// - `arr[0]` -> IndexExpr::Single(Expr::Number(0))
/// - `arr[1..5]` -> IndexExpr::Range(Some(1), Some(5))
/// - `arr[..5]` -> IndexExpr::Range(None, Some(5))
/// - `arr[1..]` -> IndexExpr::Range(Some(1), None)
/// - `arr[..]` -> IndexExpr::Range(None, None)
#[derive(Debug, Clone, PartialEq)]
pub enum IndexExpr {
    /// Single index: `arr[0]`
    Single(Expr),

    /// Range slice: `arr[1..5]`, `arr[..]`, etc.
    Range {
        start: Option<Expr>,
        end: Option<Expr>,
    },
}

/// The root of an AST - a program is a list of statements.
pub type Program = Vec<Stmt>;
