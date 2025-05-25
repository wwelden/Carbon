use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use crate::ast::{Expr, Pattern, Literal, Var, ClassName, FieldName, MethodName, Type, Statement};
use crate::error::CarbonError;

#[derive(Debug, Clone, PartialEq)]
pub struct LinkedListNode {
    pub value: Box<Value>,
    pub next: Option<Box<LinkedListNode>>,
}

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
        return_type: Option<Vec<Type>>,  // Support multiple return types
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
    // New data structures
    ArrayList(Vec<Value>),                      // Dynamic array with extra methods
    Set(HashSet<String>),                       // Set implementation (using String for simplicity)
    Map(HashMap<String, Value>),                // Map/Dictionary implementation
    Stack(Vec<Value>),                          // Stack (LIFO)
    Queue(VecDeque<Value>),                     // Queue (FIFO)
    LinkedList(Option<Box<LinkedListNode>>),    // Linked list implementation
    BuiltinFunction(String),                    // Built-in higher order functions
    Pointer(Box<Value>),                        // Pointer to another value
    NullPointer,                                // Null pointer
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
            Value::ArrayList(_) => "array list",
            Value::Set(_) => "set",
            Value::Map(_) => "map",
            Value::Stack(_) => "stack",
            Value::Queue(_) => "queue",
            Value::LinkedList(_) => "linked list",
            Value::BuiltinFunction(_) => "function",
            Value::Pointer(_) => "pointer",
            Value::NullPointer => "null pointer",
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
            Value::ArrayList(arr) => {
                write!(f, "[")?;
                for (i, value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            Value::Set(set) => {
                write!(f, "{{")?;
                for (i, item) in set.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "}}")
            }
            Value::Map(map) => {
                write!(f, "{{")?;
                for (i, (key, value)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            Value::Stack(stack) => {
                write!(f, "[")?;
                for (i, value) in stack.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            Value::Queue(queue) => {
                write!(f, "[")?;
                for (i, value) in queue.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            Value::LinkedList(list) => {
                match list {
                    Some(node) => {
                        write!(f, "LinkedList(")?;
                        let mut current = node;
                        while let Some(next) = current.next.as_ref() {
                            write!(f, "{} -> ", current.value)?;
                            current = next;
                        }
                        write!(f, "{}", current.value)?;
                        write!(f, ")")
                    }
                    None => write!(f, "LinkedList(empty)"),
                }
            }
            Value::BuiltinFunction(name) => write!(f, "[built-in function {}]", name),
            Value::Pointer(ptr) => write!(f, "[pointer to {}]", ptr),
            Value::NullPointer => write!(f, "[null pointer]"),
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

impl Value {
    // Data structure helper methods

    // ArrayList methods
    pub fn arraylist_push(&mut self, value: Value) -> Result<(), CarbonError> {
        match self {
            Value::ArrayList(ref mut list) => {
                list.push(value);
                Ok(())
            }
            _ => Err(CarbonError::type_error("push() can only be called on ArrayList")),
        }
    }

    pub fn arraylist_pop(&mut self) -> Result<Value, CarbonError> {
        match self {
            Value::ArrayList(ref mut list) => {
                list.pop().ok_or_else(|| CarbonError::runtime_error("Cannot pop from empty ArrayList"))
            }
            _ => Err(CarbonError::type_error("pop() can only be called on ArrayList")),
        }
    }

    pub fn arraylist_get(&self, index: i64) -> Result<Value, CarbonError> {
        match self {
            Value::ArrayList(list) => {
                if index < 0 || index as usize >= list.len() {
                    return Err(CarbonError::runtime_error("ArrayList index out of bounds"));
                }
                Ok(list[index as usize].clone())
            }
            _ => Err(CarbonError::type_error("get() can only be called on ArrayList")),
        }
    }

    pub fn arraylist_set(&mut self, index: i64, value: Value) -> Result<(), CarbonError> {
        match self {
            Value::ArrayList(ref mut list) => {
                if index < 0 || index as usize >= list.len() {
                    return Err(CarbonError::runtime_error("ArrayList index out of bounds"));
                }
                list[index as usize] = value;
                Ok(())
            }
            _ => Err(CarbonError::type_error("set() can only be called on ArrayList")),
        }
    }

    // Set methods
    pub fn set_add(&mut self, value: Value) -> Result<bool, CarbonError> {
        match self {
            Value::Set(ref mut set) => {
                let str_val = value.to_string();
                Ok(set.insert(str_val))
            }
            _ => Err(CarbonError::type_error("add() can only be called on Set")),
        }
    }

    pub fn set_contains(&self, value: &Value) -> Result<bool, CarbonError> {
        match self {
            Value::Set(set) => {
                let str_val = value.to_string();
                Ok(set.contains(&str_val))
            }
            _ => Err(CarbonError::type_error("contains() can only be called on Set")),
        }
    }

    pub fn set_remove(&mut self, value: &Value) -> Result<bool, CarbonError> {
        match self {
            Value::Set(ref mut set) => {
                let str_val = value.to_string();
                Ok(set.remove(&str_val))
            }
            _ => Err(CarbonError::type_error("remove() can only be called on Set")),
        }
    }

    // Map methods
    pub fn map_put(&mut self, key: Value, value: Value) -> Result<Option<Value>, CarbonError> {
        match self {
            Value::Map(ref mut map) => {
                let str_key = key.to_string();
                Ok(map.insert(str_key, value))
            }
            _ => Err(CarbonError::type_error("put() can only be called on Map")),
        }
    }

    pub fn map_get(&self, key: &Value) -> Result<Option<Value>, CarbonError> {
        match self {
            Value::Map(map) => {
                let str_key = key.to_string();
                Ok(map.get(&str_key).cloned())
            }
            _ => Err(CarbonError::type_error("get() can only be called on Map")),
        }
    }

    pub fn map_contains_key(&self, key: &Value) -> Result<bool, CarbonError> {
        match self {
            Value::Map(map) => {
                let str_key = key.to_string();
                Ok(map.contains_key(&str_key))
            }
            _ => Err(CarbonError::type_error("containsKey() can only be called on Map")),
        }
    }

    // Stack methods
    pub fn stack_push(&mut self, value: Value) -> Result<(), CarbonError> {
        match self {
            Value::Stack(ref mut stack) => {
                stack.push(value);
                Ok(())
            }
            _ => Err(CarbonError::type_error("push() can only be called on Stack")),
        }
    }

    pub fn stack_pop(&mut self) -> Result<Value, CarbonError> {
        match self {
            Value::Stack(ref mut stack) => {
                stack.pop().ok_or_else(|| CarbonError::runtime_error("Cannot pop from empty Stack"))
            }
            _ => Err(CarbonError::type_error("pop() can only be called on Stack")),
        }
    }

    pub fn stack_peek(&self) -> Result<Value, CarbonError> {
        match self {
            Value::Stack(stack) => {
                stack.last().cloned().ok_or_else(|| CarbonError::runtime_error("Cannot peek empty Stack"))
            }
            _ => Err(CarbonError::type_error("peek() can only be called on Stack")),
        }
    }

    // Queue methods
    pub fn queue_enqueue(&mut self, value: Value) -> Result<(), CarbonError> {
        match self {
            Value::Queue(ref mut queue) => {
                queue.push_back(value);
                Ok(())
            }
            _ => Err(CarbonError::type_error("enqueue() can only be called on Queue")),
        }
    }

    pub fn queue_dequeue(&mut self) -> Result<Value, CarbonError> {
        match self {
            Value::Queue(ref mut queue) => {
                queue.pop_front().ok_or_else(|| CarbonError::runtime_error("Cannot dequeue from empty Queue"))
            }
            _ => Err(CarbonError::type_error("dequeue() can only be called on Queue")),
        }
    }

    pub fn queue_front(&self) -> Result<Value, CarbonError> {
        match self {
            Value::Queue(queue) => {
                queue.front().cloned().ok_or_else(|| CarbonError::runtime_error("Cannot peek front of empty Queue"))
            }
            _ => Err(CarbonError::type_error("front() can only be called on Queue")),
        }
    }

    // LinkedList methods
    pub fn linkedlist_prepend(&mut self, value: Value) -> Result<(), CarbonError> {
        match self {
            Value::LinkedList(ref mut list) => {
                let new_node = LinkedListNode {
                    value: Box::new(value),
                    next: list.take(),
                };
                *list = Some(Box::new(new_node));
                Ok(())
            }
            _ => Err(CarbonError::type_error("prepend() can only be called on LinkedList")),
        }
    }

    pub fn linkedlist_head(&self) -> Result<Option<Value>, CarbonError> {
        match self {
            Value::LinkedList(list) => {
                Ok(list.as_ref().map(|node| *node.value.clone()))
            }
            _ => Err(CarbonError::type_error("head() can only be called on LinkedList")),
        }
    }

    // General size method for all collections
    pub fn size(&self) -> Result<i64, CarbonError> {
        match self {
            Value::Array(arr) => Ok(arr.len() as i64),
            Value::ArrayList(list) => Ok(list.len() as i64),
            Value::Set(set) => Ok(set.len() as i64),
            Value::Map(map) => Ok(map.len() as i64),
            Value::Stack(stack) => Ok(stack.len() as i64),
            Value::Queue(queue) => Ok(queue.len() as i64),
            Value::Tuple(tuple) => Ok(tuple.len() as i64),
            Value::LinkedList(list) => {
                let mut count = 0;
                let mut current = list.as_ref();
                while let Some(node) = current {
                    count += 1;
                    current = node.next.as_ref();
                }
                Ok(count)
            }
            Value::String(s) => Ok(s.len() as i64),
            _ => Err(CarbonError::type_error("size() not supported for this type")),
        }
    }

    // Check if collection is empty
    pub fn is_empty(&self) -> Result<bool, CarbonError> {
        match self {
            Value::Array(arr) => Ok(arr.is_empty()),
            Value::ArrayList(list) => Ok(list.is_empty()),
            Value::Set(set) => Ok(set.is_empty()),
            Value::Map(map) => Ok(map.is_empty()),
            Value::Stack(stack) => Ok(stack.is_empty()),
            Value::Queue(queue) => Ok(queue.is_empty()),
            Value::Tuple(tuple) => Ok(tuple.is_empty()),
            Value::LinkedList(list) => Ok(list.is_none()),
            Value::String(s) => Ok(s.is_empty()),
            _ => Err(CarbonError::type_error("isEmpty() not supported for this type")),
        }
    }
}
