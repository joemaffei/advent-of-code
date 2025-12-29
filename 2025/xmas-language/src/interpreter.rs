/// Interpreter for the xmas programming language.
///
/// The interpreter executes the AST to run xmas programs.
/// It evaluates expressions and executes statements.

use crate::ast::*;
use std::collections::HashMap;

/// Runtime values in the xmas language.
///
/// Since everything is a list, arrays are the fundamental type.
/// Numbers and strings are also represented, but strings are
/// conceptually lists of characters.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A number (integer)
    Number(i64),

    /// A boolean: true or false
    Boolean(bool),

    /// A string (list of characters)
    String(String),

    /// A 1D array: [1, 2, 3]
    Array1D(Vec<Value>),

    /// A 2D array: [[1, 2], [3, 4]]
    /// Used for the `input` variable
    Array2D(Vec<Vec<Value>>),
}

/// Represents a user-defined function.
#[derive(Debug, Clone)]
struct Function {
    params: Vec<String>,
    body: Expr,
}

/// The interpreter environment (global scope).
///
/// Stores variables and functions.
pub struct Interpreter {
    /// Global variables
    pub variables: HashMap<String, Value>,

    /// User-defined functions
    functions: HashMap<String, Function>,

    /// The input data (2D character array)
    input: Option<Vec<Vec<char>>>,

    /// Current return value (`_`)
    return_value: Option<Value>,

    /// Named return values (`_name`)
    named_returns: HashMap<String, Value>,

    /// Debug mode flag
    debug: bool,

    /// Current indentation level for debug output (in spaces)
    debug_indent: usize,
}

impl Interpreter {
    /// Create a new interpreter.
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            input: None,
            return_value: None,
            named_returns: HashMap::new(),
            debug: false,
            debug_indent: 0,
        }
    }

    /// Enable or disable debug mode.
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Set the input data (from a file or string).
    pub fn set_input(&mut self, input: &str) {
        let lines: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        self.input = Some(lines);
    }

    /// Interpret a complete program.
    pub fn interpret(&mut self, program: &Program) -> Result<Value, String> {
        let mut last_value = Value::Array1D(Vec::new());

        for stmt in program {
            match stmt {
                Stmt::Expr(expr) => {
                    // Expression statements return their value
                    last_value = self.evaluate_expression(expr)?;
                }
                _ => {
                    self.execute_statement(stmt)?;
                }
            }
        }

        // Return the last expression value, or return value if set, or empty array
        Ok(self.return_value.clone().unwrap_or(last_value))
    }

    /// Execute a statement.
    fn execute_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Assign { name, value } => {
                let val = self.evaluate_expression(value)?;
                let old_val = self.variables.get(name).cloned();
                if self.debug {
                    let old_str = old_val
                        .as_ref()
                        .map(|v| self.format_debug_value(v))
                        .unwrap_or_else(|| "undefined".to_string());
                    let new_str = self.format_debug_value(&val);
                    let indent = " ".repeat(self.debug_indent);
                    eprintln!("DEBUG: {}{}: {} → {}", indent, name, old_str, new_str);
                }
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Stmt::AssignOp { name, op, value } => {
                // Check if this is a return value assignment (_ or _name)
                let is_named_return = name.starts_with('_') && name.len() > 1;
                let is_return = name == "_" || is_named_return;

                if is_return {
                    // Handle return value assignment: _ += value or _name += value
                    let return_name = if is_named_return {
                        Some(name[1..].to_string())
                    } else {
                        None
                    };

                    // Get current return value (error if not set, same as _ = _ + value)
                    let current_val = if let Some(return_name) = &return_name {
                        // Named return
                        self.named_returns.get(return_name)
                            .or_else(|| self.return_value.as_ref())
                            .cloned()
                            .ok_or_else(|| format!("No return value set for _{}", return_name))?
                    } else {
                        // Unnamed return
                        self.return_value.clone()
                            .ok_or_else(|| "No return value set".to_string())?
                    };

                    // Evaluate the right-hand side expression
                    let right_val = self.evaluate_expression(value)?;

                    // Perform the operation: _ += y means _ = _ + y
                    let new_val = self.evaluate_binary_op(*op, &current_val, &right_val)?;

                    let old_val = if let Some(return_name) = &return_name {
                        self.named_returns.get(return_name).cloned()
                    } else {
                        self.return_value.clone()
                    };

                    if self.debug {
                        let old_str = old_val
                            .as_ref()
                            .map(|v| self.format_debug_value(v))
                            .unwrap_or_else(|| "undefined".to_string());
                        let new_str = self.format_debug_value(&new_val);
                        let indent = " ".repeat(self.debug_indent);
                        eprintln!("DEBUG: {}{} {}: {} → {}", indent, name, self.format_op(*op), old_str, new_str);
                    }

                    // Set the new return value
                    if let Some(return_name) = return_name {
                        // Named return: only set in named_returns
                        self.named_returns.insert(return_name, new_val);
                    } else {
                        // Unnamed return: set in return_value
                        self.return_value = Some(new_val);
                    }
                } else {
                    // Regular variable assignment
                    let current_val = self.variables.get(name)
                        .ok_or_else(|| format!("Undefined variable: {}", name))?
                        .clone();

                    // Evaluate the right-hand side expression
                    let right_val = self.evaluate_expression(value)?;

                    // Perform the operation: x += y means x = x + y
                    let new_val = self.evaluate_binary_op(*op, &current_val, &right_val)?;

                    let old_val = self.variables.get(name).cloned();
                    if self.debug {
                        let old_str = old_val
                            .as_ref()
                            .map(|v| self.format_debug_value(v))
                            .unwrap_or_else(|| "undefined".to_string());
                        let new_str = self.format_debug_value(&new_val);
                        let indent = " ".repeat(self.debug_indent);
                        eprintln!("DEBUG: {}{} {}: {} → {}", indent, name, self.format_op(*op), old_str, new_str);
                    }
                    self.variables.insert(name.clone(), new_val);
                }
                Ok(())
            }
            Stmt::Return { name, value } => {
                let val = self.evaluate_expression(value)?;
                // Set as default return value
                self.return_value = Some(val.clone());
                // If it's a named return, also store it by name
                if let Some(return_name) = name {
                    self.named_returns.insert(return_name.clone(), val);
                }
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                self.functions.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(())
            }
            Stmt::Expr(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
        }
    }

    /// Evaluate an expression to a value.
    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),

            Expr::String(s) => Ok(Value::String(s.clone())),

            Expr::Identifier(name) => {
                if name == "input" {
                    self.get_input_value()
                } else if name.starts_with('_') && name.len() > 1 {
                    // Named return access: _name
                    let return_name = &name[1..];
                    match self.named_returns.get(return_name) {
                        Some(val) => Ok(val.clone()),
                        None => Err(format!("Undefined named return: _{}", return_name))
                    }
                } else {
                    // Check if variable exists and log for debugging
                    match self.variables.get(name) {
                        Some(val) => {
                            // Debug: check if we're getting an Array1D when we expect a Number
                            Ok(val.clone())
                        }
                        None => Err(format!("Undefined variable: {}", name))
                    }
                }
            }

            Expr::Input => self.get_input_value(),

            Expr::ReturnValue => {
                self.return_value
                    .clone()
                    .ok_or_else(|| "No return value set".to_string())
            }

            Expr::Array(elements) => {
                let values: Result<Vec<Value>, String> = elements
                    .iter()
                    .map(|e| self.evaluate_expression(e))
                    .collect();
                Ok(Value::Array1D(values?))
            }

            Expr::Range { start, end } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;

                let start_num = match start_val {
                    Value::Number(n) => n,
                    _ => return Err("Range start must be a number".to_string()),
                };

                let end_num = match end_val {
                    Value::Number(n) => n,
                    _ => return Err("Range end must be a number".to_string()),
                };

                // Generate inclusive range [start..end]
                let mut range = Vec::new();
                if start_num <= end_num {
                    for i in start_num..=end_num {
                        range.push(Value::Number(i));
                    }
                } else {
                    // Reverse range
                    for i in (end_num..=start_num).rev() {
                        range.push(Value::Number(i));
                    }
                }

                Ok(Value::Array1D(range))
            }

            Expr::Unary { op, expr } => {
                let val = self.evaluate_expression(expr)?;
                match op {
                    UnaryOp::Tilde => {
                        // Convert string to number
                        // Arrays of characters should behave exactly like strings
                        // Booleans: true -> 1, false -> 0
                        match val {
                            Value::String(s) => {
                                s.parse::<i64>()
                                    .map(Value::Number)
                                    .map_err(|_| format!("Cannot convert '{}' to number", s))
                            }
                            Value::Array1D(arr) => {
                                // Convert array of character strings to a single string
                                let mut s = String::new();
                                for item in arr {
                                    match item {
                                        Value::String(ch) => s.push_str(&ch),
                                        _ => return Err("Cannot convert non-string array element to number".to_string()),
                                    }
                                }
                                s.parse::<i64>()
                                    .map(Value::Number)
                                    .map_err(|_| format!("Cannot convert '{}' to number", s))
                            }
                            Value::Number(n) => Ok(Value::Number(n)),
                            Value::Boolean(b) => Ok(Value::Number(if b { 1 } else { 0 })),
                            _ => Err("Cannot convert to number".to_string()),
                        }
                    }
                    UnaryOp::Bang => {
                        // Logical NOT: !value
                        Ok(Value::Boolean(!self.is_truthy(&val)))
                    }
                }
            }

            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(*op, &left_val, &right_val)
            }

            Expr::Pipe { left, right } => {
                // Pipe: f |> g means g(f(...))
                // For now, we'll evaluate left, then right with left as argument
                // This is a simplified version - full implementation would need
                // to handle function composition properly
                let left_val = self.evaluate_expression(left)?;
                // Store left value in a temporary variable for the right expression
                let temp_name = "__pipe_temp__".to_string();
                let old_val = self.variables.insert(temp_name.clone(), left_val);
                let result = self.evaluate_expression(right)?;
                // Restore or remove temp variable
                if let Some(old) = old_val {
                    self.variables.insert(temp_name, old);
                } else {
                    self.variables.remove(&temp_name);
                }
                Ok(result)
            }

            Expr::Call { callee, args } => {
                // Callee should be an identifier (function name)
                if let Expr::Identifier(name) = callee.as_ref() {
                    self.call_function(name, args)
                } else {
                    Err("Function call must use identifier".to_string())
                }
            }

            Expr::Index { array, index } => {
                let array_val = self.evaluate_expression(array)?;
                self.evaluate_index(&array_val, index)
            }

            Expr::Builtin { name, args } => {
                self.call_builtin(name, args)
            }

            Expr::MethodCall { object, method, args } => {
                let obj_val = self.evaluate_expression(object)?;
                self.call_method(&obj_val, method, args)
            }

            Expr::Block(statements) => {
                // Execute statements in block, return last return value
                let old_return = self.return_value.clone();
                let old_named_returns = self.named_returns.clone();
                self.return_value = None;
                self.named_returns.clear();

                for stmt in statements {
                    self.execute_statement(stmt)?;
                }

                let result = self.return_value.clone();
                self.return_value = old_return;
                self.named_returns = old_named_returns;
                Ok(result.unwrap_or(Value::Array1D(Vec::new())))
            }
        }
    }

    /// Get the input value (2D character array).
    fn get_input_value(&self) -> Result<Value, String> {
        if let Some(input) = &self.input {
            let array_2d: Vec<Vec<Value>> = input
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|&ch| Value::String(ch.to_string()))
                        .collect()
                })
                .collect();
            Ok(Value::Array2D(array_2d))
        } else {
            Ok(Value::Array2D(Vec::new()))
        }
    }

    /// Evaluate a binary operation.
    fn evaluate_binary_op(
        &mut self,
        op: BinaryOp,
        left: &Value,
        right: &Value,
    ) -> Result<Value, String> {
        match op {
            BinaryOp::Plus => {
                // Addition or array concatenation
                // Support boolean-to-number coercion for arithmetic
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Number(a), Value::Boolean(b)) => Ok(Value::Number(a + if *b { 1 } else { 0 })),
                    (Value::Boolean(a), Value::Number(b)) => Ok(Value::Number(if *a { 1 } else { 0 } + b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    (Value::Array1D(a), Value::Array1D(b)) => {
                        let mut result = a.clone();
                        result.extend_from_slice(b);
                        Ok(Value::Array1D(result))
                    }
                    _ => Err("Invalid operands for +".to_string()),
                }
            }
            BinaryOp::Minus => {
                // Support boolean-to-number coercion for arithmetic
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    (Value::Number(a), Value::Boolean(b)) => Ok(Value::Number(a - if *b { 1 } else { 0 })),
                    (Value::Boolean(a), Value::Number(b)) => Ok(Value::Number(if *a { 1 } else { 0 } - b)),
                    _ => {
                        // Better error message for debugging
                        Err(format!(
                            "Invalid operands for -: left is {:?}, right is {:?}",
                            left, right
                        ))
                    }
                }
            }
            BinaryOp::Star => {
                // Support boolean-to-number coercion for arithmetic
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    (Value::Number(a), Value::Boolean(b)) => Ok(Value::Number(a * if *b { 1 } else { 0 })),
                    (Value::Boolean(a), Value::Number(b)) => Ok(Value::Number(if *a { 1 } else { 0 } * b)),
                    _ => Err("Invalid operands for *".to_string()),
                }
            }
            BinaryOp::Slash => {
                // Support boolean-to-number coercion for arithmetic
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        if *b == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }
                    (Value::Number(a), Value::Boolean(b)) => {
                        let b_val = if *b { 1 } else { 0 };
                        if b_val == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(a / b_val))
                        }
                    }
                    (Value::Boolean(a), Value::Number(b)) => {
                        if *b == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(if *a { 1 } else { 0 } / b))
                        }
                    }
                    _ => Err("Invalid operands for /".to_string()),
                }
            }
            BinaryOp::Percent => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        if *b == 0 {
                            Err("Modulo by zero".to_string())
                        } else {
                            Ok(Value::Number(a % b))
                        }
                    }
                    _ => Err("Invalid operands for %".to_string()),
                }
            }
            BinaryOp::Less => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(*a < *b))
                    }
                    _ => Err("Invalid operands for <".to_string()),
                }
            }
            BinaryOp::Greater => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(*a > *b))
                    }
                    _ => Err("Invalid operands for >".to_string()),
                }
            }
            BinaryOp::LessEqual => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(*a <= *b))
                    }
                    _ => Err("Invalid operands for <=".to_string()),
                }
            }
            BinaryOp::GreaterEqual => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(*a >= *b))
                    }
                    _ => Err("Invalid operands for >=".to_string()),
                }
            }
            BinaryOp::EqualEqual => {
                Ok(Value::Boolean(left == right))
            }
            BinaryOp::And => {
                // JavaScript-style: return last truthy or first falsy
                let left_truthy = self.is_truthy(left);
                if !left_truthy {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }
            BinaryOp::Or => {
                // JavaScript-style: return first truthy or last falsy
                let left_truthy = self.is_truthy(left);
                if left_truthy {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }
        }
    }

    /// Evaluate array indexing/slicing.
    fn evaluate_index(&mut self, array: &Value, indices: &[IndexExpr]) -> Result<Value, String> {
        let mut current = array.clone();
        let mut i = 0;

        while i < indices.len() {
            let index_expr = &indices[i];

            // Special case: column access for 2D arrays: input[.., n] or input[start..end, n]
            // Check if current is 2D, current index is range, and next is single
            let is_column_access = matches!(current, Value::Array2D(_))
                && matches!(index_expr, IndexExpr::Range { .. })
                && i + 1 < indices.len()
                && matches!(indices[i + 1], IndexExpr::Single(_));

            if is_column_access {
                // Column access: input[.., n] - extract column n from rows
                let range_expr = index_expr;
                let col_expr = &indices[i + 1];

                // Get the column index
                let col_idx_val = if let IndexExpr::Single(expr) = col_expr {
                    self.evaluate_expression(expr)?
                } else {
                    return Err("Column access requires single index".to_string());
                };
                let col_idx = self.value_to_index(&col_idx_val)?;

                // Get the row range
                let start_idx = if let IndexExpr::Range { start, .. } = range_expr {
                    start
                        .as_ref()
                        .map(|e| self.evaluate_expression(e))
                        .transpose()?
                        .map(|v| self.value_to_index(&v))
                        .transpose()?
                        .unwrap_or(0)
                } else {
                    return Err("Column access requires range for rows".to_string());
                };

                let end_idx = if let IndexExpr::Range { end, .. } = range_expr {
                    end.as_ref()
                        .map(|e| self.evaluate_expression(e))
                        .transpose()?
                        .map(|v| self.value_to_index(&v))
                        .transpose()?
                } else {
                    None
                };

                // Extract column from 2D array
                if let Value::Array2D(arr) = &current {
                    let end_row = end_idx.unwrap_or(arr.len());
                    let end_row = end_row.min(arr.len());
                    let start_row = start_idx.min(arr.len());

                    let column: Vec<Value> = arr[start_row..end_row]
                        .iter()
                        .filter_map(|row| row.get(col_idx).cloned())
                        .collect();

                    current = Value::Array1D(column);
                    // Skip both the range and the column index
                    i += 2;
                    continue;
                } else {
                    return Err("Column access only works on 2D arrays".to_string());
                }
            }

            current = match index_expr {
                IndexExpr::Single(expr) => {
                    let idx_val = self.evaluate_expression(expr)?;
                    let idx = self.value_to_index(&idx_val)?;
                    self.index_value(&current, idx)?
                }
                IndexExpr::Range { start, end } => {
                    let start_idx = start
                        .as_ref()
                        .map(|e| self.evaluate_expression(e))
                        .transpose()?
                        .map(|v| self.value_to_index(&v))
                        .transpose()?
                        .unwrap_or(0);
                    let end_idx = end
                        .as_ref()
                        .map(|e| self.evaluate_expression(e))
                        .transpose()?
                        .map(|v| self.value_to_index(&v))
                        .transpose()?;

                    self.slice_value(&current, start_idx, end_idx)?
                }
            };

            i += 1;
        }

        Ok(current)
    }

    /// Index a value (single index).
    fn index_value(&self, value: &Value, index: usize) -> Result<Value, String> {
        match value {
            Value::Array1D(arr) => {
                if index >= arr.len() {
                    return Err(format!("Index {} out of bounds (array length: {})", index, arr.len()));
                }
                arr.get(index)
                    .cloned()
                    .ok_or_else(|| format!("Index {} out of bounds", index))
            }
            Value::Array2D(arr) => {
                arr.get(index)
                    .map(|row| Value::Array1D(row.clone()))
                    .ok_or_else(|| format!("Index {} out of bounds", index))
            }
            Value::String(s) => {
                s.chars()
                    .nth(index)
                    .map(|ch| Value::String(ch.to_string()))
                    .ok_or_else(|| format!("Index {} out of bounds", index))
            }
            _ => Err(format!("Cannot index non-array value: {:?}", value)),
        }
    }

    /// Slice a value (range).
    fn slice_value(
        &self,
        value: &Value,
        start: usize,
        end: Option<usize>,
    ) -> Result<Value, String> {
        match value {
            Value::Array1D(arr) => {
                let end_idx = end.unwrap_or(arr.len());
                let end_idx = end_idx.min(arr.len());
                let start_idx = start.min(arr.len());
                Ok(Value::Array1D(arr[start_idx..end_idx].to_vec()))
            }
            Value::String(s) => {
                let end_idx = end.unwrap_or(s.chars().count());
                let chars: Vec<char> = s.chars().collect();
                let end_idx = end_idx.min(chars.len());
                let start_idx = start.min(chars.len());
                let sliced: String = chars[start_idx..end_idx].iter().collect();
                Ok(Value::String(sliced))
            }
            Value::Array2D(arr) => {
                // For 2D arrays, slicing works on rows
                let end_idx = end.unwrap_or(arr.len());
                let end_idx = end_idx.min(arr.len());
                let start_idx = start.min(arr.len());
                Ok(Value::Array2D(arr[start_idx..end_idx].to_vec()))
            }
            _ => Err("Cannot slice non-array value".to_string()),
        }
    }

    /// Convert a value to an index (usize).
    fn value_to_index(&self, value: &Value) -> Result<usize, String> {
        match value {
            Value::Number(n) => {
                if *n >= 0 {
                    Ok(*n as usize)
                } else {
                    Err("Array index must be non-negative integer".to_string())
                }
            }
            _ => Err("Array index must be a number".to_string()),
        }
    }

    /// Call a user-defined function.
    fn call_function(&mut self, name: &str, args: &[Expr]) -> Result<Value, String> {
        let func = self
            .functions
            .get(name)
            .ok_or_else(|| format!("Undefined function: {}", name))?
            .clone();

        if args.len() != func.params.len() {
            return Err(format!(
                "Function {} expects {} arguments, got {}",
                name,
                func.params.len(),
                args.len()
            ));
        }

        // Evaluate arguments
        let arg_values: Result<Vec<Value>, String> = args
            .iter()
            .map(|arg| self.evaluate_expression(arg))
            .collect();
        let arg_values = arg_values?;

        // Save current variable state
        let mut saved_vars = HashMap::new();
        for (param, arg_val) in func.params.iter().zip(arg_values.iter()) {
            saved_vars.insert(param.clone(), self.variables.get(param).cloned());
            self.variables.insert(param.clone(), arg_val.clone());
        }

        // Save return value
        let old_return = self.return_value.clone();
        let old_named_returns = self.named_returns.clone();
        self.return_value = None;
        self.named_returns.clear();

        // Execute function body
        let result = self.evaluate_expression(&func.body);

        // Restore variables
        for (param, old_val) in saved_vars {
            if let Some(val) = old_val {
                self.variables.insert(param, val);
            } else {
                self.variables.remove(&param);
            }
        }

        // Get return value
        let return_val = self.return_value.clone();
        self.return_value = old_return;
        self.named_returns = old_named_returns;

        // If function didn't set return value, use the result of evaluating the body
        // (for expression bodies like: add(x, y) = x + y)
        if return_val.is_none() {
            result
        } else {
            result?; // Check for errors
            return_val.ok_or_else(|| format!("Function {} did not return a value", name))
        }
    }

    /// Call a built-in function.
    fn call_builtin(&mut self, name: &str, args: &[Expr]) -> Result<Value, String> {
        match name {
            "if" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err("if requires 2 or 3 arguments: condition, trueBlock, [falseBlock]".to_string());
                }
                let condition = self.evaluate_expression(&args[0])?;
                let is_true = self.is_truthy(&condition);
                if self.debug {
                    let cond_str = self.format_expr(&args[0]);
                    let indent = " ".repeat(self.debug_indent);
                    eprintln!("DEBUG: {}if {}: {}", indent, cond_str, is_true);
                }
                // Indent for statements inside if block
                self.debug_indent += 2;
                let result = if is_true {
                    self.evaluate_expression(&args[1])
                } else if args.len() == 3 {
                    self.evaluate_expression(&args[2])
                } else {
                    // No else block - return empty array when condition is false
                    Ok(Value::Array1D(vec![]))
                };
                self.debug_indent -= 2;
                result
            }
            "for" => {
                if args.len() < 3 || args.len() > 4 {
                    return Err("for requires 3 or 4 arguments: variable, array, block, [initialValue]".to_string());
                }
                // args[0] should be Identifier(var_name), args[1] is array, args[2] is block, args[3] is optional initial
                let var_name = if let Expr::Identifier(name) = &args[0] {
                    name.clone()
                } else {
                    return Err("First argument to for must be variable name".to_string());
                };

                let array = self.evaluate_expression(&args[1])?;

                // Get array elements
                let elements = match array {
                    Value::Array1D(elements) => elements,
                    Value::Array2D(_) => return Err("for loop requires 1D array".to_string()),
                    _ => return Err("for loop requires array".to_string()),
                };

                // Handle initial value if provided
                let old_return = self.return_value.clone();
                let old_named_returns = self.named_returns.clone();
                let has_initial = args.len() == 4;
                let initial_value = if has_initial {
                    let initial = self.evaluate_expression(&args[3])?;
                    self.return_value = Some(initial.clone());
                    Some(initial)
                } else {
                    // No initial value - don't set return value
                    self.return_value = None;
                    None
                };
                // Clear named returns at start of loop
                self.named_returns.clear();

                // Iterate
                for element in elements {
                    if self.debug {
                        let elem_str = self.format_debug_value(&element);
                        let indent = " ".repeat(self.debug_indent);
                        eprintln!("DEBUG: {}for {}: {}", indent, var_name, elem_str);
                    }
                    // Set loop variable
                    let old_var = self.variables.insert(var_name.clone(), element);

                    // Indent for statements inside for loop
                    self.debug_indent += 2;

                    // Execute block - this should update the return value if initial was provided
                    // Don't save/restore return value - we want it to persist
                    if let Expr::Block(statements) = &args[2] {
                        // Execute statements directly without saving return value
                        for stmt in statements {
                            self.execute_statement(stmt)?;
                        }
                    } else {
                        // If it's not a block, evaluate it normally
                        self.evaluate_expression(&args[2])?;
                    }

                    // Restore indentation
                    self.debug_indent -= 2;

                    // Restore or update variable
                    if let Some(old) = old_var {
                        self.variables.insert(var_name.clone(), old);
                    } else {
                        self.variables.remove(&var_name);
                    }
                }

                // Get final return value
                let result = if has_initial {
                    // If initial was provided, return the accumulated value
                    let result = self.return_value.clone();
                    self.return_value = old_return;
                    self.named_returns = old_named_returns;
                    Ok(result.unwrap_or_else(|| initial_value.unwrap_or(Value::Array1D(vec![]))))
                } else {
                    // If no initial was provided, return empty array (loop doesn't return anything)
                    self.return_value = old_return;
                    self.named_returns = old_named_returns;
                    Ok(Value::Array1D(vec![]))
                };
                result
            }
            "len" => {
                if args.len() != 1 {
                    return Err("len requires 1 argument".to_string());
                }
                let value = self.evaluate_expression(&args[0])?;
                match value {
                    Value::Array1D(arr) => Ok(Value::Number(arr.len() as i64)),
                    Value::Array2D(arr) => {
                        if arr.is_empty() {
                            Ok(Value::Array1D(vec![Value::Number(0), Value::Number(0)]))
                        } else {
                            Ok(Value::Array1D(vec![
                                Value::Number(arr.len() as i64),
                                Value::Number(arr[0].len() as i64),
                            ]))
                        }
                    }
                    Value::String(s) => Ok(Value::Number(s.chars().count() as i64)),
                    _ => Err("len requires array or string".to_string()),
                }
            }
            "max" => {
                if args.len() != 2 {
                    return Err("max requires 2 arguments".to_string());
                }
                let val1 = self.evaluate_expression(&args[0])?;
                let val2 = self.evaluate_expression(&args[1])?;
                match (val1, val2) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.max(b))),
                    _ => Err("max requires 2 numbers".to_string()),
                }
            }
            "min" => {
                if args.len() != 2 {
                    return Err("min requires 2 arguments".to_string());
                }
                let val1 = self.evaluate_expression(&args[0])?;
                let val2 = self.evaluate_expression(&args[1])?;
                match (val1, val2) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.min(b))),
                    _ => Err("min requires 2 numbers".to_string()),
                }
            }
            "floor" => {
                if args.len() != 1 {
                    return Err("floor requires 1 argument".to_string());
                }
                let value = self.evaluate_expression(&args[0])?;
                match value {
                    Value::Number(n) => Ok(Value::Number(n)), // For integers, floor is a no-op
                    _ => Err("floor requires a number".to_string()),
                }
            }
            "ceil" => {
                if args.len() != 1 {
                    return Err("ceil requires 1 argument".to_string());
                }
                let value = self.evaluate_expression(&args[0])?;
                match value {
                    Value::Number(n) => Ok(Value::Number(n)), // For integers, ceil is a no-op
                    _ => Err("ceil requires a number".to_string()),
                }
            }
            _ => Err(format!("Unknown builtin function: {}", name)),
        }
    }

    /// Call a method on a value.
    fn call_method(&mut self, object: &Value, method: &str, args: &[Expr]) -> Result<Value, String> {
        match method {
            "rows" => {
                if !args.is_empty() {
                    return Err("rows() method takes no arguments".to_string());
                }
                match object {
                    Value::Array2D(arr) => {
                        // Convert 2D array to 1D array of 1D arrays (rows)
                        let rows: Vec<Value> = arr
                            .iter()
                            .map(|row| Value::Array1D(row.clone()))
                            .collect();
                        Ok(Value::Array1D(rows))
                    }
                    _ => Err("rows() method only works on 2D arrays".to_string()),
                }
            }
            _ => Err(format!("Unknown method: {}", method)),
        }
    }

    /// Check if a value is truthy (non-zero number, true boolean, or non-empty array/string).
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Number(n) => *n != 0,
            Value::Boolean(b) => *b,
            Value::String(s) => !s.is_empty(),
            Value::Array1D(arr) => !arr.is_empty(),
            Value::Array2D(arr) => !arr.is_empty(),
        }
    }

    /// Format an expression as a string for debug output.
    fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => format!("{}", n),
            Expr::Boolean(b) => format!("{}", b),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Identifier(name) => name.clone(),
            Expr::Input => "input".to_string(),
            Expr::ReturnValue => "_".to_string(),
            Expr::Array(elements) => {
                let items: Vec<String> = elements.iter().map(|e| self.format_expr(e)).collect();
                format!("[{}]", items.join(", "))
            }
            Expr::Range { start, end } => {
                format!("[{}..{}]", self.format_expr(start), self.format_expr(end))
            }
            Expr::Unary { op, expr } => {
                let op_str = match op {
                    UnaryOp::Tilde => "~",
                    UnaryOp::Bang => "!",
                };
                format!("{}{}", op_str, self.format_expr(expr))
            }
            Expr::Binary { left, op, right } => {
                let op_str = match op {
                    BinaryOp::Plus => "+",
                    BinaryOp::Minus => "-",
                    BinaryOp::Star => "*",
                    BinaryOp::Slash => "/",
                    BinaryOp::Percent => "%",
                    BinaryOp::Less => "<",
                    BinaryOp::Greater => ">",
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::GreaterEqual => ">=",
                    BinaryOp::EqualEqual => "==",
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                };
                // Add parentheses for clarity (might want to be smarter about this later)
                format!("({} {} {})", self.format_expr(left), op_str, self.format_expr(right))
            }
            Expr::Pipe { left, right } => {
                format!("{} |> {}", self.format_expr(left), self.format_expr(right))
            }
            Expr::Call { callee, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                format!("{}({})", self.format_expr(callee), args_str.join(", "))
            }
            Expr::Index { array, index } => {
                let mut result = self.format_expr(array);
                for idx in index {
                    match idx {
                        IndexExpr::Single(expr) => {
                            result = format!("{}[{}]", result, self.format_expr(expr));
                        }
                        IndexExpr::Range { start, end } => {
                            let start_str = start.as_ref().map(|e| self.format_expr(e)).unwrap_or_else(|| "".to_string());
                            let end_str = end.as_ref().map(|e| self.format_expr(e)).unwrap_or_else(|| "".to_string());
                            result = format!("{}[{}..{}]", result, start_str, end_str);
                        }
                    }
                }
                result
            }
            Expr::Builtin { name, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                format!("{}({})", name, args_str.join(", "))
            }
            Expr::MethodCall { object, method, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                format!("{}.{}({})", self.format_expr(object), method, args_str.join(", "))
            }
            Expr::Block(_) => "{ ... }".to_string(),
        }
    }

    /// Format an operator for debug output.
    fn format_op(&self, op: BinaryOp) -> &str {
        match op {
            BinaryOp::Plus => "+=",
            BinaryOp::Minus => "-=",
            BinaryOp::Star => "*=",
            BinaryOp::Slash => "/=",
            BinaryOp::Percent => "%=",
            _ => "?=",
        }
    }

    /// Format a value for debug output.
    /// Arrays of single-character strings are concatenated into a string.
    fn format_debug_value(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => format!("{}", n),
            Value::Boolean(b) => format!("{}", b),
            Value::String(s) => format!("\"{}\"", s),
            Value::Array1D(arr) => {
                // Check if it's an array of single-character strings
                let mut is_char_array = true;
                let mut chars = String::new();
                for item in arr {
                    match item {
                        Value::String(s) if s.chars().count() == 1 => {
                            chars.push_str(s);
                        }
                        _ => {
                            is_char_array = false;
                            break;
                        }
                    }
                }
                if is_char_array && !arr.is_empty() {
                    format!("\"{}\"", chars)
                } else {
                    // Format as array
                    let items: Vec<String> = arr.iter().map(|v| self.format_debug_value(v)).collect();
                    format!("[{}]", items.join(", "))
                }
            }
            Value::Array2D(arr) => {
                let rows: Vec<String> = arr
                    .iter()
                    .map(|row| {
                        let items: Vec<String> = row.iter().map(|v| self.format_debug_value(v)).collect();
                        format!("[{}]", items.join(", "))
                    })
                    .collect();
                format!("[{}]", rows.join(", "))
            }
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn interpret_code(code: &str) -> Result<Value, String> {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens, code.to_string());
        let program = parser.parse()?;
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program)
    }

    #[test]
    fn test_number() {
        let result = interpret_code("5").unwrap();
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn test_string() {
        let result = interpret_code(r#""hello""#).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_assignment() {
        let result = interpret_code("x = 5").unwrap();
        // Should return empty array (no return value)
        assert_eq!(result, Value::Array1D(Vec::new()));
    }

    #[test]
    fn test_variable_access() {
        let result = interpret_code("x = 5\nx").unwrap();
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn test_arithmetic() {
        let result = interpret_code("1 + 2 * 3").unwrap();
        assert_eq!(result, Value::Number(7)); // 1 + (2 * 3) = 7
    }

    #[test]
    fn test_integer_division() {
        let result = interpret_code("10 / 3").unwrap();
        assert_eq!(result, Value::Number(3)); // Integer division

        let result = interpret_code("7 / 2").unwrap();
        assert_eq!(result, Value::Number(3)); // Integer division
    }

    #[test]
    fn test_comparison() {
        let result = interpret_code("5 == 5").unwrap();
        assert_eq!(result, Value::Boolean(true));

        let result = interpret_code("5 == 3").unwrap();
        assert_eq!(result, Value::Boolean(false));

        let result = interpret_code("5 < 10").unwrap();
        assert_eq!(result, Value::Boolean(true));

        let result = interpret_code("5 > 10").unwrap();
        assert_eq!(result, Value::Boolean(false));

        let result = interpret_code("5 <= 5").unwrap();
        assert_eq!(result, Value::Boolean(true));

        let result = interpret_code("5 >= 10").unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_array_literal() {
        let result = interpret_code("[1, 2, 3]").unwrap();
        if let Value::Array1D(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Value::Number(1));
            assert_eq!(arr[1], Value::Number(2));
            assert_eq!(arr[2], Value::Number(3));
        } else {
            panic!("Expected Array1D");
        }
    }

    #[test]
    fn test_range_literal() {
        let result = interpret_code("[0..5]").unwrap();
        if let Value::Array1D(arr) = result {
            assert_eq!(arr.len(), 6);
            assert_eq!(arr, vec![
                Value::Number(0),
                Value::Number(1),
                Value::Number(2),
                Value::Number(3),
                Value::Number(4),
                Value::Number(5),
            ]);
        } else {
            panic!("Expected Array1D");
        }

        // Reverse range
        let result = interpret_code("[5..0]").unwrap();
        if let Value::Array1D(arr) = result {
            assert_eq!(arr.len(), 6);
            assert_eq!(arr, vec![
                Value::Number(5),
                Value::Number(4),
                Value::Number(3),
                Value::Number(2),
                Value::Number(1),
                Value::Number(0),
            ]);
        } else {
            panic!("Expected Array1D");
        }

        // Single element range
        let result = interpret_code("[3..3]").unwrap();
        if let Value::Array1D(arr) = result {
            assert_eq!(arr.len(), 1);
            assert_eq!(arr[0], Value::Number(3));
        } else {
            panic!("Expected Array1D");
        }
    }

    #[test]
    fn test_array_concatenation() {
        let result = interpret_code("[1, 2] + [3, 4]").unwrap();
        if let Value::Array1D(arr) = result {
            assert_eq!(arr.len(), 4);
            assert_eq!(arr[0], Value::Number(1));
            assert_eq!(arr[3], Value::Number(4));
        } else {
            panic!("Expected Array1D");
        }
    }

    #[test]
    fn test_array_indexing() {
        let result = interpret_code("arr = [10, 20, 30]\narr[1]").unwrap();
        assert_eq!(result, Value::Number(20));
    }

    #[test]
    fn test_array_slicing() {
        let result = interpret_code("arr = [1, 2, 3, 4, 5]\narr[1..4]").unwrap();
        if let Value::Array1D(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Value::Number(2));
            assert_eq!(arr[2], Value::Number(4));
        } else {
            panic!("Expected Array1D");
        }
    }

    #[test]
    fn test_function_definition_and_call() {
        let code = r#"
add(a, b) = { _ = a + b }
result = add(5, 10)
result
"#;
        let result = interpret_code(code).unwrap();
        assert_eq!(result, Value::Number(15));
    }

    #[test]
    fn test_return_value() {
        let code = r#"
_ = 42
_
"#;
        let result = interpret_code(code).unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    fn test_conditional() {
        let code = r#"
x = 5
if(x == 5, 10, 20)
"#;
        let result = interpret_code(code).unwrap();
        assert_eq!(result, Value::Number(10));

        let code = r#"
x = 3
if(x == 5, 10, 20)
"#;
        let result = interpret_code(code).unwrap();
        assert_eq!(result, Value::Number(20));
    }

    #[test]
    fn test_for_loop() {
        let code = r#"
arr = [1, 2, 3]
for(n of arr, { _ = _ + n }, 0)
"#;
        let result = interpret_code(code).unwrap();
        assert_eq!(result, Value::Number(6)); // 0 + 1 + 2 + 3 = 6
    }

    #[test]
    fn test_for_loop_no_initial() {
        let code = r#"
x = 0
for(n of [1, 2, 3], { x = x + n })
x
"#;
        let result = interpret_code(code).unwrap();
        assert_eq!(result, Value::Number(6)); // Side effects: x = 0 + 1 + 2 + 3 = 6
    }

    #[test]
    fn test_builtin_len() {
        let result = interpret_code("len([1, 2, 3])").unwrap();
        assert_eq!(result, Value::Number(3));

        let result = interpret_code(r#"len("hello")"#).unwrap();
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn test_type_conversion() {
        let result = interpret_code(r#"~"123""#).unwrap();
        assert_eq!(result, Value::Number(123));

        // Test that arrays of characters behave like strings
        let mut interpreter = Interpreter::new();
        interpreter.set_input("L50\nR25");
        let code = r#"
line = input[0]
~line[1..]
"#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens, code.to_string());
        let program = parser.parse().unwrap();
        let result = interpreter.interpret(&program).unwrap();
        assert_eq!(result, Value::Number(50));

        // Test boolean conversion: true -> 1, false -> 0
        let result = interpret_code("~true").unwrap();
        assert_eq!(result, Value::Number(1));

        let result = interpret_code("~false").unwrap();
        assert_eq!(result, Value::Number(0));

        // Test user's example: a = if(a < 20, 1, 0) can be replaced with a = ~(a < 20)
        let result = interpret_code("a = 12\na = ~(a < 20)\na").unwrap();
        assert_eq!(result, Value::Number(1)); // 12 < 20 is true, so ~true = 1

        let result = interpret_code("a = 25\na = ~(a < 20)\na").unwrap();
        assert_eq!(result, Value::Number(0)); // 25 < 20 is false, so ~false = 0
    }

    #[test]
    fn test_string_concatenation() {
        let result = interpret_code(r#""hello" + " " + "world""#).unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_input_access() {
        let mut lexer = Lexer::new("input[0]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens, "input[0]".to_string());
        let program = parser.parse().unwrap();

        let mut interpreter = Interpreter::new();
        interpreter.set_input("abc\ndef");

        let result = interpreter.interpret(&program).unwrap();
        if let Value::Array1D(arr) = result {
            assert_eq!(arr.len(), 3); // "abc" -> ['a', 'b', 'c']
        } else {
            panic!("Expected Array1D");
        }
    }

    #[test]
    fn test_assignment_operators() {
        // Test +=
        let result = interpret_code("x = 5\nx += 3\nx").unwrap();
        assert_eq!(result, Value::Number(8));

        // Test -=
        let result = interpret_code("x = 10\nx -= 4\nx").unwrap();
        assert_eq!(result, Value::Number(6));

        // Test *=
        let result = interpret_code("x = 5\nx *= 3\nx").unwrap();
        assert_eq!(result, Value::Number(15));

        // Test /=
        let result = interpret_code("x = 20\nx /= 4\nx").unwrap();
        assert_eq!(result, Value::Number(5));

        // Test %=
        let result = interpret_code("x = 17\nx %= 5\nx").unwrap();
        assert_eq!(result, Value::Number(2));
    }
}
