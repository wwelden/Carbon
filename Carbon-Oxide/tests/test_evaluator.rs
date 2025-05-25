use carbon_interpreter::evaluator::*;
use carbon_interpreter::value::*;
use carbon_interpreter::ast::*;
use carbon_interpreter::error::*;
use std::collections::HashMap;

#[cfg(test)]
mod evaluator_tests {
    use super::*;

    fn create_evaluator_and_context() -> (Evaluator, Context) {
        let evaluator = Evaluator::new();
        let context = Context::new();
        (evaluator, context)
    }

    #[test]
    fn test_eval_integer_literal() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let expr = Expr::IntExpr(42);
        let result = eval.eval_expr(&mut context, expr).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_eval_real_literal() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let expr = Expr::RealExpr(3.14);
        let result = eval.eval_expr(&mut context, expr).unwrap();
        assert_eq!(result, Value::Real(3.14));
    }

    #[test]
    fn test_eval_boolean_literals() {
        let (mut eval, mut context) = create_evaluator_and_context();

        let true_expr = Expr::BoolExpr(true);
        let false_expr = Expr::BoolExpr(false);

        assert_eq!(eval.eval_expr(&mut context, true_expr).unwrap(), Value::Bool(true));
        assert_eq!(eval.eval_expr(&mut context, false_expr).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_eval_string_literal() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let expr = Expr::StringExpr("hello world".to_string());
        let result = eval.eval_expr(&mut context, expr).unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_eval_null_literal() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let expr = Expr::NullExpr;
        let result = eval.eval_expr(&mut context, expr).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_eval_math_constants() {
        let (mut eval, mut context) = create_evaluator_and_context();

        let pi_result = eval.eval_expr(&mut context, Expr::PiExpr).unwrap();
        let e_result = eval.eval_expr(&mut context, Expr::EExpr).unwrap();

        assert!(matches!(pi_result, Value::Real(f) if (f - std::f64::consts::PI).abs() < f64::EPSILON));
        assert!(matches!(e_result, Value::Real(f) if (f - std::f64::consts::E).abs() < f64::EPSILON));
    }

    #[test]
    fn test_eval_arithmetic_operations() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Addition
        let add_expr = Expr::BOpExpr(
            BOp::PlusOp,
            Box::new(Expr::IntExpr(2)),
            Box::new(Expr::IntExpr(3))
        );
        assert_eq!(eval.eval_expr(&mut context, add_expr).unwrap(), Value::Int(5));

        // Multiplication
        let mult_expr = Expr::BOpExpr(
            BOp::MultOp,
            Box::new(Expr::IntExpr(4)),
            Box::new(Expr::IntExpr(5))
        );
        assert_eq!(eval.eval_expr(&mut context, mult_expr).unwrap(), Value::Int(20));
    }

    #[test]
    fn test_eval_comparison_operations() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Equal
        let eq_expr = Expr::BOpExpr(
            BOp::EqOp,
            Box::new(Expr::IntExpr(5)),
            Box::new(Expr::IntExpr(5))
        );
        assert_eq!(eval.eval_expr(&mut context, eq_expr).unwrap(), Value::Bool(true));

        // Greater than
        let gt_expr = Expr::BOpExpr(
            BOp::GtOp,
            Box::new(Expr::IntExpr(5)),
            Box::new(Expr::IntExpr(3))
        );
        assert_eq!(eval.eval_expr(&mut context, gt_expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_variables() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Set a variable by inserting into context
        context.env.insert("x".to_string(), Value::Int(42));

        // Evaluate variable expression
        let var_expr = Expr::VarExpr("x".to_string());
        assert_eq!(eval.eval_expr(&mut context, var_expr).unwrap(), Value::Int(42));
    }

    #[test]
    fn test_eval_undefined_variable() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let var_expr = Expr::VarExpr("undefined_var".to_string());
        let result = eval.eval_expr(&mut context, var_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_array_literal() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let array_expr = Expr::ArrayExpr(vec![
            Expr::IntExpr(1),
            Expr::IntExpr(2),
            Expr::IntExpr(3)
        ]);

        let result = eval.eval_expr(&mut context, array_expr).unwrap();
        if let Value::Array(elements) = result {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Value::Int(1));
            assert_eq!(elements[1], Value::Int(2));
            assert_eq!(elements[2], Value::Int(3));
        } else {
            panic!("Expected array value");
        }
    }

    #[test]
    fn test_eval_let_statement() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let stmt = Statement::LetStmt("x".to_string(), Expr::IntExpr(42));

        eval.eval_statement(&mut context, stmt).unwrap();

        // Check that variable was set
        let var_expr = Expr::VarExpr("x".to_string());
        assert_eq!(eval.eval_expr(&mut context, var_expr).unwrap(), Value::Int(42));
    }

    #[test]
    fn test_eval_division_by_zero() {
        let (mut eval, mut context) = create_evaluator_and_context();
        let div_expr = Expr::BOpExpr(
            BOp::DivOp,
            Box::new(Expr::IntExpr(10)),
            Box::new(Expr::IntExpr(0))
        );

        let result = eval.eval_expr(&mut context, div_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_builtin_functions() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // toString function
        let tostring_expr = Expr::ToStringExpr(Box::new(Expr::IntExpr(42)));
        let result = eval.eval_expr(&mut context, tostring_expr).unwrap();
        assert_eq!(result, Value::String("42".to_string()));

        // typeof function
        let typeof_expr = Expr::TypeOfExpr(Box::new(Expr::IntExpr(42)));
        let result = eval.eval_expr(&mut context, typeof_expr).unwrap();
        assert_eq!(result, Value::String("number".to_string()));
    }

    #[test]
    fn test_eval_sum_expr_arrays() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum on integer array
        let array_expr = Expr::ArrayExpr(vec![
            Expr::IntExpr(1),
            Expr::IntExpr(2),
            Expr::IntExpr(3),
            Expr::IntExpr(4),
            Expr::IntExpr(5)
        ]);
        let sum_expr = Expr::SumExpr(Box::new(array_expr));
        let result = eval.eval_expr(&mut context, sum_expr).unwrap();
        assert_eq!(result, Value::Int(15));

        // Test sum on mixed number array
        let mixed_array_expr = Expr::ArrayExpr(vec![
            Expr::IntExpr(1),
            Expr::RealExpr(2.5),
            Expr::IntExpr(3),
            Expr::RealExpr(4.7)
        ]);
        let sum_mixed_expr = Expr::SumExpr(Box::new(mixed_array_expr));
        let result = eval.eval_expr(&mut context, sum_mixed_expr).unwrap();
        assert_eq!(result, Value::Real(11.2));

        // Test sum on empty array
        let empty_array_expr = Expr::ArrayExpr(vec![]);
        let sum_empty_expr = Expr::SumExpr(Box::new(empty_array_expr));
        let result = eval.eval_expr(&mut context, sum_empty_expr).unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_eval_sum_expr_arraylists() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum on ArrayList
        let arraylist_expr = Expr::ArrayListExpr(vec![
            Expr::IntExpr(10),
            Expr::IntExpr(20),
            Expr::IntExpr(30)
        ]);
        let sum_expr = Expr::SumExpr(Box::new(arraylist_expr));
        let result = eval.eval_expr(&mut context, sum_expr).unwrap();
        assert_eq!(result, Value::Int(60));
    }

    #[test]
    fn test_eval_sum_expr_sets() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum on Set (numeric strings)
        let set_expr = Expr::SetExpr(vec![
            Expr::IntExpr(1),
            Expr::IntExpr(2),
            Expr::IntExpr(3),
            Expr::IntExpr(4),
            Expr::IntExpr(5)
        ]);
        let sum_expr = Expr::SumExpr(Box::new(set_expr));
        let result = eval.eval_expr(&mut context, sum_expr).unwrap();
        assert_eq!(result, Value::Int(15));
    }

    #[test]
    fn test_eval_sum_expr_stacks() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum on Stack
        let stack_expr = Expr::StackExpr(vec![
            Expr::IntExpr(7),
            Expr::IntExpr(8),
            Expr::IntExpr(9)
        ]);
        let sum_expr = Expr::SumExpr(Box::new(stack_expr));
        let result = eval.eval_expr(&mut context, sum_expr).unwrap();
        assert_eq!(result, Value::Int(24));
    }

    #[test]
    fn test_eval_sum_expr_queues() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum on Queue
        let queue_expr = Expr::QueueExpr(vec![
            Expr::IntExpr(100),
            Expr::IntExpr(200),
            Expr::IntExpr(300)
        ]);
        let sum_expr = Expr::SumExpr(Box::new(queue_expr));
        let result = eval.eval_expr(&mut context, sum_expr).unwrap();
        assert_eq!(result, Value::Int(600));
    }

    #[test]
    fn test_eval_sum_expr_error_cases() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum on non-numeric array (should error)
        let string_array_expr = Expr::ArrayExpr(vec![
            Expr::StringExpr("hello".to_string()),
            Expr::StringExpr("world".to_string())
        ]);
        let sum_expr = Expr::SumExpr(Box::new(string_array_expr));
        let result = eval.eval_expr(&mut context, sum_expr);
        assert!(result.is_err());

        // Test sum on non-collection type (should error)
        let sum_int_expr = Expr::SumExpr(Box::new(Expr::IntExpr(42)));
        let result = eval.eval_expr(&mut context, sum_int_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_sum_builtin_function() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum() function call on array
        let array_value = Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
            Value::Int(4),
            Value::Int(5)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, array_value).unwrap();
        assert_eq!(result, Value::Int(15));

        // Test sum() function call on ArrayList
        let arraylist_value = Value::ArrayList(vec![
            Value::Int(10),
            Value::Int(20),
            Value::Int(30)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, arraylist_value).unwrap();
        assert_eq!(result, Value::Int(60));

        // Test sum() function call on Set
        let mut set = std::collections::HashSet::new();
        set.insert("1".to_string());
        set.insert("2".to_string());
        set.insert("3".to_string());
        let set_value = Value::Set(set);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, set_value).unwrap();
        assert_eq!(result, Value::Int(6));

        // Test sum() function call on Stack
        let stack_value = Value::Stack(vec![
            Value::Int(7),
            Value::Int(8),
            Value::Int(9)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, stack_value).unwrap();
        assert_eq!(result, Value::Int(24));

        // Test sum() function call on Queue
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Value::Int(100));
        queue.push_back(Value::Int(200));
        queue.push_back(Value::Int(300));
        let queue_value = Value::Queue(queue);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, queue_value).unwrap();
        assert_eq!(result, Value::Int(600));
    }

    #[test]
    fn test_eval_sum_mixed_types() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum with mixed integer and real numbers
        let mixed_array = Value::Array(vec![
            Value::Int(1),
            Value::Real(2.5),
            Value::Int(3),
            Value::Real(4.7)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, mixed_array).unwrap();
        assert_eq!(result, Value::Real(11.2));

        // Test sum with all real numbers
        let real_array = Value::Array(vec![
            Value::Real(1.1),
            Value::Real(2.2),
            Value::Real(3.3)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, real_array).unwrap();
        // Use approximate comparison for floating point
        if let Value::Real(sum) = result {
            assert!((sum - 6.6).abs() < f64::EPSILON);
        } else {
            panic!("Expected Real value");
        }
    }

    #[test]
    fn test_eval_sum_error_handling() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum() function call on unsupported type
        let string_value = Value::String("hello".to_string());
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, string_value);
        assert!(result.is_err());

        // Test sum on array with non-numeric values
        let mixed_array = Value::Array(vec![
            Value::Int(1),
            Value::String("hello".to_string()),
            Value::Int(3)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, mixed_array);
        assert!(result.is_err());

        // Test sum on set with non-numeric strings
        let mut set = std::collections::HashSet::new();
        set.insert("hello".to_string());
        set.insert("world".to_string());
        let set_value = Value::Set(set);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, set_value);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_sum_edge_cases() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test sum on empty collections
        let empty_array = Value::Array(vec![]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, empty_array).unwrap();
        assert_eq!(result, Value::Int(0));

        let empty_arraylist = Value::ArrayList(vec![]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, empty_arraylist).unwrap();
        assert_eq!(result, Value::Int(0));

        let empty_set = Value::Set(std::collections::HashSet::new());
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, empty_set).unwrap();
        assert_eq!(result, Value::Int(0));

        let empty_stack = Value::Stack(vec![]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, empty_stack).unwrap();
        assert_eq!(result, Value::Int(0));

        let empty_queue = Value::Queue(std::collections::VecDeque::new());
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, empty_queue).unwrap();
        assert_eq!(result, Value::Int(0));

        // Test sum with single element
        let single_array = Value::Array(vec![Value::Int(42)]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, single_array).unwrap();
        assert_eq!(result, Value::Int(42));

        // Test sum with negative numbers
        let negative_array = Value::Array(vec![
            Value::Int(-1),
            Value::Int(-2),
            Value::Int(-3)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, negative_array).unwrap();
        assert_eq!(result, Value::Int(-6));

        // Test sum with zero
        let zero_array = Value::Array(vec![
            Value::Int(0),
            Value::Int(0),
            Value::Int(0)
        ]);
        let sum_func = Value::BuiltinFunction("sum".to_string());
        let result = eval.apply_function(&mut context, sum_func, zero_array).unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_eval_multiple_return_expr() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test multiple return expression
        let multiple_return_expr = Expr::MultipleReturnExpr(vec![
            Expr::IntExpr(1),
            Expr::StringExpr("hello".to_string()),
            Expr::BoolExpr(true)
        ]);
        let result = eval.eval_expr(&mut context, multiple_return_expr).unwrap();

        if let Value::Tuple(values) = result {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], Value::Int(1));
            assert_eq!(values[1], Value::String("hello".to_string()));
            assert_eq!(values[2], Value::Bool(true));
        } else {
            panic!("Expected tuple value");
        }
    }

    #[test]
    fn test_eval_multiple_assignment_stmt() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Create a tuple value
        let tuple_value = Value::Tuple(vec![
            Value::Int(42),
            Value::String("world".to_string()),
            Value::Bool(false)
        ]);

        // Create a tuple expression that evaluates to the tuple value
        context.env.insert("test_tuple".to_string(), tuple_value.clone());
        let tuple_expr = Expr::VarExpr("test_tuple".to_string());

        // Test multiple assignment statement
        let multiple_assign_stmt = Statement::MultipleAssignStmt(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            tuple_expr
        );

        let result = eval.eval_statement(&mut context, multiple_assign_stmt).unwrap();
        assert_eq!(result, Some(tuple_value));

        // Check that variables were assigned correctly
        assert_eq!(context.env.get("a"), Some(&Value::Int(42)));
        assert_eq!(context.env.get("b"), Some(&Value::String("world".to_string())));
        assert_eq!(context.env.get("c"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_eval_multiple_assignment_length_mismatch() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Create a tuple with 2 values but try to assign to 3 variables
        let tuple_value = Value::Tuple(vec![
            Value::Int(1),
            Value::Int(2)
        ]);
        context.env.insert("test_tuple".to_string(), tuple_value);
        let tuple_expr = Expr::VarExpr("test_tuple".to_string());

        let multiple_assign_stmt = Statement::MultipleAssignStmt(
            vec!["x".to_string(), "y".to_string(), "z".to_string()],
            tuple_expr
        );

        let result = eval.eval_statement(&mut context, multiple_assign_stmt);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_multiple_assignment_non_tuple() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Try to assign a non-tuple value to multiple variables
        let multiple_assign_stmt = Statement::MultipleAssignStmt(
            vec!["x".to_string(), "y".to_string()],
            Expr::IntExpr(42)
        );

        let result = eval.eval_statement(&mut context, multiple_assign_stmt);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_function_with_multiple_return_types() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Create a function that returns multiple values
        let return_stmt = Statement::ReturnStmt(Expr::MultipleReturnExpr(vec![
            Expr::IntExpr(17),
            Expr::IntExpr(5)
        ]));

        let func_stmt = Statement::FnDeclStmt(
            "divmod".to_string(),
            vec![
                (Type::IntType, "a".to_string()),
                (Type::IntType, "b".to_string())
            ],
            Some(vec![Type::IntType, Type::IntType]), // Multiple return types
            vec![return_stmt.clone()],
            Expr::MultipleReturnExpr(vec![
                Expr::IntExpr(17),
                Expr::IntExpr(5)
            ])
        );

        // Declare the function
        eval.eval_statement(&mut context, func_stmt).unwrap();

        // Call the function with multiple arguments
        let func_call = Expr::ApplyExpr(
            Box::new(Expr::VarExpr("divmod".to_string())),
            Box::new(Expr::TupleExpr(vec![
                Expr::IntExpr(17),
                Expr::IntExpr(5)
            ]))
        );

        let result = eval.eval_expr(&mut context, func_call).unwrap();

        if let Value::Tuple(values) = result {
            assert_eq!(values.len(), 2);
            assert_eq!(values[0], Value::Int(17));
            assert_eq!(values[1], Value::Int(5));
        } else {
            panic!("Expected tuple value from function with multiple return types");
        }
    }

    #[test]
    fn test_eval_tuple_expr() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Test tuple expression
        let tuple_expr = Expr::TupleExpr(vec![
            Expr::IntExpr(1),
            Expr::StringExpr("test".to_string()),
            Expr::BoolExpr(true),
            Expr::RealExpr(3.14)
        ]);

        let result = eval.eval_expr(&mut context, tuple_expr).unwrap();

        if let Value::Tuple(values) = result {
            assert_eq!(values.len(), 4);
            assert_eq!(values[0], Value::Int(1));
            assert_eq!(values[1], Value::String("test".to_string()));
            assert_eq!(values[2], Value::Bool(true));
            assert_eq!(values[3], Value::Real(3.14));
        } else {
            panic!("Expected tuple value");
        }
    }

    #[test]
    fn test_eval_tuple_access_expr() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Create a tuple and test accessing its elements
        let tuple_value = Value::Tuple(vec![
            Value::String("first".to_string()),
            Value::Int(42),
            Value::Bool(true)
        ]);
        context.env.insert("my_tuple".to_string(), tuple_value);

        // Test accessing first element (index 0)
        let access_expr_0 = Expr::TupleAccessExpr(
            Box::new(Expr::VarExpr("my_tuple".to_string())),
            0
        );
        let result = eval.eval_expr(&mut context, access_expr_0).unwrap();
        assert_eq!(result, Value::String("first".to_string()));

        // Test accessing second element (index 1)
        let access_expr_1 = Expr::TupleAccessExpr(
            Box::new(Expr::VarExpr("my_tuple".to_string())),
            1
        );
        let result = eval.eval_expr(&mut context, access_expr_1).unwrap();
        assert_eq!(result, Value::Int(42));

        // Test accessing third element (index 2)
        let access_expr_2 = Expr::TupleAccessExpr(
            Box::new(Expr::VarExpr("my_tuple".to_string())),
            2
        );
        let result = eval.eval_expr(&mut context, access_expr_2).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_eval_tuple_access_out_of_bounds() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Create a tuple with 2 elements
        let tuple_value = Value::Tuple(vec![
            Value::Int(1),
            Value::Int(2)
        ]);
        context.env.insert("small_tuple".to_string(), tuple_value);

        // Test accessing out of bounds index
        let access_expr = Expr::TupleAccessExpr(
            Box::new(Expr::VarExpr("small_tuple".to_string())),
            5 // Out of bounds
        );
        let result = eval.eval_expr(&mut context, access_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_tuple_access_non_tuple() {
        let (mut eval, mut context) = create_evaluator_and_context();

        // Try to access tuple index on non-tuple value
        context.env.insert("not_tuple".to_string(), Value::Int(42));

        let access_expr = Expr::TupleAccessExpr(
            Box::new(Expr::VarExpr("not_tuple".to_string())),
            0
        );
        let result = eval.eval_expr(&mut context, access_expr);
        assert!(result.is_err());
    }
}
