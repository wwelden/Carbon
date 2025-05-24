use std::fmt;

pub type Var = String;
pub type ClassName = String;
pub type FieldName = String;
pub type MethodName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExprStmt(Expr),
    LetStmt(Var, Expr),
    ConstStmt(Var, Expr),
    TypedVarStmt(Type, Var, Expr),
    FnDeclStmt(Var, Vec<(Type, Var)>, Option<Type>, Vec<Statement>, Expr),
    ClassStmt(ClassName, Vec<ClassMember>),
    ForInStmt(Var, Expr, Vec<Statement>),
    WhileStmt(Expr, Vec<Statement>),
    CompoundAssignStmt(Var, CompoundOp, Expr),
    IncrementStmt(Var),
    DecrementStmt(Var),
    ReturnStmt(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    IntType,
    BoolType,
    StringType,
    ArrayType(Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    FieldDecl(FieldName),
    MethodDecl(MethodName, Var, Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntExpr(i64),
    RealExpr(f64),
    EExpr,
    PhiExpr,
    PiExpr,
    AvogadroExpr,
    NegExpr(Box<Expr>),
    SqrtExpr(Box<Expr>),
    IfExpr(Box<Expr>, Box<Expr>, Box<Expr>),
    TernaryExpr(Box<Expr>, Box<Expr>, Box<Expr>),
    NotExpr(Box<Expr>),
    StringExpr(String),
    VarExpr(Var),
    BOpExpr(BOp, Box<Expr>, Box<Expr>),
    BoolExpr(bool),
    NullExpr,
    ToStringExpr(Box<Expr>),
    TypeOfExpr(Box<Expr>),
    ArrayExpr(Vec<Expr>),
    FuncExpr(Var, Box<Expr>),
    FunctionExpr(Var, Box<Expr>),
    ApplyExpr(Box<Expr>, Box<Expr>),
    WhileExpr(Box<Expr>, Box<Expr>),
    NewExpr(ClassName, Vec<Expr>),
    MethodCallExpr(Box<Expr>, MethodName, Box<Expr>),
    FieldAccessExpr(Box<Expr>, FieldName),
    ArrayLenExpr(Box<Expr>),
    PrintExpr(Box<Expr>),
    ForInExpr(Var, Box<Expr>, Box<Expr>),
    MatchExpr(Box<Expr>, Vec<MatchCase>),
    ErrorExpr(String),
    TupleExpr(Vec<Expr>),
    IsErrorExpr(Box<Expr>),
    ThisExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BOp {
    PlusOp,
    MinusOp,
    MultOp,
    DivOp,
    ModOp,
    ExpOp,
    EqOp,
    NeqOp,
    GtOp,
    LtOp,
    LeqOp,
    GeqOp,
    AndOp,
    OrOp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompoundOp {
    PlusEq,
    MinusEq,
    MultEq,
    DivEq,
    ModEq,
}

pub type MatchCase = (Pattern, Expr);

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    LitPat(Literal),
    VarPat(Var),
    WildcardPat,
    ArrayPat(Vec<Pattern>),
    GuardPat(Box<Pattern>, Box<Expr>), // Pattern with guard condition
    RangePat(i64, i64), // Range pattern for integers (inclusive)
    TuplePat(Vec<Pattern>), // Tuple pattern matching
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntLit(i64),
    RealLit(f64),
    BoolLit(bool),
    StringLit(String),
    NullLit,
}

impl fmt::Display for BOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            BOp::PlusOp => "+",
            BOp::MinusOp => "-",
            BOp::MultOp => "*",
            BOp::DivOp => "/",
            BOp::ModOp => "%",
            BOp::ExpOp => "**",
            BOp::EqOp => "==",
            BOp::NeqOp => "!=",
            BOp::GtOp => ">",
            BOp::LtOp => "<",
            BOp::LeqOp => "<=",
            BOp::GeqOp => ">=",
            BOp::AndOp => "&&",
            BOp::OrOp => "||",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for CompoundOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            CompoundOp::PlusEq => "+=",
            CompoundOp::MinusEq => "-=",
            CompoundOp::MultEq => "*=",
            CompoundOp::DivEq => "/=",
            CompoundOp::ModEq => "%=",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::IntType => write!(f, "int"),
            Type::BoolType => write!(f, "bool"),
            Type::StringType => write!(f, "string"),
            Type::ArrayType(inner) => write!(f, "{}[]", inner),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::IntLit(i) => write!(f, "{}", i),
            Literal::RealLit(r) => write!(f, "{}", r),
            Literal::BoolLit(b) => write!(f, "{}", b),
            Literal::StringLit(s) => write!(f, "\"{}\"", s),
            Literal::NullLit => write!(f, "null"),
        }
    }
}

// Mathematical constants
impl Expr {
    pub fn math_constant_value(&self) -> Option<f64> {
        match self {
            Expr::PiExpr => Some(std::f64::consts::PI),
            Expr::EExpr => Some(std::f64::consts::E),
            Expr::PhiExpr => Some(1.618033988749894), // Golden ratio
            Expr::AvogadroExpr => Some(6.02214076e23),
            _ => None,
        }
    }
}
