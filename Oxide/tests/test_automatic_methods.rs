use carbon_interpreter::evaluator::*;
use carbon_interpreter::parser::*;
use carbon_interpreter::lexer::*;
use carbon_interpreter::value::*;
use carbon_interpreter::ast::*;

#[cfg(test)]
mod automatic_methods_tests {
    use super::*;

    fn parse_and_evaluate(code: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        let mut evaluator = Evaluator::new();
        let mut context = Context::new();

        let mut last_value = Value::Null;
        for stmt in statements {
            if let Some(value) = evaluator.eval_statement(&mut context, stmt)? {
                last_value = value;
            }
        }

        Ok(last_value)
    }

    fn create_class_and_object(class_code: &str, fields: &[&str]) -> (Evaluator, Context) {
        let code = format!("{}\nlet obj = new TestClass()", class_code);
        let mut lexer = Lexer::new(&code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let mut context = Context::new();

        for stmt in statements {
            evaluator.eval_statement(&mut context, stmt).unwrap();
        }

        (evaluator, context)
    }

        #[test]
    fn test_automatic_getters_generation() {
        let (mut evaluator, mut context) = create_class_and_object(
            "class TestClass { name age }",
            &["name", "age"]
        );

        // Test getName() method exists and returns null initially
        let get_name_code = "obj.getName()";
        let mut lexer = Lexer::new(get_name_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone()).unwrap();
        assert_eq!(result, Some(Value::Null));

        // Test getAge() method exists and returns null initially
        let get_age_code = "obj.getAge()";
        let mut lexer = Lexer::new(get_age_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone()).unwrap();
        assert_eq!(result, Some(Value::Null));
    }

        #[test]
    fn test_automatic_setters_generation() {
        let (mut evaluator, mut context) = create_class_and_object(
            "class TestClass { name age }",
            &["name", "age"]
        );

        // Test setName() method exists and returns the set value
        let set_name_code = "obj.setName(\"Alice\")";
        let mut lexer = Lexer::new(set_name_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone()).unwrap();
        assert_eq!(result, Some(Value::String("Alice".to_string())));

        // Test setAge() method exists and returns the set value
        let set_age_code = "obj.setAge(25)";
        let mut lexer = Lexer::new(set_age_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone()).unwrap();
        assert_eq!(result, Some(Value::Int(25)));
    }

    #[test]
    fn test_automatic_init_generation() {
                let (mut evaluator, mut context) = create_class_and_object(
            "class TestClass { value }",
            &["value"]
        );

        // Test init() method exists and returns the object
        let init_code = "obj.init(42)";
        let mut lexer = Lexer::new(init_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone()).unwrap();
        // Should return an object (the 'this' reference)
        assert!(matches!(result, Some(Value::Object { .. })));
    }

    #[test]
    fn test_multiple_fields_generate_all_methods() {
        let code = r#"
        class Car {
            make
            model
            year
            color
        }
        let car = new Car()
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let mut context = Context::new();

        for stmt in statements {
            evaluator.eval_statement(&mut context, stmt).unwrap();
        }

        // Test all getters exist
        let methods_to_test = [
            "car.getMake()", "car.getModel()", "car.getYear()", "car.getColor()",
            "car.setMake(\"Toyota\")", "car.setModel(\"Camry\")",
            "car.setYear(2023)", "car.setColor(\"Blue\")",
            "car.init(\"test\")"
        ];

        for method_call in &methods_to_test {
            let mut lexer = Lexer::new(method_call);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let statements = parser.parse().unwrap();

            let result = evaluator.eval_statement(&mut context, statements[0].clone());
            assert!(result.is_ok(), "Method call '{}' should work", method_call);
        }
    }

    #[test]
    fn test_manual_methods_override_automatic() {
                        let code = r#"
        class TestClass {
            value

            getValue(dummy) {
                return "manual method"
            }

            init(x) {
                return "custom init"
            }
        }
        let obj = new TestClass()
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let mut context = Context::new();

        for stmt in statements {
            evaluator.eval_statement(&mut context, stmt).unwrap();
        }

        // Test that manual getValue() method is used instead of automatic
        let get_value_code = "obj.getValue()";
        let mut lexer = Lexer::new(get_value_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone()).unwrap();
        assert_eq!(result, Some(Value::String("manual method".to_string())));

        // Test that manual init() method is used instead of automatic
        let init_code = "obj.init(42)";
        let mut lexer = Lexer::new(init_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone()).unwrap();
        assert_eq!(result, Some(Value::String("custom init".to_string())));

        // But automatic setValue() should still exist
        let set_value_code = "obj.setValue(100)";
        let mut lexer = Lexer::new(set_value_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone());
        assert!(result.is_ok());
    }

    #[test]
    fn test_zero_argument_methods_work() {
        let code = r#"
        class TestClass {
            name
        }
        let obj = new TestClass()
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let mut context = Context::new();

        for stmt in statements {
            evaluator.eval_statement(&mut context, stmt).unwrap();
        }

        // Test that zero-argument getter works
        let get_name_code = "obj.getName()";
        let mut lexer = Lexer::new(get_name_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone());
        assert!(result.is_ok(), "Zero-argument method calls should work");
    }

    #[test]
    fn test_empty_class_has_only_init() {
        let code = r#"
        class Empty {
        }
        let obj = new Empty()
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let mut context = Context::new();

        for stmt in statements {
            evaluator.eval_statement(&mut context, stmt).unwrap();
        }

        // Empty class should not have automatic getters/setters
        // since there are no fields, but should have init

        // This should fail since there are no fields to generate methods for
        let get_test_code = "obj.getName()";
        let mut lexer = Lexer::new(get_test_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let result = evaluator.eval_statement(&mut context, statements[0].clone());
        assert!(result.is_err(), "Empty class should not have field getters");
    }

    #[test]
    fn test_capitalization_in_method_names() {
        let code = r#"
        class TestClass {
            firstName
            lastName
        }
        let obj = new TestClass()
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let mut context = Context::new();

        for stmt in statements {
            evaluator.eval_statement(&mut context, stmt).unwrap();
        }

        // Test that method names are properly capitalized: getFirstName, setFirstName
        let methods_to_test = [
            "obj.getFirstName()",
            "obj.setFirstName(\"John\")",
            "obj.getLastName()",
            "obj.setLastName(\"Doe\")"
        ];

        for method_call in &methods_to_test {
            let mut lexer = Lexer::new(method_call);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let statements = parser.parse().unwrap();

            let result = evaluator.eval_statement(&mut context, statements[0].clone());
            assert!(result.is_ok(), "Method call '{}' should work with proper capitalization", method_call);
        }
    }
}
