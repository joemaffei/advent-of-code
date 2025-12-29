/// Lexer (tokenizer) for the xmas programming language.
///
/// This module breaks source code into tokens - the smallest meaningful
/// units of the language (like words in a sentence).

/// Position information for a token in the source code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenPos {
    pub line: usize,
    pub column: usize,
}

/// Represents a single token in the xmas language.
///
/// Think of tokens as the "words" of the programming language.
/// For example, the code "x = 5" becomes three tokens:
/// - Identifier("x")
/// - Equals
/// - Number(5)
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords - special words with meaning in the language
    If,
    For,
    Of,
    Input,
    Len,
    Max,
    Min,
    Floor,
    Ceil,

    // Literals - actual values
    Number(i64),           // Numbers like 5, -10 (integers only)
    String(String),        // Text in quotes like "hello"
    True,                  // true
    False,                 // false

    // Operators - symbols that do operations
    Plus,                  // +
    Minus,                 // -
    Star,                  // *
    Slash,                 // /
    Percent,               // %
    Tilde,                 // ~ (type conversion)
    Bang,                  // ! (logical NOT)
    Pipe,                  // | (part of |>)
    PipeGreater,           // |> (pipe operator)
    And,                   // && (logical AND)
    Or,                    // || (logical OR)

    // Comparison operators
    Less,                  // <
    Greater,               // >
    LessEqual,             // <=
    GreaterEqual,          // >=
    EqualEqual,            // ==

    // Assignment operators
    PlusEqual,             // +=
    MinusEqual,            // -=
    StarEqual,             // *=
    SlashEqual,            // /=
    PercentEqual,          // %=

    // Punctuation
    LeftParen,             // (
    RightParen,            // )
    LeftBrace,            // {
    RightBrace,           // }
    LeftBracket,          // [
    RightBracket,         // ]
    Comma,                 // ,
    Equals,                // =
    Dot,                   // .
    DotDot,                // .. (range operator)
    Underscore,            // _ (return value)

    // Identifiers - names for variables and functions
    Identifier(String),

    // Special
    Comment(String),       // // comments
    Newline,               // \n
    Eof,                   // End of file
}

/// The lexer converts source code into a stream of tokens.
///
/// Think of it like reading a book word by word - the lexer reads
/// the code character by character and groups them into tokens.
pub struct Lexer {
    /// The source code we're tokenizing
    source: Vec<char>,
    /// Current position in the source code
    current: usize,
    /// Current line number (for error messages)
    line: usize,
    /// Current column number (for error messages)
    column: usize,
}

impl Lexer {
    /// Create a new lexer for the given source code.
    ///
    /// This is like opening a book to start reading it.
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Get the next token from the source code.
    ///
    /// This is the main function - it reads characters and returns
    /// the next token it finds.
    pub fn next_token(&mut self) -> Token {
        // Skip whitespace (spaces, tabs) but keep track of newlines
        self.skip_whitespace();

        // Check if we've reached the end of the file
        if self.is_at_end() {
            return Token::Eof;
        }

        // Get the current character
        let ch = self.advance();

        // Match the character to determine what token it is
        match ch {
            // Single character tokens
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            '+' => {
                if self.match_char('=') {
                    Token::PlusEqual
                } else {
                    Token::Plus
                }
            }
            '-' => {
                if self.match_char('=') {
                    Token::MinusEqual
                } else {
                    Token::Minus
                }
            }
            '*' => {
                if self.match_char('=') {
                    Token::StarEqual
                } else {
                    Token::Star
                }
            }
            '/' => {
                // Could be division, assignment, or start of comment
                if self.match_char('/') {
                    // It's a comment - read until end of line
                    let comment = self.read_comment();
                    Token::Comment(comment)
                } else if self.match_char('=') {
                    Token::SlashEqual
                } else {
                    Token::Slash
                }
            }
            '%' => {
                if self.match_char('=') {
                    Token::PercentEqual
                } else {
                    Token::Percent
                }
            }
            '~' => Token::Tilde,
            '!' => Token::Bang,
            '=' => {
                if self.match_char('=') {
                    Token::EqualEqual
                } else {
                    Token::Equals
                }
            }
            '<' => {
                if self.match_char('=') {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    Token::GreaterEqual
                } else if self.match_char('|') {
                    // |> pipe operator
                    Token::PipeGreater
                } else {
                    Token::Greater
                }
            }
            '|' => {
                if self.match_char('>') {
                    Token::PipeGreater
                } else if self.match_char('|') {
                    Token::Or
                } else {
                    Token::Pipe
                }
            }
            '&' => {
                if self.match_char('&') {
                    Token::And
                } else {
                    // Single & is not a valid token - treat as error
                    Token::Eof // Placeholder - should error
                }
            }
            '.' => {
                if self.match_char('.') {
                    Token::DotDot
                } else {
                    Token::Dot
                }
            }
            '_' => {
                // Check if followed by alphanumeric - if so, treat as identifier starting with _
                if !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
                    self.read_identifier_or_keyword('_')
                } else {
                    Token::Underscore
                }
            },
            '\n' => {
                self.line += 1;
                self.column = 1;
                Token::Newline
            }
            '"' => {
                // String literal - read until closing quote
                Token::String(self.read_string())
            }
            c if c.is_ascii_digit() => {
                // Number - could be integer or float
                self.read_number(c)
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                // Identifier or keyword - starts with letter or underscore
                self.read_identifier_or_keyword(c)
            }
            _ => {
                // Unknown character - in a real compiler we'd report an error
                // For now, we'll just skip it and try the next character
                self.next_token()
            }
        }
    }

    /// Check if we've reached the end of the source code.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Get the current character without moving forward.
    /// This is like "peeking" ahead.
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// Get the character two positions ahead (for multi-character operators).
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// Move forward one character and return it.
    /// This is like reading the next character in a book.
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let ch = self.source[self.current];
            self.current += 1;
            self.column += 1;
            ch
        }
    }

    /// Check if the next character matches, and if so, consume it.
    /// This is useful for two-character operators like ==, <=, etc.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    /// Skip whitespace characters (spaces, tabs) but keep newlines.
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let ch = self.peek();
            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    // Don't skip newlines - they're tokens
                    break;
                }
                _ => break,
            }
        }
    }

    /// Read a string literal (text in quotes).
    ///
    /// Example: "hello" -> String("hello")
    fn read_string(&mut self) -> String {
        let mut result = String::new();

        // Read characters until we find the closing quote
        while !self.is_at_end() && self.peek() != '"' {
            let ch = self.advance();

            // Handle escape sequences (like \n for newline)
            if ch == '\\' && !self.is_at_end() {
                match self.peek() {
                    'n' => {
                        self.advance();
                        result.push('\n');
                    }
                    't' => {
                        self.advance();
                        result.push('\t');
                    }
                    '\\' => {
                        self.advance();
                        result.push('\\');
                    }
                    '"' => {
                        self.advance();
                        result.push('"');
                    }
                    _ => {
                        result.push(ch);
                    }
                }
            } else {
                result.push(ch);
            }
        }

        // Consume the closing quote
        if !self.is_at_end() {
            self.advance();
        }

        result
    }

    /// Read a number (integer only).
    ///
    /// Examples: 5 -> Number(5), -10 -> Number(-10)
    fn read_number(&mut self, first_digit: char) -> Token {
        let mut result = String::from(first_digit);

        // Read digits (no decimal points - integers only)
        while !self.is_at_end() {
            let ch = self.peek();
            if ch.is_ascii_digit() {
                result.push(self.advance());
            } else if ch == '.' {
                // Check if this is part of .. (range operator)
                if self.peek_next() == '.' {
                    // This is .., don't consume the dot
                    break;
                }
                // Float syntax not supported yet - treat as end of number
                break;
            } else {
                break;
            }
        }

        // Convert string to integer
        match result.parse::<i64>() {
            Ok(num) => Token::Number(num),
            Err(_) => Token::Number(0), // Shouldn't happen, but handle gracefully
        }
    }

    /// Read an identifier (variable/function name) or keyword.
    ///
    /// Keywords are special words like "if", "for", etc.
    /// Identifiers are user-defined names like "myVariable".
    fn read_identifier_or_keyword(&mut self, first_char: char) -> Token {
        let mut result = String::from(first_char);

        // Read letters, digits, and underscores
        while !self.is_at_end() {
            let ch = self.peek();
            if ch.is_ascii_alphanumeric() || ch == '_' {
                result.push(self.advance());
            } else {
                break;
            }
        }

        // Check if it's a keyword or boolean literal
        match result.as_str() {
            "if" => Token::If,
            "for" => Token::For,
            "of" => Token::Of,
            "input" => Token::Input,
            "len" => Token::Len,
            "max" => Token::Max,
            "min" => Token::Min,
            "floor" => Token::Floor,
            "ceil" => Token::Ceil,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(result),
        }
    }

    /// Read a comment (everything after // until end of line).
    fn read_comment(&mut self) -> String {
        let mut result = String::new();

        // Read until end of line
        while !self.is_at_end() && self.peek() != '\n' {
            result.push(self.advance());
        }

        result
    }

    /// Get the next token with its position.
    /// This is an internal method used by tokenize().
    fn next_token_with_pos(&mut self) -> (Token, TokenPos) {
        // Skip whitespace (spaces, tabs) but keep track of newlines
        self.skip_whitespace();

        // Capture position at the start of the token (after whitespace)
        let pos = TokenPos {
            line: self.line,
            column: self.column,
        };

        // Check if we've reached the end of the file
        if self.is_at_end() {
            return (Token::Eof, pos);
        }

        // Get the current character
        let ch = self.advance();

        // Match the character to determine what token it is
        let token = match ch {
            // Single character tokens
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            '+' => {
                if self.match_char('=') {
                    Token::PlusEqual
                } else {
                    Token::Plus
                }
            }
            '-' => {
                if self.match_char('=') {
                    Token::MinusEqual
                } else {
                    Token::Minus
                }
            }
            '*' => {
                if self.match_char('=') {
                    Token::StarEqual
                } else {
                    Token::Star
                }
            }
            '/' => {
                // Could be division, assignment, or start of comment
                if self.match_char('/') {
                    // It's a comment - read until end of line
                    let comment = self.read_comment();
                    Token::Comment(comment)
                } else if self.match_char('=') {
                    Token::SlashEqual
                } else {
                    Token::Slash
                }
            }
            '%' => {
                if self.match_char('=') {
                    Token::PercentEqual
                } else {
                    Token::Percent
                }
            }
            '~' => Token::Tilde,
            '!' => Token::Bang,
            '=' => {
                if self.match_char('=') {
                    Token::EqualEqual
                } else {
                    Token::Equals
                }
            }
            '<' => {
                if self.match_char('=') {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    Token::GreaterEqual
                } else if self.match_char('|') {
                    // |> pipe operator
                    Token::PipeGreater
                } else {
                    Token::Greater
                }
            }
            '|' => {
                if self.match_char('>') {
                    Token::PipeGreater
                } else if self.match_char('|') {
                    Token::Or
                } else {
                    Token::Pipe
                }
            }
            '&' => {
                if self.match_char('&') {
                    Token::And
                } else {
                    // Single & is not a valid token - treat as error
                    Token::Eof // Placeholder - should error
                }
            }
            '.' => {
                if self.match_char('.') {
                    Token::DotDot
                } else {
                    Token::Dot
                }
            }
            '_' => {
                // Check if followed by alphanumeric - if so, treat as identifier starting with _
                if !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
                    self.read_identifier_or_keyword('_')
                } else {
                    Token::Underscore
                }
            },
            '\n' => {
                self.line += 1;
                self.column = 1;
                Token::Newline
            }
            '"' => {
                // String literal - read until closing quote
                Token::String(self.read_string())
            }
            c if c.is_ascii_digit() => {
                // Number - could be integer or float
                self.read_number(c)
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                // Identifier or keyword - starts with letter or underscore
                self.read_identifier_or_keyword(c)
            }
            _ => {
                // Unknown character - in a real compiler we'd report an error
                // For now, we'll just skip it and try the next character
                return self.next_token_with_pos();
            }
        };

        (token, pos)
    }

    /// Tokenize the entire source code into a vector of tokens with positions.
    ///
    /// This is the main entry point - it reads all tokens until
    /// the end of the file.
    pub fn tokenize(&mut self) -> Vec<(Token, TokenPos)> {
        let mut tokens = Vec::new();

        loop {
            let (token, pos) = self.next_token_with_pos();
            let is_eof = matches!(token, Token::Eof);

            // Don't add EOF token, but include comments and newlines
            if !is_eof {
                tokens.push((token, pos));
            }

            if is_eof {
                break;
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("(){}[],=+-*/%");
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::LeftBracket);
        assert_eq!(lexer.next_token(), Token::RightBracket);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Star);
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Percent);
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("< <= > >= == =");
        assert_eq!(lexer.next_token(), Token::Less);
        assert_eq!(lexer.next_token(), Token::LessEqual);
        assert_eq!(lexer.next_token(), Token::Greater);
        assert_eq!(lexer.next_token(), Token::GreaterEqual);
        assert_eq!(lexer.next_token(), Token::EqualEqual);
        assert_eq!(lexer.next_token(), Token::Equals);
    }

    #[test]
    fn test_special_operators() {
        let mut lexer = Lexer::new("~ |> | .. . _");
        assert_eq!(lexer.next_token(), Token::Tilde);
        assert_eq!(lexer.next_token(), Token::PipeGreater);
        assert_eq!(lexer.next_token(), Token::Pipe);
        assert_eq!(lexer.next_token(), Token::DotDot);
        assert_eq!(lexer.next_token(), Token::Dot);
        assert_eq!(lexer.next_token(), Token::Underscore);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("0 42 100 -5");
        assert_eq!(lexer.next_token(), Token::Number(0));
        assert_eq!(lexer.next_token(), Token::Number(42));
        assert_eq!(lexer.next_token(), Token::Number(100));
        assert_eq!(lexer.next_token(), Token::Minus); // - is separate token
        assert_eq!(lexer.next_token(), Token::Number(5));
    }

    #[test]
    fn test_booleans() {
        let mut lexer = Lexer::new("true false");
        assert_eq!(lexer.next_token(), Token::True);
        assert_eq!(lexer.next_token(), Token::False);
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello" "world" "with\nnewline""#);
        assert_eq!(lexer.next_token(), Token::String("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::String("world".to_string()));
        assert_eq!(lexer.next_token(), Token::String("with\nnewline".to_string()));
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("if for of input len max min floor ceil");
        assert_eq!(lexer.next_token(), Token::If);
        assert_eq!(lexer.next_token(), Token::For);
        assert_eq!(lexer.next_token(), Token::Of);
        assert_eq!(lexer.next_token(), Token::Input);
        assert_eq!(lexer.next_token(), Token::Len);
        assert_eq!(lexer.next_token(), Token::Max);
        assert_eq!(lexer.next_token(), Token::Min);
        assert_eq!(lexer.next_token(), Token::Floor);
        assert_eq!(lexer.next_token(), Token::Ceil);
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("x myVar function_name var123");
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("myVar".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("function_name".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("var123".to_string()));
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("x = 5 // this is a comment");
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::Comment(" this is a comment".to_string()));
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::new("x   y\tz");
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("z".to_string()));
    }

    #[test]
    fn test_newlines() {
        let mut lexer = Lexer::new("x\ny\nz");
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));
        assert_eq!(lexer.next_token(), Token::Newline);
        assert_eq!(lexer.next_token(), Token::Identifier("z".to_string()));
    }

    #[test]
    fn test_assignment() {
        let mut lexer = Lexer::new("a = 1");
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::Number(1));
    }

    #[test]
    fn test_function_definition() {
        let mut lexer = Lexer::new("add(a, b) = { _ = a + b }");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].0, Token::Identifier("add".to_string()));
        assert_eq!(tokens[1].0, Token::LeftParen);
        assert_eq!(tokens[2].0, Token::Identifier("a".to_string()));
        assert_eq!(tokens[3].0, Token::Comma);
        assert_eq!(tokens[4].0, Token::Identifier("b".to_string()));
        assert_eq!(tokens[5].0, Token::RightParen);
        assert_eq!(tokens[6].0, Token::Equals);
        assert_eq!(tokens[7].0, Token::LeftBrace);
        assert_eq!(tokens[8].0, Token::Underscore);
        assert_eq!(tokens[9].0, Token::Equals);
        assert_eq!(tokens[10].0, Token::Identifier("a".to_string()));
        assert_eq!(tokens[11].0, Token::Plus);
        assert_eq!(tokens[12].0, Token::Identifier("b".to_string()));
        assert_eq!(tokens[13].0, Token::RightBrace);
    }

    #[test]
    fn test_array_literal() {
        let mut lexer = Lexer::new("[1, 2, 3]");
        assert_eq!(lexer.next_token(), Token::LeftBracket);
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(2));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(3));
        assert_eq!(lexer.next_token(), Token::RightBracket);
    }

    #[test]
    fn test_array_slicing() {
        let mut lexer = Lexer::new("arr[1..5]");
        assert_eq!(lexer.next_token(), Token::Identifier("arr".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftBracket);
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::DotDot);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::RightBracket);
    }

    #[test]
    fn test_input_access() {
        let mut lexer = Lexer::new("input[0, 5]");
        assert_eq!(lexer.next_token(), Token::Input);
        assert_eq!(lexer.next_token(), Token::LeftBracket);
        assert_eq!(lexer.next_token(), Token::Number(0));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::RightBracket);
    }

    #[test]
    fn test_pipe_operator() {
        let mut lexer = Lexer::new("addOne |> addTwo");
        assert_eq!(lexer.next_token(), Token::Identifier("addOne".to_string()));
        assert_eq!(lexer.next_token(), Token::PipeGreater);
        assert_eq!(lexer.next_token(), Token::Identifier("addTwo".to_string()));
    }

    #[test]
    fn test_type_conversion() {
        let mut lexer = Lexer::new(r#"~"123""#);
        assert_eq!(lexer.next_token(), Token::Tilde);
        assert_eq!(lexer.next_token(), Token::String("123".to_string()));
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("if(x == 5, { y = 10 }, { y = 20 })");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].0, Token::If);
        assert_eq!(tokens[1].0, Token::LeftParen);
        assert_eq!(tokens[2].0, Token::Identifier("x".to_string()));
        assert_eq!(tokens[3].0, Token::EqualEqual);
        assert_eq!(tokens[4].0, Token::Number(5));
        assert_eq!(tokens[5].0, Token::Comma);
        assert_eq!(tokens[6].0, Token::LeftBrace);
        assert_eq!(tokens[7].0, Token::Identifier("y".to_string()));
        assert_eq!(tokens[8].0, Token::Equals);
        assert_eq!(tokens[9].0, Token::Number(10));
        assert_eq!(tokens[10].0, Token::RightBrace);
        assert_eq!(tokens[11].0, Token::Comma);
        assert_eq!(tokens[12].0, Token::LeftBrace);
        assert_eq!(tokens[13].0, Token::Identifier("y".to_string()));
        assert_eq!(tokens[14].0, Token::Equals);
        assert_eq!(tokens[15].0, Token::Number(20));
        assert_eq!(tokens[16].0, Token::RightBrace);
        assert_eq!(tokens[17].0, Token::RightParen);
    }

    #[test]
    fn test_for_loop() {
        let mut lexer = Lexer::new("for(n of arr, { _ = _ + n }, 0)");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].0, Token::For);
        assert_eq!(tokens[1].0, Token::LeftParen);
        assert_eq!(tokens[2].0, Token::Identifier("n".to_string()));
        assert_eq!(tokens[3].0, Token::Of);
        assert_eq!(tokens[4].0, Token::Identifier("arr".to_string()));
        assert_eq!(tokens[5].0, Token::Comma);
        assert_eq!(tokens[6].0, Token::LeftBrace);
        assert_eq!(tokens[7].0, Token::Underscore);
        assert_eq!(tokens[8].0, Token::Equals);
        assert_eq!(tokens[9].0, Token::Underscore);
        assert_eq!(tokens[10].0, Token::Plus);
        assert_eq!(tokens[11].0, Token::Identifier("n".to_string()));
        assert_eq!(tokens[12].0, Token::RightBrace);
        assert_eq!(tokens[13].0, Token::Comma);
        assert_eq!(tokens[14].0, Token::Number(0));
        assert_eq!(tokens[15].0, Token::RightParen);
    }

    #[test]
    fn test_empty_source() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_multiline_code() {
        let code = r#"
a = 1
b = 2
sum = a + b
"#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();

        // Should have identifiers, equals, numbers, plus, and newlines
        assert!(tokens.iter().any(|(t, _)| matches!(t, Token::Identifier(ref s) if s == "a")));
        assert!(tokens.iter().any(|(t, _)| matches!(t, Token::Identifier(ref s) if s == "b")));
        assert!(tokens.iter().any(|(t, _)| matches!(t, Token::Identifier(ref s) if s == "sum")));
        assert!(tokens.iter().any(|(t, _)| matches!(t, Token::Number(1))));
        assert!(tokens.iter().any(|(t, _)| matches!(t, Token::Number(2))));
        assert!(tokens.iter().any(|(t, _)| matches!(t, Token::Plus)));
        assert!(tokens.iter().any(|(t, _)| matches!(t, Token::Newline)));
    }

    #[test]
    fn test_assignment_operators() {
        let mut lexer = Lexer::new("x += 3");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].0, Token::Identifier("x".to_string()));
        assert_eq!(tokens[1].0, Token::PlusEqual);
        assert_eq!(tokens[2].0, Token::Number(3));

        let mut lexer = Lexer::new("x -= 5");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[1].0, Token::MinusEqual);

        let mut lexer = Lexer::new("x *= 2");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[1].0, Token::StarEqual);

        let mut lexer = Lexer::new("x /= 4");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[1].0, Token::SlashEqual);

        let mut lexer = Lexer::new("x %= 7");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[1].0, Token::PercentEqual);
    }
}
