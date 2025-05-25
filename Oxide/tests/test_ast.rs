use carbon_interpreter::ast::*;

#[cfg(test)]
mod ast_tests {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::IntType.to_string(), "int");
        assert_eq!(Type::BoolType.to_string(), "bool");
        assert_eq!(Type::StringType.to_string(), "string");
        assert_eq!(Type::ArrayType(Box::new(Type::IntType)).to_string(), "int[]");
        assert_eq!(Type::ArrayListType(Box::new(Type::StringType)).to_string(), "ArrayList<string>");
        assert_eq!(Type::TupleType(vec![Type::IntType, Type::BoolType]).to_string(), "(int, bool)");
        assert_eq!(Type::SetType(Box::new(Type::IntType)).to_string(), "Set<int>");
        assert_eq!(Type::MapType(Box::new(Type::StringType), Box::new(Type::IntType)).to_string(), "Map<string, int>");
        assert_eq!(Type::StackType(Box::new(Type::IntType)).to_string(), "Stack<int>");
        assert_eq!(Type::QueueType(Box::new(Type::BoolType)).to_string(), "Queue<bool>");
        assert_eq!(Type::LinkedListType(Box::new(Type::StringType)).to_string(), "LinkedList<string>");
    }

    #[test]
    fn test_bop_display() {
        assert_eq!(BOp::PlusOp.to_string(), "+");
        assert_eq!(BOp::MinusOp.to_string(), "-");
        assert_eq!(BOp::MultOp.to_string(), "*");
        assert_eq!(BOp::DivOp.to_string(), "/");
        assert_eq!(BOp::ModOp.to_string(), "%");
        assert_eq!(BOp::ExpOp.to_string(), "**");
        assert_eq!(BOp::EqOp.to_string(), "==");
        assert_eq!(BOp::NeqOp.to_string(), "!=");
        assert_eq!(BOp::GtOp.to_string(), ">");
        assert_eq!(BOp::LtOp.to_string(), "<");
        assert_eq!(BOp::LeqOp.to_string(), "<=");
        assert_eq!(BOp::GeqOp.to_string(), ">=");
        assert_eq!(BOp::AndOp.to_string(), "&&");
        assert_eq!(BOp::OrOp.to_string(), "||");
    }

    #[test]
    fn test_compound_op_display() {
        assert_eq!(CompoundOp::PlusEq.to_string(), "+=");
        assert_eq!(CompoundOp::MinusEq.to_string(), "-=");
        assert_eq!(CompoundOp::MultEq.to_string(), "*=");
        assert_eq!(CompoundOp::DivEq.to_string(), "/=");
        assert_eq!(CompoundOp::ModEq.to_string(), "%=");
    }

    #[test]
    fn test_literal_display() {
        assert_eq!(Literal::IntLit(42).to_string(), "42");
        assert_eq!(Literal::RealLit(3.14).to_string(), "3.14");
        assert_eq!(Literal::BoolLit(true).to_string(), "true");
        assert_eq!(Literal::BoolLit(false).to_string(), "false");
        assert_eq!(Literal::StringLit("hello".to_string()).to_string(), "\"hello\"");
        assert_eq!(Literal::NullLit.to_string(), "null");
    }

    #[test]
    fn test_math_constants() {
        let e_expr = Expr::EExpr;
        let pi_expr = Expr::PiExpr;
        let phi_expr = Expr::PhiExpr;
        let avogadro_expr = Expr::AvogadroExpr;

        assert!(e_expr.math_constant_value().is_some());
        assert!(pi_expr.math_constant_value().is_some());
        assert!(phi_expr.math_constant_value().is_some());
        assert!(avogadro_expr.math_constant_value().is_some());

        assert!((e_expr.math_constant_value().unwrap() - std::f64::consts::E).abs() < f64::EPSILON);
        assert!((pi_expr.math_constant_value().unwrap() - std::f64::consts::PI).abs() < f64::EPSILON);

        // Test non-math constants return None
        let int_expr = Expr::IntExpr(42);
        assert!(int_expr.math_constant_value().is_none());
    }

    #[test]
    fn test_expression_variants() {
        // Test all expression variants can be created
        let int_expr = Expr::IntExpr(42);
        let real_expr = Expr::RealExpr(3.14);
        let bool_expr = Expr::BoolExpr(true);
        let string_expr = Expr::StringExpr("test".to_string());
        let var_expr = Expr::VarExpr("x".to_string());
        let null_expr = Expr::NullExpr;

        assert!(matches!(int_expr, Expr::IntExpr(42)));
        assert!(matches!(real_expr, Expr::RealExpr(f) if f == 3.14));
        assert!(matches!(bool_expr, Expr::BoolExpr(true)));
        assert!(matches!(string_expr, Expr::StringExpr(s) if s == "test"));
        assert!(matches!(var_expr, Expr::VarExpr(v) if v == "x"));
        assert!(matches!(null_expr, Expr::NullExpr));
    }

    #[test]
    fn test_pattern_variants() {
        let lit_pat = Pattern::LitPat(Literal::IntLit(42));
        let var_pat = Pattern::VarPat("x".to_string());
        let wildcard_pat = Pattern::WildcardPat;
        let array_pat = Pattern::ArrayPat(vec![Pattern::VarPat("a".to_string())]);
        let range_pat = Pattern::RangePat(1, 10);
        let tuple_pat = Pattern::TuplePat(vec![Pattern::VarPat("x".to_string()), Pattern::VarPat("y".to_string())]);

        assert!(matches!(lit_pat, Pattern::LitPat(Literal::IntLit(42))));
        assert!(matches!(var_pat, Pattern::VarPat(v) if v == "x"));
        assert!(matches!(wildcard_pat, Pattern::WildcardPat));
        assert!(matches!(array_pat, Pattern::ArrayPat(_)));
        assert!(matches!(range_pat, Pattern::RangePat(1, 10)));
        assert!(matches!(tuple_pat, Pattern::TuplePat(_)));
    }

    #[test]
    fn test_statement_variants() {
        let expr_stmt = Statement::ExprStmt(Expr::IntExpr(42));
        let let_stmt = Statement::LetStmt("x".to_string(), Expr::IntExpr(42));
        let const_stmt = Statement::ConstStmt("PI".to_string(), Expr::RealExpr(3.14));
        let var_stmt = Statement::VarStmt("y".to_string(), Expr::BoolExpr(true));

        assert!(matches!(expr_stmt, Statement::ExprStmt(Expr::IntExpr(42))));
        assert!(matches!(let_stmt, Statement::LetStmt(_, _)));
        assert!(matches!(const_stmt, Statement::ConstStmt(_, _)));
        assert!(matches!(var_stmt, Statement::VarStmt(_, _)));
    }

    #[test]
    fn test_type_equality() {
        let int1 = Type::IntType;
        let int2 = Type::IntType;
        let bool_type = Type::BoolType;

        assert_eq!(int1, int2);
        assert_ne!(int1, bool_type);

        let array1 = Type::ArrayType(Box::new(Type::IntType));
        let array2 = Type::ArrayType(Box::new(Type::IntType));
        let array3 = Type::ArrayType(Box::new(Type::BoolType));

        assert_eq!(array1, array2);
        assert_ne!(array1, array3);
    }

    #[test]
    fn test_nested_tuple_types() {
        let nested_tuple = Type::TupleType(vec![
            Type::IntType,
            Type::TupleType(vec![Type::StringType, Type::BoolType]),
            Type::ArrayType(Box::new(Type::IntType))
        ]);

        assert_eq!(nested_tuple.to_string(), "(int, (string, bool), int[])");
    }

    #[test]
    fn test_complex_expressions() {
        // Test binary operation expression
        let binop = Expr::BOpExpr(
            BOp::PlusOp,
            Box::new(Expr::IntExpr(1)),
            Box::new(Expr::IntExpr(2))
        );

        // Test nested expression
        let nested = Expr::BOpExpr(
            BOp::MultOp,
            Box::new(binop),
            Box::new(Expr::IntExpr(3))
        );

        assert!(matches!(nested, Expr::BOpExpr(BOp::MultOp, _, _)));
    }

    #[test]
    fn test_tuple_expressions() {
        let tuple_expr = Expr::TupleExpr(vec![
            Expr::IntExpr(1),
            Expr::StringExpr("hello".to_string()),
            Expr::BoolExpr(true)
        ]);

        let tuple_access = Expr::TupleAccessExpr(Box::new(tuple_expr), 0);

        assert!(matches!(tuple_access, Expr::TupleAccessExpr(_, 0)));
    }

    #[test]
    fn test_data_structure_expressions() {
        let arraylist = Expr::ArrayListExpr(vec![Expr::IntExpr(1), Expr::IntExpr(2)]);
        let set_expr = Expr::SetExpr(vec![Expr::StringExpr("a".to_string())]);
        let map_expr = Expr::MapExpr(vec![
            (Expr::StringExpr("key".to_string()), Expr::IntExpr(42))
        ]);
        let stack_expr = Expr::StackExpr(vec![Expr::IntExpr(1)]);
        let queue_expr = Expr::QueueExpr(vec![Expr::IntExpr(1)]);
        let linked_list_expr = Expr::LinkedListExpr(vec![Expr::IntExpr(1)]);

        assert!(matches!(arraylist, Expr::ArrayListExpr(_)));
        assert!(matches!(set_expr, Expr::SetExpr(_)));
        assert!(matches!(map_expr, Expr::MapExpr(_)));
        assert!(matches!(stack_expr, Expr::StackExpr(_)));
        assert!(matches!(queue_expr, Expr::QueueExpr(_)));
        assert!(matches!(linked_list_expr, Expr::LinkedListExpr(_)));
    }

    #[test]
    fn test_pattern_cloning() {
        let original = Pattern::GuardPat(
            Box::new(Pattern::VarPat("x".to_string())),
            Box::new(Expr::BOpExpr(
                BOp::GtOp,
                Box::new(Expr::VarExpr("x".to_string())),
                Box::new(Expr::IntExpr(0))
            ))
        );

        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_class_members() {
        let field = ClassMember::FieldDecl("name".to_string());
        let method = ClassMember::MethodDecl(
            "getName".to_string(),
            "self".to_string(),
            Expr::VarExpr("name".to_string())
        );

        assert!(matches!(field, ClassMember::FieldDecl(_)));
        assert!(matches!(method, ClassMember::MethodDecl(_, _, _)));
    }
}
