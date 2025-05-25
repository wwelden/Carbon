use crate::error::{CarbonError, Result};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Real(f64),
    String(String),
    Bool(bool),
    Null,
    NullPtr,

    // Identifiers and Keywords
    Identifier(String),
    ClassName(String),

    // Mathematical Constants
    E,
    Pi,
    Phi,
    Avogadro,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    // Compound Assignment
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,

    // Increment/Decrement
    Increment,
    Decrement,

    // Comparison
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    // Logical
    And,
    Or,
    Not,

    // Bitwise operators
    BitAnd,        // &
    BitOr,         // |
    BitXor,        // ^
    BitNot,        // ~
    BitLeft,       // <<
    BitRight,      // >>

    // Assignment
    Assign,

    // Keywords
    Let,
    Const,
    Var,
    If,
    Else,
    For,
    While,
    Do,
    In,
    Function,
    Fn,
    Return,
    Class,
    New,
    This,
    Match,

    // Built-in Functions
    Print,
    ToString,
    TypeOf,
    MathSqrt,
    Err,
    IsErr,

    // Conversion functions
    ToBinary,
    ToHex,
    ToDecimal,
    Sum,

    // Types
    IntType,
    BoolType,
    StringType,
    ArrayListType,
    SetType,
    MapType,
    StackType,
    QueueType,
    LinkedListType,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Arrow,
    Question,
    Colon,
    Wildcard,

    // Special
    Eof,
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            // Skip comments
            if self.match_string("//") {
                self.skip_line_comment();
                continue;
            }

            let _start_line = self.line;
            let _start_column = self.column;

            match self.scan_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => continue, // Whitespace or comment
                Err(msg) => return Err(msg),
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>> {
        let c = self.advance();

        match c {
            // Single character tokens
            '(' => Ok(Some(Token::LeftParen)),
            ')' => Ok(Some(Token::RightParen)),
            '{' => Ok(Some(Token::LeftBrace)),
            '}' => Ok(Some(Token::RightBrace)),
            '[' => Ok(Some(Token::LeftBracket)),
            ']' => Ok(Some(Token::RightBracket)),
            ';' => Ok(Some(Token::Semicolon)),
            ',' => Ok(Some(Token::Comma)),
            '.' => Ok(Some(Token::Dot)),
            '?' => Ok(Some(Token::Question)),
            ':' => Ok(Some(Token::Colon)),
            '|' => {
                if self.match_char('|') {
                    Ok(Some(Token::Or))
                } else {
                    Ok(Some(Token::BitOr))
                }
            }
            '_' => Ok(Some(Token::Wildcard)),

            // Operators that can be compound
            '+' => {
                if self.match_char('=') {
                    Ok(Some(Token::PlusAssign))
                } else if self.match_char('+') {
                    Ok(Some(Token::Increment))
                } else {
                    Ok(Some(Token::Plus))
                }
            }
            '-' => {
                if self.match_char('=') {
                    Ok(Some(Token::MinusAssign))
                } else if self.match_char('-') {
                    Ok(Some(Token::Decrement))
                } else {
                    Ok(Some(Token::Minus))
                }
            }
            '*' => {
                if self.match_char('=') {
                    Ok(Some(Token::MultiplyAssign))
                } else if self.match_char('*') {
                    Ok(Some(Token::Exponent))
                } else {
                    Ok(Some(Token::Multiply))
                }
            }
            '/' => {
                if self.match_char('=') {
                    Ok(Some(Token::DivideAssign))
                } else {
                    Ok(Some(Token::Divide))
                }
            }
            '%' => {
                if self.match_char('=') {
                    Ok(Some(Token::ModuloAssign))
                } else {
                    Ok(Some(Token::Modulo))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Ok(Some(Token::NotEqual))
                } else {
                    Ok(Some(Token::Not))
                }
            }
            '=' => {
                if self.match_char('=') {
                    Ok(Some(Token::Equal))
                } else if self.match_char('>') {
                    Ok(Some(Token::Arrow))
                } else {
                    Ok(Some(Token::Assign))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(Some(Token::GreaterEqual))
                } else if self.match_char('>') {
                    Ok(Some(Token::BitRight))
                } else {
                    Ok(Some(Token::Greater))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(Some(Token::LessEqual))
                } else if self.match_char('<') {
                    Ok(Some(Token::BitLeft))
                } else {
                    Ok(Some(Token::Less))
                }
            }
            '&' => {
                if self.match_char('&') {
                    Ok(Some(Token::And))
                } else {
                    Ok(Some(Token::BitAnd))
                }
            }
            '^' => Ok(Some(Token::BitXor)),
            '~' => Ok(Some(Token::BitNot)),
            // String literals
            '"' => self.scan_string(),

            // Numbers
            '0'..='9' => self.scan_number(),

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' => self.scan_identifier(),

            _ => Err(CarbonError::lexer_error(&format!("Unexpected character '{}'", c), self.line, self.column)),
        }
    }

    fn scan_string(&mut self) -> Result<Option<Token>> {
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            let c = self.advance();
            if c == '\\' && !self.is_at_end() {
                // Handle escape sequences
                match self.advance() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    c => {
                        value.push('\\');
                        value.push(c);
                    }
                }
            } else {
                value.push(c);
            }
        }

        if self.is_at_end() {
            return Err(CarbonError::lexer_error("Unterminated string", self.line, self.column));
        }

        // Consume closing "
        self.advance();

        Ok(Some(Token::String(value)))
    }

    fn scan_number(&mut self) -> Result<Option<Token>> {
        let start = self.position - 1;

        // Consume digits
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for decimal point
        let mut is_float = false;
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            is_float = true;
            self.advance(); // consume '.'

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let text = &self.input[start..self.position];

        if is_float {
            match text.parse::<f64>() {
                Ok(value) => Ok(Some(Token::Real(value))),
                Err(_) => Err(CarbonError::lexer_error(&format!("Invalid number: {}", text), self.line, self.column)),
            }
        } else {
            match text.parse::<i64>() {
                Ok(value) => Ok(Some(Token::Int(value))),
                Err(_) => Err(CarbonError::lexer_error(&format!("Invalid number: {}", text), self.line, self.column)),
            }
        }
    }

    fn scan_identifier(&mut self) -> Result<Option<Token>> {
        let start = self.position - 1;

        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        // Handle Math.sqrt specially
        if &self.input[start..self.position] == "Math" && self.peek() == '.' {
            let math_start = self.position;
            self.advance(); // consume '.'

            while self.is_alpha(self.peek()) {
                self.advance();
            }

            let method = &self.input[math_start + 1..self.position];
            if method == "sqrt" {
                return Ok(Some(Token::MathSqrt));
            } else {
                // Reset position if not sqrt
                self.position = math_start;
            }
        }

        let text = &self.input[start..self.position];

        let token = match text {
            // Keywords
            "let" => Token::Let,
            "const" => Token::Const,
            "var" => Token::Var,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "do" => Token::Do,
            "in" => Token::In,
            "function" => Token::Function,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "class" => Token::Class,
            "new" => Token::New,
            "this" => Token::This,
            "match" => Token::Match,

            // Built-in functions
            "print" => Token::Print,
            "toString" => Token::ToString,
            "typeof" => Token::TypeOf,
            "err" => Token::Err,
            "isErr" => Token::IsErr,
            "toBinary" => Token::ToBinary,
            "toHex" => Token::ToHex,
            "toDecimal" => Token::ToDecimal,
            "sum" => Token::Sum,

            // Types
            "int" => Token::IntType,
            "bool" => Token::BoolType,
            "string" => Token::StringType,
            "ArrayList" => Token::ArrayListType,
            "Set" => Token::SetType,
            "Map" => Token::MapType,
            "Stack" => Token::StackType,
            "Queue" => Token::QueueType,
            "LinkedList" => Token::LinkedListType,

            // Literals
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "null" => Token::Null,
            "nullptr" => Token::NullPtr,

            // Constants
            "E" => Token::E,
            "PI" => Token::Pi,
            "PHI" => Token::Phi,
            "AVOGADRO" => Token::Avogadro,

            _ => {
                // Check if it's a class name (starts with capital)
                if text.chars().next().unwrap().is_uppercase() {
                    Token::ClassName(text.to_string())
                } else {
                    Token::Identifier(text.to_string())
                }
            }
        };

        Ok(Some(token))
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    fn match_string(&self, expected: &str) -> bool {
        if self.position + expected.len() > self.input.len() {
            return false;
        }

        &self.input[self.position..self.position + expected.len()] == expected
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.position += 1;
        self.column += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input.chars().nth(self.position).unwrap_or('\0')
        }
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.position + 1).unwrap_or('\0')
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int(i) => write!(f, "{}", i),
            Token::Real(r) => write!(f, "{}", r),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Bool(b) => write!(f, "{}", b),
            Token::Null => write!(f, "null"),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::ClassName(name) => write!(f, "{}", name),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Arrow => write!(f, "=>"),
            Token::BitAnd => write!(f, "&"),
            Token::BitOr => write!(f, "|"),
            Token::BitXor => write!(f, "^"),
            Token::BitNot => write!(f, "~"),
            Token::BitLeft => write!(f, "<<"),
            Token::BitRight => write!(f, ">>"),
            _ => write!(f, "{:?}", self),
        }
    }
}
