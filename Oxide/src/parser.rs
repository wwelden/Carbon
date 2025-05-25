use crate::ast::*;
use crate::error::{CarbonError, Result};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    column: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.check(&Token::Eof) {
                break;
            }
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement> {
        match &self.peek() {
            Token::Let => self.let_statement(),
            Token::Const => self.const_statement(),
            Token::Var => self.var_statement(),
            Token::IntType | Token::BoolType | Token::StringType => self.typed_var_statement(),
            Token::Fn => self.fn_declaration(),
            Token::Class => self.class_statement(),
            Token::For => self.for_statement(),
            Token::Do => self.do_while_statement(),
            Token::Return => self.return_statement(),
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                match &self.peek() {
                    Token::PlusAssign | Token::MinusAssign | Token::MultiplyAssign
                    | Token::DivideAssign | Token::ModuloAssign => {
                        let token = self.advance().clone();
                        let op = self.compound_op_from_token(&token)?;
                        let expr = self.expression()?;
                        self.consume_semicolon()?;
                        Ok(Statement::CompoundAssignStmt(name, op, expr))
                    }
                    Token::Increment => {
                        self.advance();
                        self.consume_semicolon()?;
                        Ok(Statement::IncrementStmt(name))
                    }
                    Token::Decrement => {
                        self.advance();
                        self.consume_semicolon()?;
                        Ok(Statement::DecrementStmt(name))
                    }
                    Token::Assign => {
                        self.advance();
                        let expr = self.expression()?;
                        self.consume_semicolon()?;
                        Ok(Statement::ExprStmt(Expr::BOpExpr(BOp::EqOp, Box::new(Expr::VarExpr(name)), Box::new(expr))))
                    }
                    _ => {
                        // It's an expression statement, backtrack
                        self.current -= 1;
                        let expr = self.expression()?;
                        self.consume_semicolon()?;
                        Ok(Statement::ExprStmt(expr))
                    }
                }
            }
            Token::Multiply => {
                // Check for pointer assignment: *ptr = value
                self.advance(); // consume '*'
                let ptr_expr = self.unary()?;
                if self.match_token(&Token::Assign) {
                    let value_expr = self.expression()?;
                    self.consume_semicolon()?;
                    Ok(Statement::PointerAssignStmt(ptr_expr, value_expr))
                } else {
                    // It's a dereference expression, backtrack
                    self.current -= 1;
                    let expr = self.expression()?;
                    self.consume_semicolon()?;
                    Ok(Statement::ExprStmt(expr))
                }
            }
            _ => {
                let expr = self.expression()?;
                self.consume_semicolon()?;
                Ok(Statement::ExprStmt(expr))
            }
        }
    }

    fn let_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Let)?;

        // Check if this is a typed let statement: let int x = 42
        if matches!(self.peek(), Token::IntType | Token::BoolType | Token::StringType) {
            let var_type = self.parse_type()?;
            let name = self.consume_identifier()?;
            self.consume(&Token::Assign)?;
            let expr = self.expression()?;
            self.consume_semicolon()?;
            Ok(Statement::TypedLetStmt(var_type, name, expr))
        } else {
            // Regular let statement: let x = 42
            let name = self.consume_identifier()?;
            self.consume(&Token::Assign)?;
            let expr = self.expression()?;
            self.consume_semicolon()?;
            Ok(Statement::LetStmt(name, expr))
        }
    }

    fn const_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Const)?;
        let name = self.consume_identifier()?;
        self.consume(&Token::Assign)?;
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::ConstStmt(name, expr))
    }

    fn var_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Var)?;
        let name = self.consume_identifier()?;
        self.consume(&Token::Assign)?;
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::VarStmt(name, expr))
    }

    fn typed_var_statement(&mut self) -> Result<Statement> {
        let var_type = self.parse_type()?;
        let name = self.consume_identifier()?;
        self.consume(&Token::Assign)?;
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::TypedVarStmt(var_type, name, expr))
    }

    fn fn_declaration(&mut self) -> Result<Statement> {
        self.consume(&Token::Fn)?;
        let name = self.consume_identifier()?;
        self.consume(&Token::LeftParen)?;
        let params = self.parameter_list()?;
        self.consume(&Token::RightParen)?;

        // Check for optional return type annotation
        let return_type = if self.match_token(&Token::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(&Token::LeftBrace)?;
        let statements = self.statement_list()?;

        // Extract return expression from last statement or default to null
        let return_expr = if let Some(last_stmt) = statements.last() {
            match last_stmt {
                Statement::ReturnStmt(expr) => expr.clone(),
                Statement::ExprStmt(expr) => expr.clone(),
                _ => Expr::NullExpr,
            }
        } else {
            Expr::NullExpr
        };

        self.consume(&Token::RightBrace)?;

        Ok(Statement::FnDeclStmt(name, params, return_type, statements, return_expr))
    }

    fn class_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Class)?;
        let name = self.consume_class_name()?;

        // Check for inheritance (extends keyword)
        let parent = if self.match_identifier("extends") {
            Some(self.consume_class_name()?)
        } else {
            None
        };

        self.consume(&Token::LeftBrace)?;
        let members = self.class_members()?;
        self.consume(&Token::RightBrace)?;
        Ok(Statement::ClassStmt(name, parent, members))
    }

    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::For)?;

        // Check if this is a for-in loop or a while-like loop
        if self.peek_for_in() {
            // for var in expr { ... }
            let var = self.consume_identifier()?;
            self.consume(&Token::In)?;
            let expr = self.expression()?;
            self.consume(&Token::LeftBrace)?;
            let statements = self.statement_list()?;
            self.consume(&Token::RightBrace)?;
            Ok(Statement::ForInStmt(var, expr, statements))
        } else {
            // for condition { ... } (while-like loop)
            let condition = self.expression()?;
            self.consume(&Token::LeftBrace)?;
            let statements = self.statement_list()?;
            self.consume(&Token::RightBrace)?;
            Ok(Statement::WhileStmt(condition, statements))
        }
    }

    fn peek_for_in(&self) -> bool {
        // Look ahead to see if this is "identifier in" pattern
        if let Token::Identifier(_) = self.peek() {
            if self.current + 1 < self.tokens.len() {
                matches!(self.tokens[self.current + 1], Token::In)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn do_while_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Do)?;
        self.consume(&Token::LeftBrace)?;
        let statements = self.statement_list()?;
        self.consume(&Token::RightBrace)?;
        self.consume(&Token::While)?;
        self.consume(&Token::LeftParen)?;
        let condition = self.expression()?;
        self.consume(&Token::RightParen)?;
        self.consume_semicolon()?;
        Ok(Statement::DoWhileStmt(statements, condition))
    }

    fn return_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Return)?;
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::ReturnStmt(expr))
    }

    fn parameter_list(&mut self) -> Result<Vec<(Type, Var)>> {
        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                let param_type = self.parse_type()?;
                let param_name = self.consume_identifier()?;
                params.push((param_type, param_name));

                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        Ok(params)
    }

    fn statement_list(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn class_members(&mut self) -> Result<Vec<ClassMember>> {
        let mut members = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let name = self.consume_identifier()?;

            if self.match_token(&Token::LeftParen) {
                // Method declaration
                let param = if self.check(&Token::RightParen) {
                    "".to_string() // No parameter
                } else {
                    self.consume_identifier()?
                };
                self.consume(&Token::RightParen)?;
                self.consume(&Token::LeftBrace)?;
                self.consume(&Token::Return)?;
                let expr = self.expression()?;
                self.consume(&Token::RightBrace)?;
                members.push(ClassMember::MethodDecl(name, param, expr));
            } else {
                // Field declaration
                self.consume_semicolon()?;
                members.push(ClassMember::FieldDecl(name));
            }
        }

        Ok(members)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expr> {
        let expr = self.or()?;

        if self.match_token(&Token::Question) {
            let then_expr = self.expression()?;
            self.consume(&Token::Colon)?;
            let else_expr = self.expression()?;
            Ok(Expr::TernaryExpr(Box::new(expr), Box::new(then_expr), Box::new(else_expr)))
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.match_token(&Token::Or) {
            let right = self.and()?;
            expr = Expr::BOpExpr(BOp::OrOp, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.bitwise_or()?;

        while self.match_token(&Token::And) {
            let right = self.bitwise_or()?;
            expr = Expr::BOpExpr(BOp::AndOp, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn bitwise_or(&mut self) -> Result<Expr> {
        let mut expr = self.bitwise_xor()?;

        while self.match_token(&Token::BitOr) {
            let right = self.bitwise_xor()?;
            expr = Expr::BOpExpr(BOp::BitOrOp, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn bitwise_xor(&mut self) -> Result<Expr> {
        let mut expr = self.bitwise_and()?;

        while self.match_token(&Token::BitXor) {
            let right = self.bitwise_and()?;
            expr = Expr::BOpExpr(BOp::BitXorOp, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn bitwise_and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(&Token::BitAnd) {
            let right = self.equality()?;
            expr = Expr::BOpExpr(BOp::BitAndOp, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.match_equality_op() {
            let right = self.comparison()?;
            expr = Expr::BOpExpr(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.bitwise_shift()?;

        while let Some(op) = self.match_comparison_op() {
            let right = self.bitwise_shift()?;
            expr = Expr::BOpExpr(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn bitwise_shift(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while let Some(op) = self.match_shift_op() {
            let right = self.term()?;
            expr = Expr::BOpExpr(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while let Some(op) = self.match_term_op() {
            let right = self.factor()?;
            // Check if this is pointer arithmetic (pointer + integer)
            if matches!(op, BOp::PlusOp) {
                // We'll handle pointer arithmetic detection in the evaluator
                expr = Expr::BOpExpr(op, Box::new(expr), Box::new(right));
            } else {
                expr = Expr::BOpExpr(op, Box::new(expr), Box::new(right));
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.exponent()?;

        while let Some(op) = self.match_factor_op() {
            let right = self.exponent()?;
            expr = Expr::BOpExpr(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn exponent(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        if self.match_token(&Token::Exponent) {
            let right = self.exponent()?; // Right associative
            expr = Expr::BOpExpr(BOp::ExpOp, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_token(&Token::Not) {
            let expr = self.unary()?;
            Ok(Expr::NotExpr(Box::new(expr)))
        } else if self.match_token(&Token::Minus) {
            let expr = self.unary()?;
            Ok(Expr::NegExpr(Box::new(expr)))
        } else if self.match_token(&Token::BitNot) {
            let expr = self.unary()?;
            Ok(Expr::BitNotExpr(Box::new(expr)))
        } else if self.match_token(&Token::Multiply) {
            // Dereference operator
            let expr = self.unary()?;
            Ok(Expr::DerefExpr(Box::new(expr)))
        } else if self.match_token(&Token::BitAnd) {
            // Address-of operator
            let expr = self.unary()?;
            Ok(Expr::AddressOfExpr(Box::new(expr)))
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&Token::LeftParen) {
                let arg = self.expression()?;
                self.consume(&Token::RightParen)?;
                expr = Expr::ApplyExpr(Box::new(expr), Box::new(arg));
            } else if self.match_token(&Token::Dot) {
                let method_name = self.consume_identifier()?;
                if method_name == "len" {
                    expr = Expr::ArrayLenExpr(Box::new(expr));
                } else if self.match_token(&Token::LeftParen) {
                    // Handle method calls with multiple arguments
                    let mut args = Vec::new();

                    if !self.check(&Token::RightParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }

                    self.consume(&Token::RightParen)?;

                    // Check if this is a data structure method call or regular method call
                    if self.is_data_structure_method(&method_name) {
                        expr = Expr::DataStructureMethodCall(Box::new(expr), method_name, args);
                    } else if args.len() == 1 {
                        // Traditional single-argument method call
                        expr = Expr::MethodCallExpr(Box::new(expr), method_name, Box::new(args[0].clone()));
                    } else if args.is_empty() {
                        // Zero-argument method call - use NullExpr as placeholder argument
                        expr = Expr::MethodCallExpr(Box::new(expr), method_name, Box::new(Expr::NullExpr));
                    } else {
                        return Err(self.error("Traditional method calls only support single arguments"));
                    }
                } else {
                    expr = Expr::FieldAccessExpr(Box::new(expr), method_name);
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn is_data_structure_method(&self, method_name: &str) -> bool {
        matches!(method_name,
            "push" | "pop" | "get" | "set" | "add" | "contains" | "remove" |
            "put" | "containsKey" | "enqueue" | "dequeue" | "front" | "peek" |
            "prepend" | "head" | "size" | "isEmpty"
        )
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.advance().clone();

        match token {
            Token::Int(i) => Ok(Expr::IntExpr(i)),
            Token::Real(r) => Ok(Expr::RealExpr(r)),
            Token::Bool(b) => Ok(Expr::BoolExpr(b)),
            Token::String(s) => Ok(Expr::StringExpr(s)),
            Token::Null => Ok(Expr::NullExpr),
            Token::NullPtr => Ok(Expr::VarExpr("nullptr".to_string())),
            Token::Identifier(name) => {
                if self.match_token(&Token::Arrow) {
                    // Arrow function
                    let body = self.expression()?;
                    Ok(Expr::FuncExpr(name, Box::new(body)))
                } else {
                    Ok(Expr::VarExpr(name))
                }
            }
            Token::E => Ok(Expr::EExpr),
            Token::Pi => Ok(Expr::PiExpr),
            Token::Phi => Ok(Expr::PhiExpr),
            Token::Avogadro => Ok(Expr::AvogadroExpr),
            Token::This => Ok(Expr::ThisExpr),

            // Built-in functions
            Token::Print => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::PrintExpr(Box::new(expr)))
            }
            Token::ToString => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::ToStringExpr(Box::new(expr)))
            }
            Token::TypeOf => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::TypeOfExpr(Box::new(expr)))
            }
            Token::MathSqrt => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::SqrtExpr(Box::new(expr)))
            }
            Token::ToBinary => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::ToBinaryExpr(Box::new(expr)))
            }
            Token::ToHex => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::ToHexExpr(Box::new(expr)))
            }
            Token::ToDecimal => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::ToDecimalExpr(Box::new(expr)))
            }
            Token::Sum => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::SumExpr(Box::new(expr)))
            }
            Token::Err => {
                self.consume(&Token::LeftParen)?;
                let msg = match self.advance().clone() {
                    Token::String(s) => s,
                    _ => return Err(self.error("Expected string literal after 'err'")),
                };
                self.consume(&Token::RightParen)?;
                Ok(Expr::ErrorExpr(msg))
            }
            Token::IsErr => {
                self.consume(&Token::LeftParen)?;
                let expr = self.expression()?;
                self.consume(&Token::RightParen)?;
                Ok(Expr::IsErrorExpr(Box::new(expr)))
            }

            // Control structures
            Token::If => self.if_expression(),
            Token::Match => self.match_expression(),
            Token::For => self.for_expression_or_while(),
            Token::Function => self.function_expression(),
            Token::New => self.new_expression(),

            // Grouping and arrays
            Token::LeftParen => {
                if self.is_lambda() {
                    // Multi-parameter lambda: (x, y) => x + y
                    self.multi_param_lambda()
                } else if self.is_tuple() {
                    self.tuple_expression()
                } else {
                    let expr = self.expression()?;
                    self.consume(&Token::RightParen)?;
                    Ok(expr)
                }
            }
            Token::LeftBracket => self.array_expression(),

            // Data structure constructors
            Token::ArrayListType => self.arraylist_expression(),
            Token::SetType => self.set_expression(),
            Token::MapType => self.map_expression(),
            Token::StackType => self.stack_expression(),
            Token::QueueType => self.queue_expression(),
            Token::LinkedListType => self.linkedlist_expression(),

            _ => Err(self.error(&format!("Unexpected token: {:?}", token))),
        }
    }

    fn if_expression(&mut self) -> Result<Expr> {
        let condition = self.expression()?;
        self.consume(&Token::LeftBrace)?;
        let then_expr = self.expression()?;
        self.consume(&Token::RightBrace)?;
        self.consume(&Token::Else)?;
        self.consume(&Token::LeftBrace)?;
        let else_expr = self.expression()?;
        self.consume(&Token::RightBrace)?;

        Ok(Expr::IfExpr(
            Box::new(condition),
            Box::new(then_expr),
            Box::new(else_expr),
        ))
    }

    fn match_expression(&mut self) -> Result<Expr> {
        let expr = self.expression()?;
        self.consume(&Token::LeftBrace)?;
        let cases = self.match_cases()?;
        self.consume(&Token::RightBrace)?;

        Ok(Expr::MatchExpr(Box::new(expr), cases))
    }

    fn match_cases(&mut self) -> Result<Vec<MatchCase>> {
        let mut cases = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let pattern = self.pattern()?;
            self.consume(&Token::Arrow)?;
            let expr = self.expression()?;
            cases.push((pattern, expr));

            // Optional comma - continue parsing regardless
            self.match_token(&Token::Comma);
        }

        Ok(cases)
    }

    fn pattern(&mut self) -> Result<Pattern> {
        let base_pattern = self.base_pattern()?;

        // Check for guard pattern: pattern if condition
        if self.match_token(&Token::If) {
            let guard_condition = self.expression()?;
            Ok(Pattern::GuardPat(Box::new(base_pattern), Box::new(guard_condition)))
        } else {
            Ok(base_pattern)
        }
    }

    fn base_pattern(&mut self) -> Result<Pattern> {
        let token = self.advance().clone();

        match token {
            Token::Int(i) => {
                // Check if this is a range pattern
                if self.match_token(&Token::Dot) && self.match_token(&Token::Dot) {
                    if let Token::Int(end) = self.advance().clone() {
                        Ok(Pattern::RangePat(i, end))
                    } else {
                        Err(self.error("Expected integer after '..' in range pattern"))
                    }
                } else {
                    Ok(Pattern::LitPat(Literal::IntLit(i)))
                }
            }
            Token::Real(r) => Ok(Pattern::LitPat(Literal::RealLit(r))),
            Token::Bool(b) => Ok(Pattern::LitPat(Literal::BoolLit(b))),
            Token::String(s) => Ok(Pattern::LitPat(Literal::StringLit(s))),
            Token::Null => Ok(Pattern::LitPat(Literal::NullLit)),
            Token::Identifier(name) => Ok(Pattern::VarPat(name)),
            Token::Wildcard => Ok(Pattern::WildcardPat),
            Token::LeftBracket => {
                let mut patterns = Vec::new();

                if !self.check(&Token::RightBracket) {
                    loop {
                        patterns.push(self.base_pattern()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }

                self.consume(&Token::RightBracket)?;
                Ok(Pattern::ArrayPat(patterns))
            }
            Token::LeftParen => {
                // Try to parse as tuple pattern first
                let first_pattern = self.base_pattern()?;

                if self.match_token(&Token::Comma) {
                    // This is a tuple pattern
                    let mut patterns = vec![first_pattern];
                    loop {
                        patterns.push(self.base_pattern()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    self.consume(&Token::RightParen)?;
                    Ok(Pattern::TuplePat(patterns))
                } else {
                    // This is a grouped pattern
                    self.consume(&Token::RightParen)?;
                    Ok(first_pattern)
                }
            }
            _ => Err(self.error(&format!("Invalid pattern: {:?}", token))),
        }
    }

    fn is_tuple_pattern(&self) -> bool {
        // Look ahead to see if this is a tuple pattern (has comma before closing paren)
        let mut lookahead = self.current;
        let mut paren_count = 1;

        while lookahead < self.tokens.len() && paren_count > 0 {
            match &self.tokens[lookahead] {
                Token::LeftParen => paren_count += 1,
                Token::RightParen => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        // Reached the end without finding a comma at the top level
                        return false;
                    }
                }
                Token::Comma if paren_count == 1 => return true,
                _ => {}
            }
            lookahead += 1;
        }

        false
    }

    fn for_expression_or_while(&mut self) -> Result<Expr> {
        // Check if this is a for-in expression or a while-like expression
        if self.peek_for_in() {
            // for var in expr { ... }
            let var = self.consume_identifier()?;
            self.consume(&Token::In)?;
            let array_expr = self.expression()?;
            self.consume(&Token::LeftBrace)?;
            let body_expr = self.expression()?;
            self.consume(&Token::RightBrace)?;

            Ok(Expr::ForInExpr(var, Box::new(array_expr), Box::new(body_expr)))
        } else {
            // for condition { ... } (while-like expression)
            let condition = self.expression()?;
            self.consume(&Token::LeftBrace)?;
            let body = self.expression()?;
            self.consume(&Token::RightBrace)?;

            Ok(Expr::WhileExpr(Box::new(condition), Box::new(body)))
        }
    }

    fn function_expression(&mut self) -> Result<Expr> {
        self.consume(&Token::LeftParen)?;
        let param = self.consume_identifier()?;
        self.consume(&Token::RightParen)?;
        self.consume(&Token::LeftBrace)?;
        self.consume(&Token::Return)?;
        let body = self.expression()?;
        self.consume(&Token::RightBrace)?;

        Ok(Expr::FunctionExpr(param, Box::new(body)))
    }

    fn new_expression(&mut self) -> Result<Expr> {
        let class_name = self.consume_class_name()?;
        self.consume(&Token::LeftParen)?;
        let args = self.expression_list()?;
        self.consume(&Token::RightParen)?;

        Ok(Expr::NewExpr(class_name, args))
    }

    fn array_expression(&mut self) -> Result<Expr> {
        let elements = self.expression_list()?;
        self.consume(&Token::RightBracket)?;
        Ok(Expr::ArrayExpr(elements))
    }

    fn tuple_expression(&mut self) -> Result<Expr> {
        let mut exprs = Vec::new();
        exprs.push(self.expression()?);

        self.consume(&Token::Comma)?;
        loop {
            exprs.push(self.expression()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        self.consume(&Token::RightParen)?;
        Ok(Expr::TupleExpr(exprs))
    }

    fn expression_list(&mut self) -> Result<Vec<Expr>> {
        let mut exprs = Vec::new();

        if !self.check(&Token::RightParen) && !self.check(&Token::RightBracket) {
            loop {
                exprs.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        Ok(exprs)
    }

    fn parse_type(&mut self) -> Result<Type> {
        let token = self.advance().clone();

        let base_type = match token {
            Token::IntType => Type::IntType,
            Token::BoolType => Type::BoolType,
            Token::StringType => Type::StringType,
            Token::Identifier(name) => {
                // Generic type parameter (e.g., T, U, V)
                if name.chars().next().unwrap().is_uppercase() && name.len() == 1 {
                    Type::GenericType(name)
                } else {
                    return Err(self.error(&format!("Unknown type: {}", name)));
                }
            }
            Token::Multiply => {
                // Pointer type (*T)
                let inner_type = self.parse_type()?;
                return Ok(Type::PointerType(Box::new(inner_type)));
            }
            Token::ArrayListType => {
                self.consume(&Token::Less)?;
                let inner_type = self.parse_type()?;
                self.consume(&Token::Greater)?;
                return Ok(Type::ArrayListType(Box::new(inner_type)));
            }
            Token::SetType => {
                self.consume(&Token::Less)?;
                let inner_type = self.parse_type()?;
                self.consume(&Token::Greater)?;
                return Ok(Type::SetType(Box::new(inner_type)));
            }
            Token::MapType => {
                self.consume(&Token::Less)?;
                let key_type = self.parse_type()?;
                self.consume(&Token::Comma)?;
                let value_type = self.parse_type()?;
                self.consume(&Token::Greater)?;
                return Ok(Type::MapType(Box::new(key_type), Box::new(value_type)));
            }
            Token::StackType => {
                self.consume(&Token::Less)?;
                let inner_type = self.parse_type()?;
                self.consume(&Token::Greater)?;
                return Ok(Type::StackType(Box::new(inner_type)));
            }
            Token::QueueType => {
                self.consume(&Token::Less)?;
                let inner_type = self.parse_type()?;
                self.consume(&Token::Greater)?;
                return Ok(Type::QueueType(Box::new(inner_type)));
            }
            Token::LinkedListType => {
                self.consume(&Token::Less)?;
                let inner_type = self.parse_type()?;
                self.consume(&Token::Greater)?;
                return Ok(Type::LinkedListType(Box::new(inner_type)));
            }
            _ => return Err(self.error("Expected type")),
        };

        if self.match_token(&Token::LeftBracket) {
            self.consume(&Token::RightBracket)?;
            Ok(Type::ArrayType(Box::new(base_type)))
        } else {
            Ok(base_type)
        }
    }

    // Helper methods
    fn match_equality_op(&mut self) -> Option<BOp> {
        match self.peek() {
            Token::Equal => { self.advance(); Some(BOp::EqOp) }
            Token::NotEqual => { self.advance(); Some(BOp::NeqOp) }
            _ => None,
        }
    }

    fn match_comparison_op(&mut self) -> Option<BOp> {
        match self.peek() {
            Token::Greater => { self.advance(); Some(BOp::GtOp) }
            Token::GreaterEqual => { self.advance(); Some(BOp::GeqOp) }
            Token::Less => { self.advance(); Some(BOp::LtOp) }
            Token::LessEqual => { self.advance(); Some(BOp::LeqOp) }
            _ => None,
        }
    }

    fn match_shift_op(&mut self) -> Option<BOp> {
        match self.peek() {
            Token::BitLeft => { self.advance(); Some(BOp::BitLeftOp) }
            Token::BitRight => { self.advance(); Some(BOp::BitRightOp) }
            _ => None,
        }
    }

    fn match_term_op(&mut self) -> Option<BOp> {
        match self.peek() {
            Token::Plus => { self.advance(); Some(BOp::PlusOp) }
            Token::Minus => { self.advance(); Some(BOp::MinusOp) }
            _ => None,
        }
    }

    fn match_factor_op(&mut self) -> Option<BOp> {
        match self.peek() {
            Token::Multiply => { self.advance(); Some(BOp::MultOp) }
            Token::Divide => { self.advance(); Some(BOp::DivOp) }
            Token::Modulo => { self.advance(); Some(BOp::ModOp) }
            _ => None,
        }
    }

    fn compound_op_from_token(&self, token: &Token) -> Result<CompoundOp> {
        match token {
            Token::PlusAssign => Ok(CompoundOp::PlusEq),
            Token::MinusAssign => Ok(CompoundOp::MinusEq),
            Token::MultiplyAssign => Ok(CompoundOp::MultEq),
            Token::DivideAssign => Ok(CompoundOp::DivEq),
            Token::ModuloAssign => Ok(CompoundOp::ModEq),
            _ => Err(self.error("Invalid compound assignment operator")),
        }
    }

    fn is_lambda(&self) -> bool {
        // Look ahead to see if this is a lambda: (params...) =>
        let mut lookahead = self.current;
        let mut paren_count = 1;

        while lookahead < self.tokens.len() && paren_count > 0 {
            match &self.tokens[lookahead] {
                Token::LeftParen => paren_count += 1,
                Token::RightParen => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        // Check if next token is arrow
                        return lookahead + 1 < self.tokens.len() &&
                               matches!(self.tokens[lookahead + 1], Token::Arrow);
                    }
                }
                _ => {}
            }
            lookahead += 1;
        }

        false
    }

    fn is_tuple(&self) -> bool {
        // Look ahead to see if this is a tuple (has comma before closing paren)
        let mut lookahead = self.current;
        let mut paren_count = 1;

        while lookahead < self.tokens.len() && paren_count > 0 {
            match &self.tokens[lookahead] {
                Token::LeftParen => paren_count += 1,
                Token::RightParen => paren_count -= 1,
                Token::Comma if paren_count == 1 => return true,
                _ => {}
            }
            lookahead += 1;
        }

        false
    }

    fn multi_param_lambda(&mut self) -> Result<Expr> {
        // Parse (param1, param2, ...) => body
        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                let param = self.consume_identifier()?;
                params.push(param);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightParen)?;
        self.consume(&Token::Arrow)?;

        let body = self.expression()?;

        // For now, create nested single-parameter functions for multi-param lambdas
        // (x, y) => x + y becomes x => y => x + y
        let mut result = body;
        for param in params.into_iter().rev() {
            result = Expr::FuncExpr(param, Box::new(result));
        }

        Ok(result)
    }

    fn consume_semicolon(&mut self) -> Result<()> {
        if self.check(&Token::Semicolon) {
            self.advance();
            Ok(())
        } else {
            Ok(()) // Semicolons are optional in many contexts
        }
    }

    fn consume_identifier(&mut self) -> Result<String> {
        match self.advance().clone() {
            Token::Identifier(name) => Ok(name),
            token => Err(self.error(&format!("Expected identifier, got {:?}", token))),
        }
    }

    fn consume_class_name(&mut self) -> Result<String> {
        match self.advance().clone() {
            Token::ClassName(name) => Ok(name),
            Token::Identifier(name) => Ok(name), // Allow lowercase for now
            token => Err(self.error(&format!("Expected class name, got {:?}", token))),
        }
    }

    fn match_token(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_identifier(&mut self, expected: &str) -> bool {
        if let Token::Identifier(name) = self.peek() {
            if name == expected {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token) ||
            self.peek() == token
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, expected: &Token) -> Result<()> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("Expected {:?}, got {:?}", expected, self.peek())))
        }
    }

    fn error(&self, message: &str) -> CarbonError {
        CarbonError::parser_error(message, self.line, self.column)
    }

    // Data structure parsing methods
    fn arraylist_expression(&mut self) -> Result<Expr> {
        self.consume(&Token::LeftBracket)?;
        let mut exprs = Vec::new();

        if !self.check(&Token::RightBracket) {
            loop {
                exprs.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightBracket)?;
        Ok(Expr::ArrayListExpr(exprs))
    }

    fn set_expression(&mut self) -> Result<Expr> {
        self.consume(&Token::LeftBrace)?;
        let mut exprs = Vec::new();

        if !self.check(&Token::RightBrace) {
            loop {
                exprs.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightBrace)?;
        Ok(Expr::SetExpr(exprs))
    }

    fn map_expression(&mut self) -> Result<Expr> {
        self.consume(&Token::LeftBrace)?;
        let mut pairs = Vec::new();

        if !self.check(&Token::RightBrace) {
            loop {
                let key = self.expression()?;
                self.consume(&Token::Colon)?;
                let value = self.expression()?;
                pairs.push((key, value));

                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightBrace)?;
        Ok(Expr::MapExpr(pairs))
    }

    fn stack_expression(&mut self) -> Result<Expr> {
        self.consume(&Token::LeftBracket)?;
        let mut exprs = Vec::new();

        if !self.check(&Token::RightBracket) {
            loop {
                exprs.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightBracket)?;
        Ok(Expr::StackExpr(exprs))
    }

    fn queue_expression(&mut self) -> Result<Expr> {
        self.consume(&Token::LeftBracket)?;
        let mut exprs = Vec::new();

        if !self.check(&Token::RightBracket) {
            loop {
                exprs.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightBracket)?;
        Ok(Expr::QueueExpr(exprs))
    }

    fn linkedlist_expression(&mut self) -> Result<Expr> {
        self.consume(&Token::LeftBracket)?;
        let mut exprs = Vec::new();

        if !self.check(&Token::RightBracket) {
            loop {
                exprs.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightBracket)?;
        Ok(Expr::LinkedListExpr(exprs))
    }
}
