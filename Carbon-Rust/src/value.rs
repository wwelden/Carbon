use std::collections::HashMap;
use std::fmt;
use crate::ast::{Expr, Pattern, Literal, Var, ClassName, FieldName, MethodName, Type, Statement};
use crate::error::CarbonError;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Real(f64),
    Bool(bool),
    String(String),
    Null,
    Array(Vec<Value>),
    Function {
        env: Environment,
        param: Var,
        body: Box<Expr>,
    },
    TypedFunction {
        env: Environment,
        params: Vec<(Type, Var)>,
        return_type: Option<Type>,
        statements: Vec<Statement>,
        return_expr: Box<Expr>,
    },
    Object {
        class_name: ClassName,
        fields: HashMap<FieldName, Value>,
        methods: HashMap<MethodName, (Var, Expr)>,
    },
    Print(String),
    Error(String),
    Tuple(Vec<Value>),
}

pub type Environment = HashMap<Var, Value>;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDef {
    pub fields: Vec<FieldName>,
    pub methods: Vec<(MethodName, Var, Expr)>,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "number",
            Value::Real(_) => "number",
            Value::Bool(_) => "boolean",
            Value::String(_) => "string",
            Value::Null => "null",
            Value::Array(_) => "array",
            Value::Function { .. } => "function",
            Value::TypedFunction { .. } => "function",
            Value::Object { .. } => "object",
            Value::Print(_) => "print",
            Value::Error(_) => "error",
            Value::Tuple(_) => "tuple",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(i) => *i != 0,
            Value::Real(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            _ => true,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Value::Error(_))
    }

    // Convert to number for arithmetic operations
    pub fn to_number(&self) -> Result<f64, CarbonError> {
        match self {
            Value::Int(i) => Ok(*i as f64),
            Value::Real(f) => Ok(*f),
            Value::Bool(true) => Ok(1.0),
            Value::Bool(false) => Ok(0.0),
            _ => Err(CarbonError::type_error(&format!(
                "Cannot convert {} to number",
                self.type_name()
            ))),
        }
    }

    // Convert to integer for certain operations
    pub fn to_int(&self) -> Result<i64, CarbonError> {
        match self {
            Value::Int(i) => Ok(*i),
            Value::Real(f) => Ok(*f as i64),
            Value::Bool(true) => Ok(1),
            Value::Bool(false) => Ok(0),
            _ => Err(CarbonError::type_error(&format!(
                "Cannot convert {} to integer",
                self.type_name()
            ))),
        }
    }

    // Check if patterns match
    pub fn matches_pattern(&self, pattern: &Pattern, bindings: &mut Environment) -> bool {
        match (self, pattern) {
            (_, Pattern::WildcardPat) => true,
            (value, Pattern::VarPat(var)) => {
                bindings.insert(var.clone(), value.clone());
                true
            }
            (Value::Int(i), Pattern::LitPat(Literal::IntLit(pi))) => i == pi,
            (Value::Real(f), Pattern::LitPat(Literal::RealLit(pf))) => (f - pf).abs() < f64::EPSILON,
            (Value::Bool(b), Pattern::LitPat(Literal::BoolLit(pb))) => b == pb,
            (Value::String(s), Pattern::LitPat(Literal::StringLit(ps))) => s == ps,
            (Value::Null, Pattern::LitPat(Literal::NullLit)) => true,
            (Value::Array(arr), Pattern::ArrayPat(patterns)) => {
                if arr.len() != patterns.len() {
                    return false;
                }
                for (value, pattern) in arr.iter().zip(patterns.iter()) {
                    if !value.matches_pattern(pattern, bindings) {
                        return false;
                    }
                }
                true
            }
            (Value::Int(i), Pattern::RangePat(start, end)) => i >= start && i <= end,
            (Value::Tuple(values), Pattern::TuplePat(patterns)) => {
                if values.len() != patterns.len() {
                    return false;
                }
                for (value, pattern) in values.iter().zip(patterns.iter()) {
                    if !value.matches_pattern(pattern, bindings) {
                        return false;
                    }
                }
                true
            }
            // Guard patterns are handled by the evaluator since they need context
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Real(r) => {
                if r.fract() == 0.0 {
                    write!(f, "{:.1}", r)
                } else {
                    write!(f, "{}", r)
                }
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s), // No quotes in output
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            Value::Function { .. } => write!(f, "[function]"),
            Value::TypedFunction { .. } => write!(f, "[function]"),
            Value::Object { class_name, .. } => write!(f, "[object {}]", class_name),
            Value::Print(s) => write!(f, "{}", s),
            Value::Error(msg) => write!(f, "error: {}", msg),
            Value::Tuple(values) => {
                write!(f, "(")?;
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, ")")
            }
        }
    }
}

// Helper function to create mathematical constant values
impl Value {
    pub fn pi() -> Self {
        Value::Real(std::f64::consts::PI)
    }

    pub fn e() -> Self {
        Value::Real(std::f64::consts::E)
    }

    pub fn phi() -> Self {
        Value::Real(1.618033988749894) // Golden ratio
    }

    pub fn avogadro() -> Self {
        Value::Real(6.02214076e23)
    }
}
