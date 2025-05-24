use std::collections::HashMap;
use crate::ast::*;
use crate::error::{CarbonError, Result};
use crate::value::{Value, Environment, ClassDef};

pub struct Context {
    pub env: Environment,
    pub classes: HashMap<ClassName, ClassDef>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            classes: HashMap::new(),
        }
    }
}

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn eval_statement(&mut self, context: &mut Context, stmt: Statement) -> Result<Option<Value>> {
        match stmt {
            Statement::ExprStmt(expr) => {
                let value = self.eval_expr(context, expr)?;
                Ok(Some(value))
            }
            Statement::LetStmt(var, expr) => {
                let value = self.eval_expr(context, expr)?;
                context.env.insert(var, value);
                Ok(Some(Value::Null))
            }
            Statement::ConstStmt(var, expr) => {
                let value = self.eval_expr(context, expr)?;
                context.env.insert(var, value);
                Ok(Some(Value::Null))
            }
            Statement::TypedVarStmt(_, var, expr) => {
                let value = self.eval_expr(context, expr)?;
                context.env.insert(var, value);
                Ok(Some(Value::Null))
            }
            Statement::FnDeclStmt(name, params, return_type, statements, return_expr) => {
                let func = Value::TypedFunction {
                    env: context.env.clone(),
                    params,
                    return_type,
                    statements,
                    return_expr: Box::new(return_expr),
                };
                context.env.insert(name, func);
                Ok(Some(Value::Null))
            }
            Statement::ClassStmt(class_name, members) => {
                let mut fields = Vec::new();
                let mut methods = Vec::new();

                for member in members {
                    match member {
                        ClassMember::FieldDecl(field_name) => {
                            fields.push(field_name);
                        }
                        ClassMember::MethodDecl(method_name, param, expr) => {
                            methods.push((method_name, param, expr));
                        }
                    }
                }

                let class_def = ClassDef { fields, methods };
                context.classes.insert(class_name, class_def);
                Ok(Some(Value::Null))
            }
            Statement::ForInStmt(var, array_expr, statements) => {
                let array_value = self.eval_expr(context, array_expr)?;
                match array_value {
                    Value::Array(values) => {
                        let mut last_value = Value::Null;
                        for value in values {
                            context.env.insert(var.clone(), value);
                            for stmt in &statements {
                                if let Some(val) = self.eval_statement(context, stmt.clone())? {
                                    last_value = val;
                                }
                            }
                        }
                        Ok(Some(last_value))
                    }
                    _ => Err(CarbonError::type_error(
                        "for-in loop requires an array",
                    )),
                }
            }
            Statement::CompoundAssignStmt(var, op, expr) => {
                let current = context.env.get(&var)
                    .ok_or_else(|| CarbonError::undefined_variable(&var))?
                    .clone();
                let right = self.eval_expr(context, expr)?;
                let result = self.eval_compound_assignment(op, current, right)?;
                context.env.insert(var, result.clone());
                Ok(Some(result))
            }
            Statement::IncrementStmt(var) => {
                let current = context.env.get(&var)
                    .ok_or_else(|| CarbonError::undefined_variable(&var))?
                    .clone();
                let result = self.eval_increment(current)?;
                context.env.insert(var, result.clone());
                Ok(Some(result))
            }
            Statement::DecrementStmt(var) => {
                let current = context.env.get(&var)
                    .ok_or_else(|| CarbonError::undefined_variable(&var))?
                    .clone();
                let result = self.eval_decrement(current)?;
                context.env.insert(var, result.clone());
                Ok(Some(result))
            }
            Statement::ReturnStmt(expr) => {
                let value = self.eval_expr(context, expr)?;
                Ok(Some(value))
            }
        }
    }

    pub fn eval_expr(&mut self, context: &mut Context, expr: Expr) -> Result<Value> {
        match expr {
            Expr::IntExpr(i) => Ok(Value::Int(i)),
            Expr::RealExpr(r) => Ok(Value::Real(r)),
            Expr::BoolExpr(b) => Ok(Value::Bool(b)),
            Expr::StringExpr(s) => Ok(Value::String(s)),
            Expr::NullExpr => Ok(Value::Null),

            // Mathematical constants
            Expr::EExpr => Ok(Value::e()),
            Expr::PiExpr => Ok(Value::pi()),
            Expr::PhiExpr => Ok(Value::phi()),
            Expr::AvogadroExpr => Ok(Value::avogadro()),

            // Variables
            Expr::VarExpr(name) => {
                context.env.get(&name)
                    .cloned()
                    .ok_or_else(|| CarbonError::undefined_variable(&name))
            }

            // Unary operations
            Expr::NegExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                self.eval_negate(value)
            }
            Expr::NotExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                Ok(Value::Bool(!value.is_truthy()))
            }
            Expr::SqrtExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                self.eval_sqrt(value)
            }

            // Binary operations
            Expr::BOpExpr(op, left, right) => {
                let left_val = self.eval_expr(context, *left)?;
                let right_val = self.eval_expr(context, *right)?;
                self.eval_binary_op(op, left_val, right_val)
            }

            // Control flow
            Expr::IfExpr(condition, then_expr, else_expr) => {
                let cond_val = self.eval_expr(context, *condition)?;
                if cond_val.is_truthy() {
                    self.eval_expr(context, *then_expr)
                } else {
                    self.eval_expr(context, *else_expr)
                }
            }
            Expr::TernaryExpr(condition, then_expr, else_expr) => {
                let cond_val = self.eval_expr(context, *condition)?;
                if cond_val.is_truthy() {
                    self.eval_expr(context, *then_expr)
                } else {
                    self.eval_expr(context, *else_expr)
                }
            }

            // Arrays
            Expr::ArrayExpr(exprs) => {
                let mut values = Vec::new();
                for expr in exprs {
                    values.push(self.eval_expr(context, expr)?);
                }
                Ok(Value::Array(values))
            }
            Expr::ArrayLenExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                match value {
                    Value::Array(ref arr) => Ok(Value::Int(arr.len() as i64)),
                    _ => Err(CarbonError::type_error("'.len' can only be used on arrays")),
                }
            }

            // Functions
            Expr::FuncExpr(param, body) => Ok(Value::Function {
                env: context.env.clone(),
                param,
                body,
            }),
            Expr::FunctionExpr(param, body) => Ok(Value::Function {
                env: context.env.clone(),
                param,
                body,
            }),
            Expr::ApplyExpr(func_expr, arg_expr) => {
                let func = self.eval_expr(context, *func_expr)?;
                let arg = self.eval_expr(context, *arg_expr)?;
                self.apply_function(context, func, arg)
            }

            // Built-in functions
            Expr::PrintExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                Ok(Value::Print(value.to_string()))
            }
            Expr::ToStringExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                Ok(Value::String(value.to_string()))
            }
            Expr::TypeOfExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                Ok(Value::String(value.type_name().to_string()))
            }

            // Pattern matching
            Expr::MatchExpr(expr, cases) => {
                let value = self.eval_expr(context, *expr)?;
                self.eval_match(context, value, cases)
            }

            // Error handling
            Expr::ErrorExpr(msg) => Ok(Value::Error(msg)),
            Expr::IsErrorExpr(expr) => {
                let value = self.eval_expr(context, *expr)?;
                Ok(Value::Bool(value.is_error()))
            }

            // Tuples
            Expr::TupleExpr(exprs) => {
                let mut values = Vec::new();
                for expr in exprs {
                    values.push(self.eval_expr(context, expr)?);
                }
                Ok(Value::Tuple(values))
            }

            // Object-oriented features
            Expr::NewExpr(class_name, _args) => {
                let class_def = context.classes.get(&class_name)
                    .ok_or_else(|| CarbonError::undefined_class(&class_name))?
                    .clone();

                let mut field_values = HashMap::new();
                let mut method_map = HashMap::new();

                // Initialize fields to null
                for field in &class_def.fields {
                    field_values.insert(field.clone(), Value::Null);
                }

                // Store methods
                for (method_name, param, expr) in &class_def.methods {
                    method_map.insert(method_name.clone(), (param.clone(), expr.clone()));
                }

                Ok(Value::Object {
                    class_name,
                    fields: field_values,
                    methods: method_map,
                })
            }
            Expr::FieldAccessExpr(object_expr, field_name) => {
                let object = self.eval_expr(context, *object_expr)?;
                match object {
                    Value::Object { fields, .. } => {
                        fields.get(&field_name)
                            .cloned()
                            .ok_or_else(|| CarbonError::runtime_error(&format!("Field '{}' not found", field_name)))
                    }
                    _ => Err(CarbonError::type_error("Field access requires an object")),
                }
            }
            Expr::MethodCallExpr(object_expr, method_name, arg_expr) => {
                let object = self.eval_expr(context, *object_expr)?;
                let arg = self.eval_expr(context, *arg_expr)?;

                match &object {
                    Value::Object { methods, .. } => {
                        if let Some((param, body)) = methods.get(&method_name) {
                            let mut method_env = context.env.clone();
                            method_env.insert(param.clone(), arg);
                            method_env.insert("this".to_string(), object.clone());

                            let old_env = std::mem::replace(&mut context.env, method_env);
                            let result = self.eval_expr(context, body.clone());
                            context.env = old_env;

                            result
                        } else {
                            Err(CarbonError::runtime_error(&format!("Method '{}' not found", method_name)))
                        }
                    }
                    _ => Err(CarbonError::type_error("Method call requires an object")),
                }
            }
            Expr::ThisExpr => {
                context.env.get("this")
                    .cloned()
                    .ok_or_else(|| CarbonError::runtime_error("'this' is not available in this context"))
            }

            // Loop expressions
            Expr::ForInExpr(var, array_expr, body_expr) => {
                let array_value = self.eval_expr(context, *array_expr)?;
                match array_value {
                    Value::Array(values) => {
                        let mut last_value = Value::Null;
                        for value in values {
                            context.env.insert(var.clone(), value);
                            last_value = self.eval_expr(context, *body_expr.clone())?;
                        }
                        Ok(last_value)
                    }
                    _ => Err(CarbonError::type_error("for-in expression requires an array")),
                }
            }
            Expr::WhileExpr(condition_expr, body_expr) => {
                let mut last_value = Value::Null;
                while {
                    let condition = self.eval_expr(context, *condition_expr.clone())?;
                    condition.is_truthy()
                } {
                    last_value = self.eval_expr(context, *body_expr.clone())?;
                }
                Ok(last_value)
            }
        }
    }

    fn eval_binary_op(&self, op: BOp, left: Value, right: Value) -> Result<Value> {
        match op {
            BOp::PlusOp => self.eval_add(left, right),
            BOp::MinusOp => self.eval_subtract(left, right),
            BOp::MultOp => self.eval_multiply(left, right),
            BOp::DivOp => self.eval_divide(left, right),
            BOp::ModOp => self.eval_modulo(left, right),
            BOp::ExpOp => self.eval_exponent(left, right),
            BOp::EqOp => Ok(Value::Bool(self.values_equal(&left, &right))),
            BOp::NeqOp => Ok(Value::Bool(!self.values_equal(&left, &right))),
            BOp::GtOp => self.eval_greater_than(left, right),
            BOp::LtOp => self.eval_less_than(left, right),
            BOp::GeqOp => self.eval_greater_equal(left, right),
            BOp::LeqOp => self.eval_less_equal(left, right),
            BOp::AndOp => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BOp::OrOp => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
        }
    }

    fn eval_add(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Real(a), Value::Real(b)) => Ok(Value::Real(a + b)),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Real(a as f64 + b)),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Real(a + b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(CarbonError::type_error("Invalid operands for addition")),
        }
    }

    fn eval_subtract(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Real(a), Value::Real(b)) => Ok(Value::Real(a - b)),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Real(a as f64 - b)),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Real(a - b as f64)),
            _ => Err(CarbonError::type_error("Invalid operands for subtraction")),
        }
    }

    fn eval_multiply(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Real(a), Value::Real(b)) => Ok(Value::Real(a * b)),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Real(a as f64 * b)),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Real(a * b as f64)),
            _ => Err(CarbonError::type_error("Invalid operands for multiplication")),
        }
    }

    fn eval_divide(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(CarbonError::DivisionByZero)
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Real(a), Value::Real(b)) => {
                if b == 0.0 {
                    Err(CarbonError::DivisionByZero)
                } else {
                    Ok(Value::Real(a / b))
                }
            }
            (Value::Int(a), Value::Real(b)) => {
                if b == 0.0 {
                    Err(CarbonError::DivisionByZero)
                } else {
                    Ok(Value::Real(a as f64 / b))
                }
            }
            (Value::Real(a), Value::Int(b)) => {
                if b == 0 {
                    Err(CarbonError::DivisionByZero)
                } else {
                    Ok(Value::Real(a / b as f64))
                }
            }
            _ => Err(CarbonError::type_error("Invalid operands for division")),
        }
    }

    fn eval_modulo(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(CarbonError::DivisionByZero)
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            _ => Err(CarbonError::type_error("Modulo operation requires integers")),
        }
    }

    fn eval_exponent(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if b < 0 {
                    Ok(Value::Real((a as f64).powf(b as f64)))
                } else {
                    Ok(Value::Int(a.pow(b as u32)))
                }
            }
            (Value::Real(a), Value::Real(b)) => Ok(Value::Real(a.powf(b))),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Real((a as f64).powf(b))),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Real(a.powf(b as f64))),
            _ => Err(CarbonError::type_error("Invalid operands for exponentiation")),
        }
    }

    fn eval_greater_than(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Real(a), Value::Real(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Bool(a as f64 > b)),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Bool(a > b as f64)),
            _ => Err(CarbonError::type_error("Invalid operands for comparison")),
        }
    }

    fn eval_less_than(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Real(a), Value::Real(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Bool((a as f64) < b)),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Bool(a < b as f64)),
            _ => Err(CarbonError::type_error("Invalid operands for comparison")),
        }
    }

    fn eval_greater_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Real(a), Value::Real(b)) => Ok(Value::Bool(a >= b)),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Bool(a as f64 >= b)),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Bool(a >= b as f64)),
            _ => Err(CarbonError::type_error("Invalid operands for comparison")),
        }
    }

    fn eval_less_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Real(a), Value::Real(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), Value::Real(b)) => Ok(Value::Bool(a as f64 <= b)),
            (Value::Real(a), Value::Int(b)) => Ok(Value::Bool(a <= b as f64)),
            _ => Err(CarbonError::type_error("Invalid operands for comparison")),
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Real(a), Value::Real(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Int(a), Value::Real(b)) => (*a as f64 - b).abs() < f64::EPSILON,
            (Value::Real(a), Value::Int(b)) => (a - *b as f64).abs() < f64::EPSILON,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| self.values_equal(x, y))
            }
            _ => false,
        }
    }

    fn eval_negate(&self, value: Value) -> Result<Value> {
        match value {
            Value::Int(i) => Ok(Value::Int(-i)),
            Value::Real(r) => Ok(Value::Real(-r)),
            _ => Err(CarbonError::type_error("Cannot negate non-numeric value")),
        }
    }

    fn eval_sqrt(&self, value: Value) -> Result<Value> {
        match value {
            Value::Int(i) => Ok(Value::Real((i as f64).sqrt())),
            Value::Real(r) => Ok(Value::Real(r.sqrt())),
            _ => Err(CarbonError::type_error("sqrt requires a number")),
        }
    }

    fn eval_compound_assignment(&self, op: CompoundOp, left: Value, right: Value) -> Result<Value> {
        match op {
            CompoundOp::PlusEq => self.eval_add(left, right),
            CompoundOp::MinusEq => self.eval_subtract(left, right),
            CompoundOp::MultEq => self.eval_multiply(left, right),
            CompoundOp::DivEq => self.eval_divide(left, right),
            CompoundOp::ModEq => self.eval_modulo(left, right),
        }
    }

    fn eval_increment(&self, value: Value) -> Result<Value> {
        match value {
            Value::Int(i) => Ok(Value::Int(i + 1)),
            Value::Real(r) => Ok(Value::Real(r + 1.0)),
            _ => Err(CarbonError::type_error("Cannot increment non-numeric value")),
        }
    }

    fn eval_decrement(&self, value: Value) -> Result<Value> {
        match value {
            Value::Int(i) => Ok(Value::Int(i - 1)),
            Value::Real(r) => Ok(Value::Real(r - 1.0)),
            _ => Err(CarbonError::type_error("Cannot decrement non-numeric value")),
        }
    }

    fn apply_function(&mut self, context: &mut Context, func: Value, arg: Value) -> Result<Value> {
        match func {
            Value::Function { env, param, body } => {
                let mut func_env = env;
                func_env.insert(param, arg);

                let old_env = std::mem::replace(&mut context.env, func_env);
                let result = self.eval_expr(context, *body);
                context.env = old_env;

                result
            }
            Value::TypedFunction { env, params, statements, return_expr, .. } => {
                if params.len() != 1 {
                    return Err(CarbonError::arity_mismatch(params.len(), 1));
                }

                let mut func_env = env;
                func_env.insert(params[0].1.clone(), arg);

                let old_env = std::mem::replace(&mut context.env, func_env);

                // Execute statements
                for stmt in statements {
                    self.eval_statement(context, stmt)?;
                }

                // Evaluate return expression
                let result = self.eval_expr(context, *return_expr);
                context.env = old_env;

                result
            }
            _ => Err(CarbonError::type_error("Cannot apply non-function value")),
        }
    }

    fn eval_match(&mut self, context: &mut Context, value: Value, cases: Vec<MatchCase>) -> Result<Value> {
        for (pattern, expr) in cases {
            let mut bindings = HashMap::new();
            if value.matches_pattern(&pattern, &mut bindings) {
                // Create new environment with pattern bindings
                let old_env = context.env.clone();
                for (var, val) in bindings {
                    context.env.insert(var, val);
                }

                let result = self.eval_expr(context, expr);
                context.env = old_env;
                return result;
            }
        }

        Err(CarbonError::PatternMatchFailed)
    }
}
