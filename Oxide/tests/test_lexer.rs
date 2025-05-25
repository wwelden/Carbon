use carbon_interpreter::lexer::*;

#[cfg(test)]
mod lexer_tests {
    use super::*;

    fn tokenize_string(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        lexer.tokenize().unwrap()
    }

    #[test]
    fn test_empty_input() {
        let tokens = tokenize_string("");
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_whitespace_only() {
        let tokens = tokenize_string("   \t\n\r  ");
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_integer_literals() {
        let tokens = tokenize_string("42 0 123");
        assert_eq!(tokens, vec![
            Token::Int(42),
            Token::Int(0),
            Token::Int(123),
            Token::Eof
        ]);
    }

    #[test]
    fn test_real_literals() {
        let tokens = tokenize_string("3.14 0.5 123.456");
        assert_eq!(tokens, vec![
            Token::Real(3.14),
            Token::Real(0.5),
            Token::Real(123.456),
            Token::Eof
        ]);
    }

    #[test]
    fn test_string_literals() {
        let tokens = tokenize_string(r#""hello" "world" "test string""#);
        assert_eq!(tokens, vec![
            Token::String("hello".to_string()),
            Token::String("world".to_string()),
            Token::String("test string".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_string_with_escapes() {
        let tokens = tokenize_string(r#""hello\nworld" "test\"quote""#);
        assert_eq!(tokens, vec![
            Token::String("hello\nworld".to_string()),
            Token::String("test\"quote".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_boolean_literals() {
        let tokens = tokenize_string("true false");
        assert_eq!(tokens, vec![
            Token::Bool(true),
            Token::Bool(false),
            Token::Eof
        ]);
    }

    #[test]
    fn test_null_literal() {
        let tokens = tokenize_string("null");
        assert_eq!(tokens, vec![Token::Null, Token::Eof]);
    }

    #[test]
    fn test_math_constants() {
        let tokens = tokenize_string("E PI PHI AVOGADRO");
        assert_eq!(tokens, vec![
            Token::E,
            Token::Pi,
            Token::Phi,
            Token::Avogadro,
            Token::Eof
        ]);
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize_string("variable_name x y test123");
        assert_eq!(tokens, vec![
            Token::Identifier("variable_name".to_string()),
            Token::Identifier("x".to_string()),
            Token::Identifier("y".to_string()),
            Token::Identifier("test123".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_keywords() {
        let tokens = tokenize_string("let const var if else for while in function fn return class new this match");
        assert_eq!(tokens, vec![
            Token::Let,
            Token::Const,
            Token::Var,
            Token::If,
            Token::Else,
            Token::For,
            Token::While,
            Token::In,
            Token::Function,
            Token::Fn,
            Token::Return,
            Token::Class,
            Token::New,
            Token::This,
            Token::Match,
            Token::Eof
        ]);
    }

    #[test]
    fn test_builtin_functions() {
        let tokens = tokenize_string("print toString typeof Math.sqrt err isErr");
        assert_eq!(tokens, vec![
            Token::Print,
            Token::ToString,
            Token::TypeOf,
            Token::MathSqrt,
            Token::Err,
            Token::IsErr,
            Token::Eof
        ]);
    }

    #[test]
    fn test_type_keywords() {
        let tokens = tokenize_string("int bool string ArrayList Set Map Stack Queue LinkedList");
        assert_eq!(tokens, vec![
            Token::IntType,
            Token::BoolType,
            Token::StringType,
            Token::ArrayListType,
            Token::SetType,
            Token::MapType,
            Token::StackType,
            Token::QueueType,
            Token::LinkedListType,
            Token::Eof
        ]);
    }

    #[test]
    fn test_arithmetic_operators() {
        let tokens = tokenize_string("+ - * / % **");
        assert_eq!(tokens, vec![
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
            Token::Modulo,
            Token::Exponent,
            Token::Eof
        ]);
    }

    #[test]
    fn test_compound_assignment() {
        let tokens = tokenize_string("+= -= *= /= %=");
        assert_eq!(tokens, vec![
            Token::PlusAssign,
            Token::MinusAssign,
            Token::MultiplyAssign,
            Token::DivideAssign,
            Token::ModuloAssign,
            Token::Eof
        ]);
    }

    #[test]
    fn test_increment_decrement() {
        let tokens = tokenize_string("++ --");
        assert_eq!(tokens, vec![
            Token::Increment,
            Token::Decrement,
            Token::Eof
        ]);
    }

    #[test]
    fn test_comparison_operators() {
        let tokens = tokenize_string("== != > < >= <=");
        assert_eq!(tokens, vec![
            Token::Equal,
            Token::NotEqual,
            Token::Greater,
            Token::Less,
            Token::GreaterEqual,
            Token::LessEqual,
            Token::Eof
        ]);
    }

    #[test]
    fn test_logical_operators() {
        let tokens = tokenize_string("&& || !");
        assert_eq!(tokens, vec![
            Token::And,
            Token::Or,
            Token::Not,
            Token::Eof
        ]);
    }

    #[test]
    fn test_assignment() {
        let tokens = tokenize_string("=");
        assert_eq!(tokens, vec![Token::Assign, Token::Eof]);
    }

    #[test]
    fn test_punctuation() {
        let tokens = tokenize_string("( ) { } [ ] ; , . => ? : | _");
        assert_eq!(tokens, vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Semicolon,
            Token::Comma,
            Token::Dot,
            Token::Arrow,
            Token::Question,
            Token::Colon,
            Token::BitOr,
            Token::Wildcard,
            Token::Eof
        ]);
    }

    #[test]
    fn test_comments() {
        let tokens = tokenize_string("42 // this is a comment\n43");
        assert_eq!(tokens, vec![
            Token::Int(42),
            Token::Int(43),
            Token::Eof
        ]);
    }

    #[test]
    fn test_complex_expression() {
        let tokens = tokenize_string("let x = (a + b) * c;");
        assert_eq!(tokens, vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::LeftParen,
            Token::Identifier("a".to_string()),
            Token::Plus,
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::Multiply,
            Token::Identifier("c".to_string()),
            Token::Semicolon,
            Token::Eof
        ]);
    }

    #[test]
    fn test_function_definition() {
        let tokens = tokenize_string("fn add(x, y) { return x + y; }");
        assert_eq!(tokens, vec![
            Token::Fn,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Eof
        ]);
    }

    #[test]
    fn test_class_definition() {
        let tokens = tokenize_string("class MyClass { }");
        assert_eq!(tokens, vec![
            Token::Class,
            Token::ClassName("MyClass".to_string()),
            Token::LeftBrace,
            Token::RightBrace,
            Token::Eof
        ]);
    }

    #[test]
    fn test_match_expression() {
        let tokens = tokenize_string("match x { 1 => true, _ => false }");
        assert_eq!(tokens, vec![
            Token::Match,
            Token::Identifier("x".to_string()),
            Token::LeftBrace,
            Token::Int(1),
            Token::Arrow,
            Token::Bool(true),
            Token::Comma,
            Token::Wildcard,
            Token::Arrow,
            Token::Bool(false),
            Token::RightBrace,
            Token::Eof
        ]);
    }

    #[test]
    fn test_tuple_syntax() {
        let tokens = tokenize_string("(1, 2, 3)");
        assert_eq!(tokens, vec![
            Token::LeftParen,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::Comma,
            Token::Int(3),
            Token::RightParen,
            Token::Eof
        ]);
    }

    #[test]
    fn test_array_syntax() {
        let tokens = tokenize_string("[1, 2, 3]");
        assert_eq!(tokens, vec![
            Token::LeftBracket,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::Comma,
            Token::Int(3),
            Token::RightBracket,
            Token::Eof
        ]);
    }

    #[test]
    fn test_data_structure_operations() {
        let tokens = tokenize_string("list.push(1); stack.pop();");
        assert_eq!(tokens, vec![
            Token::Identifier("list".to_string()),
            Token::Dot,
            Token::Identifier("push".to_string()),
            Token::LeftParen,
            Token::Int(1),
            Token::RightParen,
            Token::Semicolon,
            Token::Identifier("stack".to_string()),
            Token::Dot,
            Token::Identifier("pop".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::Semicolon,
            Token::Eof
        ]);
    }

    #[test]
    fn test_ternary_operator() {
        let tokens = tokenize_string("condition ? true_value : false_value");
        assert_eq!(tokens, vec![
            Token::Identifier("condition".to_string()),
            Token::Question,
            Token::Identifier("true_value".to_string()),
            Token::Colon,
            Token::Identifier("false_value".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_pattern_matching_syntax() {
        let tokens = tokenize_string("| x if x > 0 => \"positive\"");
        assert_eq!(tokens, vec![
            Token::BitOr,
            Token::Identifier("x".to_string()),
            Token::If,
            Token::Identifier("x".to_string()),
            Token::Greater,
            Token::Int(0),
            Token::Arrow,
            Token::String("positive".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_error_handling() {
        let mut lexer = Lexer::new("@");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"unterminated");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_token_display() {
        assert_eq!(Token::Int(42).to_string(), "42");
        assert_eq!(Token::Real(3.14).to_string(), "3.14");
        assert_eq!(Token::String("hello".to_string()).to_string(), "\"hello\"");
        assert_eq!(Token::Bool(true).to_string(), "true");
        assert_eq!(Token::Null.to_string(), "null");
        assert_eq!(Token::Plus.to_string(), "+");
        assert_eq!(Token::Identifier("var".to_string()).to_string(), "var");
        assert_eq!(Token::Let.to_string(), "Let");
        assert_eq!(Token::LeftParen.to_string(), "LeftParen");
        assert_eq!(Token::Eof.to_string(), "Eof");
    }

    #[test]
    fn test_token_equality() {
        assert_eq!(Token::Int(42), Token::Int(42));
        assert_ne!(Token::Int(42), Token::Int(43));
        assert_eq!(Token::String("test".to_string()), Token::String("test".to_string()));
        assert_eq!(Token::Plus, Token::Plus);
        assert_ne!(Token::Plus, Token::Minus);
    }

    #[test]
    fn test_token_cloning() {
        let original = Token::String("test".to_string());
        let cloned = original.clone();
        assert_eq!(original, cloned);

        let complex_token = Token::Identifier("complex_name".to_string());
        let cloned_complex = complex_token.clone();
        assert_eq!(complex_token, cloned_complex);
    }

    #[test]
    fn test_multiline_input() {
        let input = r#"
let x = 42;
let y = "hello";
print(x + y);
"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Should successfully tokenize multiline input
        assert!(tokens.len() > 10);
        assert_eq!(tokens.last(), Some(&Token::Eof));
    }

    #[test]
    fn test_edge_cases() {
        // Test single characters that might be confused
        let tokens = tokenize_string("+ ++ += - -- -= = == ! !=");
        assert!(tokens.contains(&Token::Plus));
        assert!(tokens.contains(&Token::Increment));
        assert!(tokens.contains(&Token::PlusAssign));
        assert!(tokens.contains(&Token::Minus));
        assert!(tokens.contains(&Token::Decrement));
        assert!(tokens.contains(&Token::MinusAssign));
        assert!(tokens.contains(&Token::Assign));
        assert!(tokens.contains(&Token::Equal));
        assert!(tokens.contains(&Token::Not));
        assert!(tokens.contains(&Token::NotEqual));
    }

    #[test]
    fn test_numbers_at_boundaries() {
        let tokens = tokenize_string("0 1 123 0.0 1.0 999.999");
        assert_eq!(tokens, vec![
            Token::Int(0),
            Token::Int(1),
            Token::Int(123),
            Token::Real(0.0),
            Token::Real(1.0),
            Token::Real(999.999),
            Token::Eof
        ]);
    }

    #[test]
    fn test_mixed_content() {
        let input = r#"
// A simple Carbon program
let name = "Carbon";
let version = 1.0;
let active = true;

fn greet(name) {
    print("Hello, " + name + "!");
}

greet(name);
"#;
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert!(tokens.len() > 30); // Should have many tokens
        assert_eq!(tokens.last(), Some(&Token::Eof));
    }
}
