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
}
