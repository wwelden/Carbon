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
    VarStmt(Var, Expr),
    TypedVarStmt(Type, Var, Expr),
    TypedLetStmt(Type, Var, Expr),
    FnDeclStmt(Var, Vec<(Type, Var)>, Option<Type>, Vec<Statement>, Expr),
    ClassStmt(ClassName, Option<ClassName>, Vec<ClassMember>),
    ForInStmt(Var, Expr, Vec<Statement>),
    WhileStmt(Expr, Vec<Statement>),
    DoWhileStmt(Vec<Statement>, Expr),
    CompoundAssignStmt(Var, CompoundOp, Expr),
    IncrementStmt(Var),
    DecrementStmt(Var),
    PointerAssignStmt(Expr, Expr),  // *ptr = value
    ReturnStmt(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    IntType,
    BoolType,
    StringType,
    ArrayType(Box<Type>),
    ArrayListType(Box<Type>),    // Dynamic array
    TupleType(Vec<Type>),        // Typed tuple
    SetType(Box<Type>),          // Set data structure
    MapType(Box<Type>, Box<Type>), // Map/Dictionary with key-value types
    StackType(Box<Type>),        // Stack data structure
    QueueType(Box<Type>),        // Queue data structure
    LinkedListType(Box<Type>),   // Linked list
    GenericType(String),         // Generic type parameter (e.g., T, U, V)
    PointerType(Box<Type>),      // Pointer type (*T)
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
    BitNotExpr(Box<Expr>),  // Bitwise NOT operator (~)
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
    TupleAccessExpr(Box<Expr>, usize),               // Tuple element access by index: tuple.0, tuple.1, etc.
    IsErrorExpr(Box<Expr>),
    ThisExpr,
    ArrayListExpr(Vec<Expr>),                    // ArrayList creation
    SetExpr(Vec<Expr>),                          // Set creation
    MapExpr(Vec<(Expr, Expr)>),                  // Map creation with key-value pairs
    StackExpr(Vec<Expr>),                        // Stack creation
    QueueExpr(Vec<Expr>),                        // Queue creation
    LinkedListExpr(Vec<Expr>),                   // LinkedList creation
    DataStructureMethodCall(Box<Expr>, String, Vec<Expr>), // For built-in methods like push, pop, etc.
    SumExpr(Box<Expr>),                          // Sum function for data structures
    // Number base conversion functions
    ToBinaryExpr(Box<Expr>),                     // Convert to binary string
    ToHexExpr(Box<Expr>),                        // Convert to hexadecimal string
    ToDecimalExpr(Box<Expr>),                    // Convert from binary/hex string to decimal
    // Pointer operations
    DerefExpr(Box<Expr>),                        // Dereference pointer (*ptr)
    AddressOfExpr(Box<Expr>),                    // Address-of operator (&var)
    PtrArithExpr(Box<Expr>, i64),                // Pointer arithmetic (ptr + offset)
    PtrIncrementExpr(Box<Expr>),                 // Pointer increment (ptr++)
    PtrDecrementExpr(Box<Expr>),                 // Pointer decrement (ptr--)
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
    // Bitwise operators
    BitAndOp,    // &
    BitOrOp,     // |
    BitXorOp,    // ^
    BitLeftOp,   // <<
    BitRightOp,  // >>
    // Pointer operations
    PtrEqOp,     // Pointer equality comparison
    PtrNeqOp,    // Pointer inequality comparison
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
            BOp::BitAndOp => "&",
            BOp::BitOrOp => "|",
            BOp::BitXorOp => "^",
            BOp::BitLeftOp => "<<",
            BOp::BitRightOp => ">>",
            BOp::PtrEqOp => "==",
            BOp::PtrNeqOp => "!=",
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
            Type::ArrayListType(inner) => write!(f, "ArrayList<{}>", inner),
            Type::TupleType(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::SetType(inner) => write!(f, "Set<{}>", inner),
            Type::MapType(key, value) => write!(f, "Map<{}, {}>", key, value),
            Type::StackType(inner) => write!(f, "Stack<{}>", inner),
            Type::QueueType(inner) => write!(f, "Queue<{}>", inner),
            Type::LinkedListType(inner) => write!(f, "LinkedList<{}>", inner),
            Type::GenericType(name) => write!(f, "{}", name),
            Type::PointerType(inner) => write!(f, "*{}", inner),
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
