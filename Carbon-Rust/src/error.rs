use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CarbonError {
    #[error("Lexer error: {message} at line {line}, column {column}")]
    LexerError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Parser error: {message} at line {line}, column {column}")]
    ParserError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Runtime error: {message}")]
    RuntimeError { message: String },

    #[error("Type error: {message}")]
    TypeError { message: String },

    #[error("Variable '{name}' is not defined")]
    UndefinedVariable { name: String },

    #[error("Function '{name}' is not defined")]
    UndefinedFunction { name: String },

    #[error("Class '{name}' is not defined")]
    UndefinedClass { name: String },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Index out of bounds: index {index}, length {length}")]
    IndexOutOfBounds { index: usize, length: usize },

    #[error("Invalid operation: {operation} on {value_type}")]
    InvalidOperation {
        operation: String,
        value_type: String,
    },

    #[error("Pattern match failed: no matching case found")]
    PatternMatchFailed,

    #[error("Arity mismatch: expected {expected} arguments, got {actual}")]
    ArityMismatch { expected: usize, actual: usize },

    #[error("Custom error: {message}")]
    CustomError { message: String },

    #[error("IO error: {message}")]
    IoError { message: String },
}

impl CarbonError {
    pub fn lexer_error(message: &str, line: usize, column: usize) -> Self {
        CarbonError::LexerError {
            message: message.to_string(),
            line,
            column,
        }
    }

    pub fn parser_error(message: &str, line: usize, column: usize) -> Self {
        CarbonError::ParserError {
            message: message.to_string(),
            line,
            column,
        }
    }

    pub fn runtime_error(message: &str) -> Self {
        CarbonError::RuntimeError {
            message: message.to_string(),
        }
    }

    pub fn type_error(message: &str) -> Self {
        CarbonError::TypeError {
            message: message.to_string(),
        }
    }

    pub fn undefined_variable(name: &str) -> Self {
        CarbonError::UndefinedVariable {
            name: name.to_string(),
        }
    }

    pub fn undefined_function(name: &str) -> Self {
        CarbonError::UndefinedFunction {
            name: name.to_string(),
        }
    }

    pub fn undefined_class(name: &str) -> Self {
        CarbonError::UndefinedClass {
            name: name.to_string(),
        }
    }

    pub fn invalid_operation(operation: &str, value_type: &str) -> Self {
        CarbonError::InvalidOperation {
            operation: operation.to_string(),
            value_type: value_type.to_string(),
        }
    }

    pub fn arity_mismatch(expected: usize, actual: usize) -> Self {
        CarbonError::ArityMismatch { expected, actual }
    }

    pub fn custom_error(message: &str) -> Self {
        CarbonError::CustomError {
            message: message.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, CarbonError>;
