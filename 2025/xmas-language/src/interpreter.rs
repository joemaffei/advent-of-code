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
}

impl Interpreter {
    /// Create a new interpreter.
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            input: None,
            return_value: None,
        }
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
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Stmt::Return { value } => {
                let val = self.evaluate_expression(value)?;
                self.return_value = Some(val);
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
                } else {
                    self.variables
                        .get(name)
                        .cloned()
                        .ok_or_else(|| format!("Undefined variable: {}", name))
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
                        match val {
                            Value::String(s) => {
                                s.parse::<i64>()
                                    .map(Value::Number)
                                    .map_err(|_| format!("Cannot convert '{}' to number", s))
                            }
                            Value::Number(n) => Ok(Value::Number(n)),
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

            Expr::Block(statements) => {
                // Execute statements in block, return last return value
                let old_return = self.return_value.clone();
                self.return_value = None;

                for stmt in statements {
                    self.execute_statement(stmt)?;
                }

                let result = self.return_value.clone();
                self.return_value = old_return;
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
                    _ => Err("Invalid operands for -".to_string()),
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
            _ => Err("Cannot index non-array value".to_string()),
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
        self.return_value = None;

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
                if args.len() != 3 {
                    return Err("if requires 3 arguments: condition, trueBlock, falseBlock".to_string());
                }
                let condition = self.evaluate_expression(&args[0])?;
                let is_true = self.is_truthy(&condition);
                if is_true {
                    self.evaluate_expression(&args[1])
                } else {
                    self.evaluate_expression(&args[2])
                }
            }
            "for" => {
                if args.len() != 4 {
                    return Err("for requires 4 arguments: variable, array, block, initialValue".to_string());
                }
                // args[0] should be Identifier(var_name), args[1] is array, args[2] is block, args[3] is initial
                let var_name = if let Expr::Identifier(name) = &args[0] {
                    name.clone()
                } else {
                    return Err("First argument to for must be variable name".to_string());
                };

                let array = self.evaluate_expression(&args[1])?;
                let initial = self.evaluate_expression(&args[3])?;

                // Get array elements
                let elements = match array {
                    Value::Array1D(elements) => elements,
                    Value::Array2D(_) => return Err("for loop requires 1D array".to_string()),
                    _ => return Err("for loop requires array".to_string()),
                };

                // Set initial return value
                let old_return = self.return_value.clone();
                self.return_value = Some(initial.clone());

                // Iterate
                for element in elements {
                    // Set loop variable
                    let old_var = self.variables.insert(var_name.clone(), element);

                    // Execute block - this should update the return value
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

                    // Restore or update variable
                    if let Some(old) = old_var {
                        self.variables.insert(var_name.clone(), old);
                    } else {
                        self.variables.remove(&var_name);
                    }
                }

                // Get final return value (should be set by the block)
                let result = self.return_value.clone();
                self.return_value = old_return;
                Ok(result.unwrap_or(initial))
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
            _ => Err(format!("Unknown builtin function: {}", name)),
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
        let mut parser = Parser::new(tokens);
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
        let mut parser = Parser::new(tokens);
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
}
