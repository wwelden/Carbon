use carbon_interpreter::error::*;

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_lexer_error() {
        let error = CarbonError::lexer_error("Invalid character", 1, 5);
        assert!(matches!(error, CarbonError::LexerError { .. }));
        assert_eq!(error.to_string(), "Lexer error: Invalid character at line 1, column 5");
    }

    #[test]
    fn test_parser_error() {
        let error = CarbonError::parser_error("Unexpected token", 2, 10);
        assert!(matches!(error, CarbonError::ParserError { .. }));
        assert_eq!(error.to_string(), "Parser error: Unexpected token at line 2, column 10");
    }

    #[test]
    fn test_runtime_error() {
        let error = CarbonError::runtime_error("Stack overflow");
        assert!(matches!(error, CarbonError::RuntimeError { .. }));
        assert_eq!(error.to_string(), "Runtime error: Stack overflow");
    }

    #[test]
    fn test_type_error() {
        let error = CarbonError::type_error("Cannot add string to number");
        assert!(matches!(error, CarbonError::TypeError { .. }));
        assert_eq!(error.to_string(), "Type error: Cannot add string to number");
    }

    #[test]
    fn test_undefined_variable() {
        let error = CarbonError::undefined_variable("x");
        assert!(matches!(error, CarbonError::UndefinedVariable { .. }));
        assert_eq!(error.to_string(), "Variable 'x' is not defined");
    }

    #[test]
    fn test_undefined_function() {
        let error = CarbonError::undefined_function("foo");
        assert!(matches!(error, CarbonError::UndefinedFunction { .. }));
        assert_eq!(error.to_string(), "Function 'foo' is not defined");
    }

    #[test]
    fn test_undefined_class() {
        let error = CarbonError::undefined_class("MyClass");
        assert!(matches!(error, CarbonError::UndefinedClass { .. }));
        assert_eq!(error.to_string(), "Class 'MyClass' is not defined");
    }

    #[test]
    fn test_division_by_zero() {
        let error = CarbonError::DivisionByZero;
        assert!(matches!(error, CarbonError::DivisionByZero));
        assert_eq!(error.to_string(), "Division by zero");
    }

    #[test]
    fn test_index_out_of_bounds() {
        let error = CarbonError::IndexOutOfBounds { index: 5, length: 3 };
        assert!(matches!(error, CarbonError::IndexOutOfBounds { .. }));
        assert_eq!(error.to_string(), "Index out of bounds: index 5, length 3");
    }

    #[test]
    fn test_invalid_operation() {
        let error = CarbonError::invalid_operation("sqrt", "string");
        assert!(matches!(error, CarbonError::InvalidOperation { .. }));
        assert_eq!(error.to_string(), "Invalid operation: sqrt on string");
    }

    #[test]
    fn test_pattern_match_failed() {
        let error = CarbonError::PatternMatchFailed;
        assert!(matches!(error, CarbonError::PatternMatchFailed));
        assert_eq!(error.to_string(), "Pattern match failed: no matching case found");
    }

    #[test]
    fn test_arity_mismatch() {
        let error = CarbonError::arity_mismatch(2, 3);
        assert!(matches!(error, CarbonError::ArityMismatch { .. }));
        assert_eq!(error.to_string(), "Arity mismatch: expected 2 arguments, got 3");
    }

    #[test]
    fn test_custom_error() {
        let error = CarbonError::custom_error("Custom error message");
        assert!(matches!(error, CarbonError::CustomError { .. }));
        assert_eq!(error.to_string(), "Custom error: Custom error message");
    }

    #[test]
    fn test_io_error() {
        let error = CarbonError::IoError { message: "File not found".to_string() };
        assert!(matches!(error, CarbonError::IoError { .. }));
        assert_eq!(error.to_string(), "IO error: File not found");
    }

    #[test]
    fn test_error_debug() {
        let error = CarbonError::runtime_error("Test error");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("RuntimeError"));
        assert!(debug_str.contains("Test error"));
    }

    #[test]
    fn test_error_clone() {
        let original = CarbonError::type_error("Original error");
        let cloned = original.clone();
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_result_type_alias() {
        fn test_function() -> Result<i32> {
            Ok(42)
        }

        fn test_error_function() -> Result<i32> {
            Err(CarbonError::runtime_error("Test"))
        }

        assert_eq!(test_function().unwrap(), 42);
        assert!(test_error_function().is_err());
    }

    #[test]
    fn test_error_source() {
        let error = CarbonError::runtime_error("Test error");
        // Test that the error implements std::error::Error
        assert!(std::error::Error::source(&error).is_none());
    }

    #[test]
    fn test_error_equality() {
        let error1 = CarbonError::runtime_error("Same message");
        let error2 = CarbonError::runtime_error("Same message");
        let error3 = CarbonError::runtime_error("Different message");

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_error_destructuring() {
        let error = CarbonError::LexerError {
            message: "Invalid token".to_string(),
            line: 5,
            column: 12,
        };

        if let CarbonError::LexerError { message, line, column } = error {
            assert_eq!(message, "Invalid token");
            assert_eq!(line, 5);
            assert_eq!(column, 12);
        } else {
            panic!("Wrong error type");
        }
    }

    #[test]
    fn test_helper_methods_return_correct_types() {
        let lexer_err = CarbonError::lexer_error("test", 1, 1);
        let parser_err = CarbonError::parser_error("test", 1, 1);
        let runtime_err = CarbonError::runtime_error("test");
        let type_err = CarbonError::type_error("test");
        let undef_var = CarbonError::undefined_variable("x");
        let undef_func = CarbonError::undefined_function("f");
        let undef_class = CarbonError::undefined_class("C");
        let invalid_op = CarbonError::invalid_operation("op", "type");
        let arity_err = CarbonError::arity_mismatch(1, 2);
        let custom_err = CarbonError::custom_error("custom");

        assert!(matches!(lexer_err, CarbonError::LexerError { .. }));
        assert!(matches!(parser_err, CarbonError::ParserError { .. }));
        assert!(matches!(runtime_err, CarbonError::RuntimeError { .. }));
        assert!(matches!(type_err, CarbonError::TypeError { .. }));
        assert!(matches!(undef_var, CarbonError::UndefinedVariable { .. }));
        assert!(matches!(undef_func, CarbonError::UndefinedFunction { .. }));
        assert!(matches!(undef_class, CarbonError::UndefinedClass { .. }));
        assert!(matches!(invalid_op, CarbonError::InvalidOperation { .. }));
        assert!(matches!(arity_err, CarbonError::ArityMismatch { .. }));
        assert!(matches!(custom_err, CarbonError::CustomError { .. }));
    }

    #[test]
    fn test_error_message_contents() {
        let error = CarbonError::LexerError {
            message: "Unexpected character '&'".to_string(),
            line: 10,
            column: 25,
        };

        let error_string = error.to_string();
        assert!(error_string.contains("Unexpected character '&'"));
        assert!(error_string.contains("10"));
        assert!(error_string.contains("25"));
    }

    #[test]
    fn test_comprehensive_error_chain() {
        fn level3() -> Result<i32> {
            Err(CarbonError::runtime_error("Level 3 error"))
        }

        fn level2() -> Result<i32> {
            level3()
        }

        fn level1() -> Result<i32> {
            level2()
        }

        match level1() {
            Ok(_) => panic!("Should have failed"),
            Err(e) => assert_eq!(e.to_string(), "Runtime error: Level 3 error"),
        }
    }
}
