/// Parser for the xmas programming language.
///
/// The parser takes tokens from the lexer and builds an Abstract Syntax Tree (AST).
/// Think of it like a grammar checker - it verifies the code follows the language rules
/// and organizes it into a tree structure.

use crate::ast::*;
use crate::lexer::{Token, TokenPos};

/// The parser converts a stream of tokens into an AST.
pub struct Parser {
    /// The tokens to parse with their positions
    tokens: Vec<(Token, TokenPos)>,
    /// Current position in the token stream
    current: usize,
    /// Source code for error messages
    source: String,
}

impl Parser {
    /// Create a new parser from a vector of tokens with positions and source code.
    pub fn new(tokens: Vec<(Token, TokenPos)>, source: String) -> Self {
        Parser { tokens, current: 0, source }
    }

    /// Parse a complete program (list of statements).
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip comments and newlines at statement level
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }

            // Skip newlines between statements
            self.skip_whitespace_and_comments();
        }

        Ok(statements)
    }

    /// Parse a single statement.
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        self.skip_whitespace_and_comments();

        // Check for function definition: name(params) = ...
        // We need to look ahead to see if there's an '=' after the closing paren
        if self.check_identifier() && self.check_next(Token::LeftParen) {
            // Look ahead to see if this is a function definition (has = after params)
            // or a function call (no =)
            let saved_current = self.current;
            self.advance(); // skip identifier
            self.advance(); // skip (

            // Skip parameters
            let mut paren_count = 1;
            while paren_count > 0 && !self.is_at_end() {
                let token = self.advance();
                match token {
                    Token::LeftParen => paren_count += 1,
                    Token::RightParen => paren_count -= 1,
                    _ => {}
                }
            }

            // Check if next token is =
            let is_function_def = self.check(Token::Equals);
            self.current = saved_current; // restore position

            if is_function_def {
                return self.parse_function();
            }
        }

        // Check for assignment: name = ... or name += ... or _ = ... or _name = ...
        if self.check_identifier() || self.check(Token::Underscore) {
            let name = if self.check(Token::Underscore) {
                self.advance();
                "_".to_string()
            } else {
                if let Token::Identifier(name) = self.advance() {
                    name
                } else {
                    return Err(self.error("Expected identifier"));
                }
            };

            // Check if this is a named return (starts with _ but isn't just _)
            let is_named_return = name.starts_with('_') && name.len() > 1;
            let return_name = if is_named_return {
                Some(name[1..].to_string())  // Extract name after _
            } else if name == "_" {
                None  // Unnamed return
            } else {
                None  // Regular variable
            };

            // Check for assignment operators: +=, -=, *=, /=, %=
            let assignment_op = if self.check(Token::PlusEqual) {
                self.advance();
                Some(BinaryOp::Plus)
            } else if self.check(Token::MinusEqual) {
                self.advance();
                Some(BinaryOp::Minus)
            } else if self.check(Token::StarEqual) {
                self.advance();
                Some(BinaryOp::Star)
            } else if self.check(Token::SlashEqual) {
                self.advance();
                Some(BinaryOp::Slash)
            } else if self.check(Token::PercentEqual) {
                self.advance();
                Some(BinaryOp::Percent)
            } else {
                None
            };

            if let Some(op) = assignment_op {
                // Assignment operator: x += 5
                let value = self.parse_expression()?;
                return Ok(Stmt::AssignOp { name, op, value });
            }

            if self.check(Token::Equals) {
                self.advance();
                let value = self.parse_expression()?;

                if name == "_" || is_named_return {
                    return Ok(Stmt::Return { name: return_name, value });
                } else {
                    return Ok(Stmt::Assign { name, value });
                }
            } else {
                // Not an assignment, parse as expression
                self.current -= 1; // Back up
                return Ok(Stmt::Expr(self.parse_expression()?));
            }
        }

        // Otherwise, parse as expression statement
        Ok(Stmt::Expr(self.parse_expression()?))
    }

    /// Parse a function definition: name(params) = body
    fn parse_function(&mut self) -> Result<Stmt, String> {
        // Get function name
        let name = if let Token::Identifier(name) = self.advance() {
            name
        } else {
            return Err(self.error("Expected function name"));
        };

        // Parse parameters
        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        let mut params = Vec::new();

        if !self.check(Token::RightParen) {
            loop {
                if let Token::Identifier(param) = self.advance() {
                    params.push(param);
                } else {
                    return Err(self.error("Expected parameter name"));
                }

                if !self.check(Token::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.consume(Token::Equals, "Expected '=' after function definition")?;

        // Parse body (expression or block)
        let body = self.parse_expression()?;

        Ok(Stmt::Function { name, params, body })
    }

    /// Parse an expression (with operator precedence).
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_pipe()
    }

    /// Parse pipe operator (lowest precedence): expr |> expr
    fn parse_pipe(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_or()?;

        while self.check(Token::PipeGreater) {
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Pipe {
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse logical OR operator: expr || expr
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and()?;

        while self.check(Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse logical AND operator: expr && expr
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;

        while self.check(Token::And) {
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse comparison operators: <, >, <=, >=, ==
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive()?;

        while self.is_comparison_op() {
            let op = self.comparison_op();
            self.advance();
            let right = self.parse_additive()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse additive operators: +, -
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative()?;

        while self.check(Token::Plus) || self.check(Token::Minus) {
            let op = if self.check(Token::Plus) {
                BinaryOp::Plus
            } else {
                BinaryOp::Minus
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse multiplicative operators: *, /, %
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;

        while self.check(Token::Star) || self.check(Token::Slash) || self.check(Token::Percent) {
            let op = if self.check(Token::Star) {
                BinaryOp::Star
            } else if self.check(Token::Slash) {
                BinaryOp::Slash
            } else {
                BinaryOp::Percent
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse unary operators: ~, !
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.check(Token::Tilde) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Tilde,
                expr: Box::new(expr),
            });
        }

        if self.check(Token::Bang) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Bang,
                expr: Box::new(expr),
            });
        }

        self.parse_primary()
    }

    /// Parse primary expressions (literals, identifiers, function calls, etc.).
    fn parse_primary(&mut self) -> Result<Expr, String> {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Err(self.error("Unexpected end of input"));
        }

        // Check if identifier before advancing (to handle function calls)
        let is_identifier = self.check_identifier();
        let is_function_call = is_identifier && self.check_next(Token::LeftParen);

        let token = self.advance();

        match token {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::String(s) => Ok(Expr::String(s)),
            Token::True => Ok(Expr::Boolean(true)),
            Token::False => Ok(Expr::Boolean(false)),
            Token::Input => {
                // Parse indexing if present
                let expr = Expr::Input;
                self.parse_indexing(expr)
            }
            Token::Identifier(name) => {
                // Could be identifier, function call, or builtin
                if is_function_call {
                    // Function call
                    self.parse_call(name)
                } else {
                    // Regular identifier, check for indexing
                    let expr = Expr::Identifier(name);
                    self.parse_indexing(expr)
                }
            }
            Token::Underscore => {
                let expr = Expr::ReturnValue;
                self.parse_indexing(expr)
            }
            Token::LeftBracket => {
                // Could be array literal [1, 2, 3] or range literal [0..5]
                // Check if it's a range: [expr..expr]
                // Try to parse as range: look for expr..expr]
                let is_range = {
                    let mut temp_parser = Parser {
                        tokens: self.tokens.clone(),
                        current: self.current,
                        source: self.source.clone(),
                    };
                    // Skip whitespace
                    temp_parser.skip_whitespace_and_comments();
                    // Try to parse an expression
                    if let Ok(_start_expr) = temp_parser.parse_expression() {
                        // Check if next is ..
                        temp_parser.skip_whitespace_and_comments();
                        if temp_parser.check(Token::DotDot) {
                            temp_parser.advance(); // consume ..
                            temp_parser.skip_whitespace_and_comments();
                            // Try to parse end expression
                            if let Ok(_end_expr) = temp_parser.parse_expression() {
                                temp_parser.skip_whitespace_and_comments();
                                // Check if next is ]
                                temp_parser.check(Token::RightBracket)
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if is_range {
                    // Parse as range literal [start..end]
                    self.skip_whitespace_and_comments();
                    let start = self.parse_expression()?;
                    self.skip_whitespace_and_comments();
                    self.consume(Token::DotDot, "Expected '..' in range literal")?;
                    self.skip_whitespace_and_comments();
                    let end = self.parse_expression()?;
                    self.skip_whitespace_and_comments();
                    self.consume(Token::RightBracket, "Expected ']' after range")?;
                    let expr = Expr::Range {
                        start: Box::new(start),
                        end: Box::new(end),
                    };
                    self.parse_indexing(expr)
                } else {
                    // Parse as array literal
                    let mut elements = Vec::new();

                    if !self.check(Token::RightBracket) {
                        loop {
                            elements.push(self.parse_expression()?);
                            if !self.check(Token::Comma) {
                                break;
                            }
                            self.advance(); // consume comma
                        }
                    }

                    self.consume(Token::RightBracket, "Expected ']' after array elements")?;
                    let expr = Expr::Array(elements);
                    self.parse_indexing(expr)
                }
            }
            Token::LeftBrace => {
                // Block
                let mut statements = Vec::new();

                while !self.check(Token::RightBrace) && !self.is_at_end() {
                    self.skip_whitespace_and_comments();
                    if self.check(Token::RightBrace) {
                        break;
                    }
                    statements.push(self.parse_statement()?);
                    self.skip_whitespace_and_comments();
                }

                self.consume(Token::RightBrace, "Expected '}' after block")?;
                Ok(Expr::Block(statements))
            }
            Token::LeftParen => {
                // Parenthesized expression for grouping
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after expression")?;
                self.parse_indexing(expr)
            }
            Token::If => {
                // Conditional: if(condition, trueBlock, falseBlock?)
                // The falseBlock is optional
                self.consume(Token::LeftParen, "Expected '(' after 'if'")?;
                let condition = self.parse_expression()?;
                self.consume(Token::Comma, "Expected ',' after condition")?;
                let true_block = self.parse_expression()?;

                // Check if there's a comma (indicating a false block) or right paren (end of if)
                let false_block = if self.check(Token::Comma) {
                    self.advance(); // consume comma
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                self.consume(Token::RightParen, "Expected ')' after if expression")?;

                // Represent if as a builtin call for now
                let mut args = vec![condition, true_block];
                if let Some(fb) = false_block {
                    args.push(fb);
                }
                Ok(Expr::Builtin {
                    name: "if".to_string(),
                    args,
                })
            }
            Token::For => {
                // Loop: for(variable of array, block, initialValue?)
                // initialValue is optional - if not provided, the loop doesn't return anything
                self.consume(Token::LeftParen, "Expected '(' after 'for'")?;

                let var_name = if let Token::Identifier(name) = self.advance() {
                    name
                } else {
                    return Err(self.error("Expected variable name after 'for'"));
                };

                self.consume(Token::Of, "Expected 'of' after variable name")?;
                let array = self.parse_expression()?;
                self.consume(Token::Comma, "Expected ',' after array")?;
                let block = self.parse_expression()?;

                // Check if there's a comma (indicating an initial value) or right paren (end of for)
                let initial = if self.check(Token::Comma) {
                    self.advance(); // consume comma
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                self.consume(Token::RightParen, "Expected ')' after for expression")?;

                // Represent for as a builtin call for now
                let mut args = vec![
                    Expr::Identifier(var_name),
                    array,
                    block,
                ];
                if let Some(init) = initial {
                    args.push(init);
                }
                Ok(Expr::Builtin {
                    name: "for".to_string(),
                    args,
                })
            }
            Token::Len => {
                // Builtin function: len(...)
                self.consume(Token::LeftParen, "Expected '(' after 'len'")?;
                let arg = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after len argument")?;
                Ok(Expr::Builtin {
                    name: "len".to_string(),
                    args: vec![arg],
                })
            }
            Token::Max => {
                // Builtin function: max(a, b)
                self.consume(Token::LeftParen, "Expected '(' after 'max'")?;
                let arg1 = self.parse_expression()?;
                self.consume(Token::Comma, "Expected ',' after first max argument")?;
                let arg2 = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after max arguments")?;
                Ok(Expr::Builtin {
                    name: "max".to_string(),
                    args: vec![arg1, arg2],
                })
            }
            Token::Min => {
                // Builtin function: min(a, b)
                self.consume(Token::LeftParen, "Expected '(' after 'min'")?;
                let arg1 = self.parse_expression()?;
                self.consume(Token::Comma, "Expected ',' after first min argument")?;
                let arg2 = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after min arguments")?;
                Ok(Expr::Builtin {
                    name: "min".to_string(),
                    args: vec![arg1, arg2],
                })
            }
            Token::Floor => {
                // Builtin function: floor(...)
                self.consume(Token::LeftParen, "Expected '(' after 'floor'")?;
                let arg = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after floor argument")?;
                Ok(Expr::Builtin {
                    name: "floor".to_string(),
                    args: vec![arg],
                })
            }
            Token::Ceil => {
                // Builtin function: ceil(...)
                self.consume(Token::LeftParen, "Expected '(' after 'ceil'")?;
                let arg = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after ceil argument")?;
                Ok(Expr::Builtin {
                    name: "ceil".to_string(),
                    args: vec![arg],
                })
            }
            _ => Err(self.error(&format!("Unexpected token: {:?}", token))),
        }
    }

    /// Parse function call: name(args...)
    fn parse_call(&mut self, name: String) -> Result<Expr, String> {
        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        let mut args = Vec::new();

        if !self.check(Token::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.check(Token::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        self.consume(Token::RightParen, "Expected ')' after arguments")?;
        let expr = Expr::Call {
            callee: Box::new(Expr::Identifier(name)),
            args,
        };
        self.parse_indexing(expr)
    }

    /// Parse array indexing/slicing: expr[index] or expr[start..end]
    fn parse_indexing(&mut self, expr: Expr) -> Result<Expr, String> {
        let mut current_expr = expr;

        // Handle multiple levels of indexing: input[0, 5] or arr[1][2]
        while self.check(Token::LeftBracket) {
            self.advance(); // consume '['

            let mut indices = Vec::new();

            // Parse index expressions (can be multiple: input[i, j])
            loop {
                if self.check(Token::DotDot) {
                    // Range: ..end or start..end or start..
                    self.advance();
                    let start = None;
                    let end = if self.check(Token::RightBracket) || self.check(Token::Comma) {
                        None
                    } else {
                        Some(self.parse_expression()?)
                    };
                    indices.push(IndexExpr::Range { start, end });
                } else {
                    let first = self.parse_expression()?;
                    if self.check(Token::DotDot) {
                        // Range: start..end or start..
                        self.advance();
                        let end = if self.check(Token::RightBracket) || self.check(Token::Comma) {
                            None
                        } else {
                            Some(self.parse_expression()?)
                        };
                        indices.push(IndexExpr::Range {
                            start: Some(first),
                            end,
                        });
                    } else {
                        // Single index
                        indices.push(IndexExpr::Single(first));
                    }
                }

                if !self.check(Token::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }

            self.consume(Token::RightBracket, "Expected ']' after index")?;

            current_expr = Expr::Index {
                array: Box::new(current_expr),
                index: indices,
            };
        }

        // Handle method calls: array.rows()
        while self.check(Token::Dot) {
            self.advance(); // consume '.'

            let method_name = if let Token::Identifier(name) = self.advance() {
                name
            } else {
                return Err(self.error("Expected method name after '.'"));
            };

            self.consume(Token::LeftParen, "Expected '(' after method name")?;

            let mut args = Vec::new();
            if !self.check(Token::RightParen) {
                loop {
                    args.push(self.parse_expression()?);
                    if !self.check(Token::Comma) {
                        break;
                    }
                    self.advance(); // consume comma
                }
            }

            self.consume(Token::RightParen, "Expected ')' after method arguments")?;

            current_expr = Expr::MethodCall {
                object: Box::new(current_expr),
                method: method_name,
                args,
            };
        }

        Ok(current_expr)
    }

    // Helper methods

    /// Check if we're at the end of tokens.
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Get the current token without advancing.
    fn peek(&self) -> Option<&Token> {
        if self.is_at_end() {
            None
        } else {
            Some(&self.tokens[self.current].0)
        }
    }

    /// Get the current token position without advancing.
    fn peek_pos(&self) -> Option<TokenPos> {
        if self.is_at_end() {
            None
        } else {
            Some(self.tokens[self.current].1)
        }
    }

    /// Check if current token matches.
    fn check(&self, token: Token) -> bool {
        match (self.peek(), &token) {
            (Some(Token::Number(_)), Token::Number(_)) => true,
            (Some(Token::String(_)), Token::String(_)) => true,
            (Some(Token::Identifier(_)), Token::Identifier(_)) => true,
            (Some(Token::Comment(_)), Token::Comment(_)) => true,
            (Some(t), expected) => std::mem::discriminant(t) == std::mem::discriminant(expected),
            _ => false,
        }
    }

    /// Create a formatted error message with position information.
    fn error(&self, message: &str) -> String {
        if let Some(pos) = self.peek_pos() {
            self.format_error(message, pos)
        } else {
            format!("Parse error: {} (at end of input)", message)
        }
    }

    /// Format an error message with line/column and source line with caret.
    fn format_error(&self, message: &str, pos: TokenPos) -> String {
        let lines: Vec<&str> = self.source.lines().collect();
        if pos.line > 0 && pos.line <= lines.len() {
            let line_content = lines[pos.line - 1];
            let caret_pos = if pos.column > 0 && pos.column <= line_content.len() {
                pos.column - 1
            } else if pos.column > line_content.len() {
                line_content.len()
            } else {
                0
            };
            let caret = " ".repeat(caret_pos) + "^";
            format!(
                "Parse error: {} (line {}, column {})\n{}\n{}",
                message, pos.line, pos.column, line_content, caret
            )
        } else {
            format!("Parse error: {} (line {}, column {})", message, pos.line, pos.column)
        }
    }

    /// Check if current token is an identifier.
    fn check_identifier(&self) -> bool {
        matches!(self.peek(), Some(Token::Identifier(_)))
    }

    /// Check if next token matches.
    fn check_next(&self, token: Token) -> bool {
        if self.current + 1 >= self.tokens.len() {
            false
        } else {
            match (&self.tokens[self.current + 1].0, &token) {
                (Token::Number(_), Token::Number(_)) => true,
                (Token::String(_), Token::String(_)) => true,
                (Token::Identifier(_), Token::Identifier(_)) => true,
                (Token::Comment(_), Token::Comment(_)) => true,
                (t, expected) => std::mem::discriminant(t) == std::mem::discriminant(expected),
            }
        }
    }

    /// Advance and return the current token.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].0.clone()
    }

    /// Consume a token, error if it doesn't match.
    fn consume(&mut self, token: Token, message: &str) -> Result<(), String> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    /// Skip whitespace (newlines) and comments.
    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                Some(Token::Newline) | Some(Token::Comment(_)) => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    /// Check if current token is a comparison operator.
    fn is_comparison_op(&self) -> bool {
        self.check(Token::Less)
            || self.check(Token::Greater)
            || self.check(Token::LessEqual)
            || self.check(Token::GreaterEqual)
            || self.check(Token::EqualEqual)
    }

    /// Get the comparison operator from current token.
    fn comparison_op(&self) -> BinaryOp {
        if self.check(Token::Less) {
            BinaryOp::Less
        } else if self.check(Token::Greater) {
            BinaryOp::Greater
        } else if self.check(Token::LessEqual) {
            BinaryOp::LessEqual
        } else if self.check(Token::GreaterEqual) {
            BinaryOp::GreaterEqual
        } else {
            BinaryOp::EqualEqual
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_code(code: &str) -> Result<Program, String> {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens, code.to_string());
        parser.parse()
    }

    #[test]
    fn test_number() {
        let result = parse_code("5").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::Number(n)) = &result[0] {
            assert_eq!(*n, 5);
        } else {
            panic!("Expected number expression");
        }
    }

    #[test]
    fn test_string() {
        let result = parse_code(r#""hello""#).unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::String(s)) = &result[0] {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected string expression");
        }
    }

    #[test]
    fn test_identifier() {
        let result = parse_code("x").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::Identifier(name)) = &result[0] {
            assert_eq!(name, "x");
        } else {
            panic!("Expected identifier expression");
        }
    }

    #[test]
    fn test_assignment() {
        let result = parse_code("x = 5").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Assign { name, value } = &result[0] {
            assert_eq!(name, "x");
            assert_eq!(*value, Expr::Number(5));
        } else {
            panic!("Expected assignment statement");
        }
    }

    #[test]
    fn test_binary_arithmetic() {
        let result = parse_code("1 + 2 * 3").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::Binary { left, op, right }) = &result[0] {
            assert_eq!(*op, BinaryOp::Plus);
            assert_eq!(**left, Expr::Number(1));
            if let Expr::Binary { left, op, right } = right.as_ref() {
                assert_eq!(*op, BinaryOp::Star);
                assert_eq!(**left, Expr::Number(2));
                assert_eq!(**right, Expr::Number(3));
            } else {
                panic!("Expected multiplication");
            }
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_comparison() {
        let result = parse_code("x == 5").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::Binary { op, .. }) = &result[0] {
            assert_eq!(*op, BinaryOp::EqualEqual);
        } else {
            panic!("Expected comparison");
        }
    }

    #[test]
    fn test_array_literal() {
        let result = parse_code("[1, 2, 3]").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::Array(elements)) = &result[0] {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Expr::Number(1));
            assert_eq!(elements[1], Expr::Number(2));
            assert_eq!(elements[2], Expr::Number(3));
        } else {
            panic!("Expected array literal");
        }
    }

    #[test]
    fn test_array_indexing() {
        let result = parse_code("arr[0]").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::Index { array, index }) = &result[0] {
            assert_eq!(**array, Expr::Identifier("arr".to_string()));
            assert_eq!(index.len(), 1);
            if let IndexExpr::Single(Expr::Number(n)) = &index[0] {
                assert_eq!(*n, 0);
            } else {
                panic!("Expected single index");
            }
        } else {
            panic!("Expected index expression");
        }
    }

    #[test]
    fn test_array_slicing() {
        let result = parse_code("arr[1..5]").unwrap();
        assert_eq!(result.len(), 1);
        if let Stmt::Expr(Expr::Index { index, .. }) = &result[0] {
            if let IndexExpr::Range { start, end } = &index[0] {
                assert_eq!(start, &Some(Expr::Number(1)));
                assert_eq!(end, &Some(Expr::Number(5)));
            } else {
                panic!("Expected range");
            }
        } else {
            panic!("Expected index expression");
        }
    }

    #[test]
    fn test_array_slicing_open() {
        let result = parse_code("arr[1..]").unwrap();
        if let Stmt::Expr(Expr::Index { index, .. }) = &result[0] {
            if let IndexExpr::Range { start, end } = &index[0] {
                assert_eq!(start, &Some(Expr::Number(1)));
                assert_eq!(end, &None);
            } else {
                panic!("Expected open range");
            }
        }
    }

    #[test]
    fn test_function_call() {
        let result = parse_code("add(1, 2)").unwrap();
        if let Stmt::Expr(Expr::Call { callee, args }) = &result[0] {
            assert_eq!(**callee, Expr::Identifier("add".to_string()));
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], Expr::Number(1));
            assert_eq!(args[1], Expr::Number(2));
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_function_definition() {
        let result = parse_code("add(a, b) = { _ = a + b }").unwrap();
        if let Stmt::Function { name, params, body } = &result[0] {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "a");
            assert_eq!(params[1], "b");
            if let Expr::Block(statements) = body {
                assert_eq!(statements.len(), 1);
            } else {
                panic!("Expected block body");
            }
        } else {
            panic!("Expected function definition");
        }
    }

    #[test]
    fn test_block() {
        let result = parse_code("{ x = 5; y = 10 }").unwrap();
        if let Stmt::Expr(Expr::Block(statements)) = &result[0] {
            assert_eq!(statements.len(), 2);
        } else {
            panic!("Expected block");
        }
    }

    #[test]
    fn test_return_value() {
        let result = parse_code("_ = 5").unwrap();
        if let Stmt::Return { name, value } = &result[0] {
            assert_eq!(name, &None);
            assert_eq!(*value, Expr::Number(5));
        } else {
            panic!("Expected return statement");
        }
    }

    #[test]
    fn test_input_access() {
        let result = parse_code("input[0]").unwrap();
        if let Stmt::Expr(Expr::Index { array, .. }) = &result[0] {
            assert_eq!(**array, Expr::Input);
        } else {
            panic!("Expected input access");
        }
    }

    #[test]
    fn test_input_2d_access() {
        let result = parse_code("input[0, 5]").unwrap();
        if let Stmt::Expr(Expr::Index { index, .. }) = &result[0] {
            assert_eq!(index.len(), 2);
        } else {
            panic!("Expected 2D input access");
        }
    }

    #[test]
    fn test_conditional() {
        let result = parse_code("if(x == 5, { y = 10 }, { y = 20 })").unwrap();
        if let Stmt::Expr(Expr::Builtin { name, args }) = &result[0] {
            assert_eq!(name, "if");
            assert_eq!(args.len(), 3);
        } else {
            panic!("Expected conditional");
        }
    }

    #[test]
    fn test_conditional_optional_else() {
        let result = parse_code("if(x == 5, { y = 10 })").unwrap();
        if let Stmt::Expr(Expr::Builtin { name, args }) = &result[0] {
            assert_eq!(name, "if");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected conditional");
        }
    }

    #[test]
    fn test_for_loop() {
        let result = parse_code("for(n of arr, { _ = _ + n }, 0)").unwrap();
        if let Stmt::Expr(Expr::Builtin { name, args }) = &result[0] {
            assert_eq!(name, "for");
            assert_eq!(args.len(), 4);
        } else {
            panic!("Expected for loop");
        }
    }

    #[test]
    fn test_for_loop_no_initial() {
        let result = parse_code("for(n of arr, { x = x + n })").unwrap();
        if let Stmt::Expr(Expr::Builtin { name, args }) = &result[0] {
            assert_eq!(name, "for");
            assert_eq!(args.len(), 3);
        } else {
            panic!("Expected for loop without initial value");
        }
    }

    #[test]
    fn test_pipe_operator() {
        let result = parse_code("addOne |> addTwo").unwrap();
        if let Stmt::Expr(Expr::Pipe { left, right }) = &result[0] {
            assert_eq!(**left, Expr::Identifier("addOne".to_string()));
            assert_eq!(**right, Expr::Identifier("addTwo".to_string()));
        } else {
            panic!("Expected pipe expression");
        }
    }

    #[test]
    fn test_unary_tilde() {
        let result = parse_code(r#"~"123""#).unwrap();
        if let Stmt::Expr(Expr::Unary { op, expr }) = &result[0] {
            assert_eq!(*op, UnaryOp::Tilde);
            assert_eq!(**expr, Expr::String("123".to_string()));
        } else {
            panic!("Expected unary expression");
        }
    }

    #[test]
    fn test_builtin_len() {
        let result = parse_code("len(arr)").unwrap();
        if let Stmt::Expr(Expr::Builtin { name, args }) = &result[0] {
            assert_eq!(name, "len");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected builtin call");
        }
    }

    #[test]
    fn test_multiple_statements() {
        let result = parse_code("x = 1\ny = 2\nz = x + y").unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_complex_expression() {
        let result = parse_code("(1 + 2) * 3").unwrap();
        // Note: parentheses aren't in the spec, but we should handle them
        // For now, this will parse as 1 + (2 * 3) due to precedence
        assert_eq!(result.len(), 1);
    }
}
