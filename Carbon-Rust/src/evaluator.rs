use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use crate::ast::{Statement, Expr, BOp, CompoundOp, Var, ClassName, ClassMember, MatchCase, Pattern, Literal};
use crate::value::{Value, Environment, ClassDef, LinkedListNode};
use crate::error::{CarbonError, Result};

pub struct Context {
    pub env: Environment,
    pub classes: HashMap<ClassName, ClassDef>,
}

impl Context {
    pub fn new() -> Self {
        let mut context = Self {
            env: HashMap::new(),
            classes: HashMap::new(),
        };

        // Initialize with built-in higher order functions
        context.init_global_functions();
        context
    }

    fn init_global_functions(&mut self) {
        // Initialize built-in higher order functions
        self.env.insert("map".to_string(), Value::BuiltinFunction("map".to_string()));
        self.env.insert("filter".to_string(), Value::BuiltinFunction("filter".to_string()));
        self.env.insert("fold".to_string(), Value::BuiltinFunction("fold".to_string()));
        self.env.insert("reduce".to_string(), Value::BuiltinFunction("reduce".to_string()));
        self.env.insert("forEach".to_string(), Value::BuiltinFunction("forEach".to_string()));
        self.env.insert("find".to_string(), Value::BuiltinFunction("find".to_string()));
        self.env.insert("any".to_string(), Value::BuiltinFunction("any".to_string()));
        self.env.insert("some".to_string(), Value::BuiltinFunction("some".to_string()));
        self.env.insert("all".to_string(), Value::BuiltinFunction("all".to_string()));
        self.env.insert("every".to_string(), Value::BuiltinFunction("every".to_string()));
        self.env.insert("compose".to_string(), Value::BuiltinFunction("compose".to_string()));
        self.env.insert("zip".to_string(), Value::BuiltinFunction("zip".to_string()));

        // Additional higher order functions
        self.env.insert("flatMap".to_string(), Value::BuiltinFunction("flatMap".to_string()));
        self.env.insert("partition".to_string(), Value::BuiltinFunction("partition".to_string()));
        self.env.insert("groupBy".to_string(), Value::BuiltinFunction("groupBy".to_string()));
        self.env.insert("take".to_string(), Value::BuiltinFunction("take".to_string()));
        self.env.insert("drop".to_string(), Value::BuiltinFunction("drop".to_string()));
        self.env.insert("reverse".to_string(), Value::BuiltinFunction("reverse".to_string()));
        self.env.insert("sort".to_string(), Value::BuiltinFunction("sort".to_string()));
        self.env.insert("unique".to_string(), Value::BuiltinFunction("unique".to_string()));
        self.env.insert("zipWith".to_string(), Value::BuiltinFunction("zipWith".to_string()));
        self.env.insert("scan".to_string(), Value::BuiltinFunction("scan".to_string()));

        // New higher order functions
        self.env.insert("concatMap".to_string(), Value::BuiltinFunction("concatMap".to_string()));
        self.env.insert("chunk".to_string(), Value::BuiltinFunction("chunk".to_string()));
        self.env.insert("window".to_string(), Value::BuiltinFunction("window".to_string()));
        self.env.insert("intercalate".to_string(), Value::BuiltinFunction("intercalate".to_string()));
        self.env.insert("transpose".to_string(), Value::BuiltinFunction("transpose".to_string()));
        self.env.insert("max".to_string(), Value::BuiltinFunction("max".to_string()));
        self.env.insert("min".to_string(), Value::BuiltinFunction("min".to_string()));
        self.env.insert("count".to_string(), Value::BuiltinFunction("count".to_string()));
        self.env.insert("sum".to_string(), Value::BuiltinFunction("sum".to_string()));
        self.env.insert("product".to_string(), Value::BuiltinFunction("product".to_string()));
        self.env.insert("head".to_string(), Value::BuiltinFunction("head".to_string()));
        self.env.insert("tail".to_string(), Value::BuiltinFunction("tail".to_string()));
        self.env.insert("init".to_string(), Value::BuiltinFunction("init".to_string()));
        self.env.insert("last".to_string(), Value::BuiltinFunction("last".to_string()));
        self.env.insert("repeat".to_string(), Value::BuiltinFunction("repeat".to_string()));
        self.env.insert("range".to_string(), Value::BuiltinFunction("range".to_string()));
        self.env.insert("intersperse".to_string(), Value::BuiltinFunction("intersperse".to_string()));
        self.env.insert("sortBy".to_string(), Value::BuiltinFunction("sortBy".to_string()));
        self.env.insert("nub".to_string(), Value::BuiltinFunction("nub".to_string()));
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
            Statement::WhileStmt(condition, statements) => {
                let mut last_value = Value::Null;
                loop {
                    let cond_val = self.eval_expr(context, condition.clone())?;
                    if !cond_val.is_truthy() {
                        break;
                    }
                    for stmt in &statements {
                        if let Some(val) = self.eval_statement(context, stmt.clone())? {
                            last_value = val;
                        }
                    }
                }
                Ok(Some(last_value))
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
                // Handle built-in higher order function implementations
                match name.as_str() {
                    "__builtin_map_implementation" => {
                        self.builtin_map_implementation(context)
                    }
                    "__builtin_filter_implementation" => {
                        self.builtin_filter_implementation(context)
                    }
                    "__builtin_fold_implementation" => {
                        self.builtin_fold_implementation(context)
                    }
                    "__builtin_foreach_implementation" => {
                        self.builtin_foreach_implementation(context)
                    }
                    "__builtin_find_implementation" => {
                        self.builtin_find_implementation(context)
                    }
                    "__builtin_any_implementation" => {
                        self.builtin_any_implementation(context)
                    }
                    "__builtin_all_implementation" => {
                        self.builtin_all_implementation(context)
                    }
                    "__builtin_compose_implementation" => {
                        self.builtin_compose_implementation(context)
                    }
                    "__builtin_zip_implementation" => {
                        self.builtin_zip_implementation(context)
                    }
                    "__builtin_flatmap_implementation" => {
                        self.builtin_flatmap_implementation(context)
                    }
                    "__builtin_partition_implementation" => {
                        self.builtin_partition_implementation(context)
                    }
                    "__builtin_groupby_implementation" => {
                        self.builtin_groupby_implementation(context)
                    }
                    "__builtin_take_implementation" => {
                        self.builtin_take_implementation(context)
                    }
                    "__builtin_drop_implementation" => {
                        self.builtin_drop_implementation(context)
                    }
                    "__builtin_reverse_implementation" => {
                        self.builtin_reverse_implementation(context)
                    }
                    "__builtin_sort_implementation" => {
                        self.builtin_sort_implementation(context)
                    }
                    "__builtin_unique_implementation" => {
                        self.builtin_unique_implementation(context)
                    }
                    "__builtin_zipwith_implementation" => {
                        self.builtin_zipwith_implementation(context)
                    }
                    "__builtin_scan_implementation" => {
                        self.builtin_scan_implementation(context)
                    }
                    // New builtin implementations
                    "__builtin_concatmap_implementation" => {
                        self.builtin_concatmap_implementation(context)
                    }
                    "__builtin_chunk_implementation" => {
                        self.builtin_chunk_implementation(context)
                    }
                    "__builtin_window_implementation" => {
                        self.builtin_window_implementation(context)
                    }
                    "__builtin_intercalate_implementation" => {
                        self.builtin_intercalate_implementation(context)
                    }
                    "__builtin_transpose_implementation" => {
                        self.builtin_transpose_implementation(context)
                    }
                    "__builtin_max_implementation" => {
                        self.builtin_max_implementation(context)
                    }
                    "__builtin_min_implementation" => {
                        self.builtin_min_implementation(context)
                    }
                    "__builtin_count_implementation" => {
                        self.builtin_count_implementation(context)
                    }
                    "__builtin_sum_implementation" => {
                        self.builtin_sum_implementation(context)
                    }
                    "__builtin_product_implementation" => {
                        self.builtin_product_implementation(context)
                    }
                    "__builtin_head_implementation" => {
                        self.builtin_head_implementation(context)
                    }
                    "__builtin_tail_implementation" => {
                        self.builtin_tail_implementation(context)
                    }
                    "__builtin_init_implementation" => {
                        self.builtin_init_implementation(context)
                    }
                    "__builtin_last_implementation" => {
                        self.builtin_last_implementation(context)
                    }
                    "__builtin_repeat_implementation" => {
                        self.builtin_repeat_implementation(context)
                    }
                    "__builtin_range_implementation" => {
                        self.builtin_range_implementation(context)
                    }
                    "__builtin_intersperse_implementation" => {
                        self.builtin_intersperse_implementation(context)
                    }
                    "__builtin_sortby_implementation" => {
                        self.builtin_sortby_implementation(context)
                    }
                    "__builtin_nub_implementation" => {
                        self.builtin_nub_implementation(context)
                    }
                    _ => {
                        context.env.get(&name)
                            .cloned()
                            .ok_or_else(|| CarbonError::undefined_variable(&name))
                    }
                }
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
                    Value::ArrayList(ref list) => Ok(Value::Int(list.len() as i64)),
                    Value::String(ref s) => Ok(Value::Int(s.len() as i64)),
                    _ => Err(CarbonError::type_error("'.len' can only be used on arrays, arraylists, and strings")),
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

            // New data structures
            Expr::ArrayListExpr(exprs) => {
                let mut values = Vec::new();
                for expr in exprs {
                    values.push(self.eval_expr(context, expr)?);
                }
                Ok(Value::ArrayList(values))
            }
            Expr::SetExpr(exprs) => {
                let mut set = HashSet::new();
                for expr in exprs {
                    let value = self.eval_expr(context, expr)?;
                    set.insert(value.to_string());
                }
                Ok(Value::Set(set))
            }
            Expr::MapExpr(pairs) => {
                let mut map = HashMap::new();
                for (key_expr, value_expr) in pairs {
                    let key = self.eval_expr(context, key_expr)?;
                    let value = self.eval_expr(context, value_expr)?;
                    map.insert(key.to_string(), value);
                }
                Ok(Value::Map(map))
            }
            Expr::StackExpr(exprs) => {
                let mut values = Vec::new();
                for expr in exprs {
                    values.push(self.eval_expr(context, expr)?);
                }
                Ok(Value::Stack(values))
            }
            Expr::QueueExpr(exprs) => {
                let mut queue = VecDeque::new();
                for expr in exprs {
                    let value = self.eval_expr(context, expr)?;
                    queue.push_back(value);
                }
                Ok(Value::Queue(queue))
            }
            Expr::LinkedListExpr(exprs) => {
                let mut list = None;
                // Build the linked list in reverse order
                for expr in exprs.into_iter().rev() {
                    let value = self.eval_expr(context, expr)?;
                    let new_node = LinkedListNode {
                        value: Box::new(value),
                        next: list.take(),
                    };
                    list = Some(Box::new(new_node));
                }
                Ok(Value::LinkedList(list))
            }
            Expr::DataStructureMethodCall(obj_expr, method_name, args) => {
                let mut obj = self.eval_expr(context, *obj_expr)?;
                let evaluated_args: Result<Vec<Value>> = args.into_iter()
                    .map(|arg| self.eval_expr(context, arg))
                    .collect();
                let evaluated_args = evaluated_args?;

                match method_name.as_str() {
                    // ArrayList methods
                    "push" if evaluated_args.len() == 1 => {
                        obj.arraylist_push(evaluated_args[0].clone())?;
                        Ok(Value::Null)
                    }
                    "pop" if evaluated_args.is_empty() => {
                        obj.arraylist_pop()
                    }
                    "get" if evaluated_args.len() == 1 => {
                        let index = evaluated_args[0].to_int()?;
                        obj.arraylist_get(index)
                    }
                    "set" if evaluated_args.len() == 2 => {
                        let index = evaluated_args[0].to_int()?;
                        obj.arraylist_set(index, evaluated_args[1].clone())?;
                        Ok(Value::Null)
                    }

                    // Set methods
                    "add" if evaluated_args.len() == 1 => {
                        let added = obj.set_add(evaluated_args[0].clone())?;
                        Ok(Value::Bool(added))
                    }
                    "contains" if evaluated_args.len() == 1 => {
                        let contains = obj.set_contains(&evaluated_args[0])?;
                        Ok(Value::Bool(contains))
                    }
                    "remove" if evaluated_args.len() == 1 => {
                        let removed = obj.set_remove(&evaluated_args[0])?;
                        Ok(Value::Bool(removed))
                    }

                    // Map methods
                    "put" if evaluated_args.len() == 2 => {
                        let old_value = obj.map_put(evaluated_args[0].clone(), evaluated_args[1].clone())?;
                        Ok(old_value.unwrap_or(Value::Null))
                    }
                    "get" if evaluated_args.len() == 1 => {
                        let value = obj.map_get(&evaluated_args[0])?;
                        Ok(value.unwrap_or(Value::Null))
                    }
                    "containsKey" if evaluated_args.len() == 1 => {
                        let contains = obj.map_contains_key(&evaluated_args[0])?;
                        Ok(Value::Bool(contains))
                    }

                    // Stack methods
                    "push" if evaluated_args.len() == 1 => {
                        obj.stack_push(evaluated_args[0].clone())?;
                        Ok(Value::Null)
                    }
                    "pop" if evaluated_args.is_empty() => {
                        obj.stack_pop()
                    }
                    "peek" if evaluated_args.is_empty() => {
                        obj.stack_peek()
                    }

                    // Queue methods
                    "enqueue" if evaluated_args.len() == 1 => {
                        obj.queue_enqueue(evaluated_args[0].clone())?;
                        Ok(Value::Null)
                    }
                    "dequeue" if evaluated_args.is_empty() => {
                        obj.queue_dequeue()
                    }
                    "front" if evaluated_args.is_empty() => {
                        obj.queue_front()
                    }

                    // LinkedList methods
                    "prepend" if evaluated_args.len() == 1 => {
                        obj.linkedlist_prepend(evaluated_args[0].clone())?;
                        Ok(Value::Null)
                    }
                    "head" if evaluated_args.is_empty() => {
                        let head = obj.linkedlist_head()?;
                        Ok(head.unwrap_or(Value::Null))
                    }

                    // Common methods
                    "size" if evaluated_args.is_empty() => {
                        let size = obj.size()?;
                        Ok(Value::Int(size))
                    }
                    "isEmpty" if evaluated_args.is_empty() => {
                        let empty = obj.is_empty()?;
                        Ok(Value::Bool(empty))
                    }

                    _ => Err(CarbonError::runtime_error(&format!(
                        "Unknown method '{}' or wrong number of arguments", method_name)))
                }
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
            Value::BuiltinFunction(name) => {
                // Handle built-in higher order functions
                self.apply_builtin_function(context, &name, arg)
            }
            _ => Err(CarbonError::type_error("Cannot apply non-function value")),
        }
    }

    fn apply_builtin_function(&mut self, context: &mut Context, name: &str, arg: Value) -> Result<Value> {
        match name {
            "map" => {
                // map(fn) returns a function that takes an array
                let captured_fn = arg;

                // Create a closure that captures the function
                let mut env = context.env.clone();
                env.insert("__captured_fn".to_string(), captured_fn);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_map_implementation".to_string())),
                })
            }
            "filter" => {
                // filter(predicate) returns a function that takes an array
                let captured_predicate = arg;

                let mut env = context.env.clone();
                env.insert("__captured_predicate".to_string(), captured_predicate);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_filter_implementation".to_string())),
                })
            }
            "fold" | "reduce" => {
                // fold(fn) returns a function that takes initial value, then array
                let captured_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_fold_fn".to_string(), captured_fn);

                Ok(Value::Function {
                    env,
                    param: "initial".to_string(),
                    body: Box::new(Expr::FuncExpr(
                        "array".to_string(),
                        Box::new(Expr::VarExpr("__builtin_fold_implementation".to_string()))
                    )),
                })
            }
            "forEach" => {
                // forEach(fn) returns a function that takes an array
                let captured_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_foreach_fn".to_string(), captured_fn);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_foreach_implementation".to_string())),
                })
            }
            "find" => {
                // find(predicate) returns a function that takes an array
                let captured_predicate = arg;

                let mut env = context.env.clone();
                env.insert("__captured_find_predicate".to_string(), captured_predicate);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_find_implementation".to_string())),
                })
            }
            "any" | "some" => {
                // any(predicate) returns a function that takes an array
                let captured_predicate = arg;

                let mut env = context.env.clone();
                env.insert("__captured_any_predicate".to_string(), captured_predicate);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_any_implementation".to_string())),
                })
            }
            "all" | "every" => {
                // all(predicate) returns a function that takes an array
                let captured_predicate = arg;

                let mut env = context.env.clone();
                env.insert("__captured_all_predicate".to_string(), captured_predicate);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_all_implementation".to_string())),
                })
            }
            "compose" => {
                // compose(f) returns a function that takes g, then x
                let captured_f = arg;

                let mut env = context.env.clone();
                env.insert("__captured_compose_f".to_string(), captured_f);

                Ok(Value::Function {
                    env,
                    param: "g".to_string(),
                    body: Box::new(Expr::FuncExpr(
                        "x".to_string(),
                        Box::new(Expr::VarExpr("__builtin_compose_implementation".to_string()))
                    )),
                })
            }
            "zip" => {
                // zip(array1) returns a function that takes array2
                let captured_array1 = arg;

                let mut env = context.env.clone();
                env.insert("__captured_zip_array1".to_string(), captured_array1);

                Ok(Value::Function {
                    env,
                    param: "array2".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_zip_implementation".to_string())),
                })
            }
            "flatMap" => {
                // flatMap(fn) returns a function that takes an array
                let captured_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_flatmap_fn".to_string(), captured_fn);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_flatmap_implementation".to_string())),
                })
            }
            "partition" => {
                // partition(predicate) returns a function that takes an array
                let captured_predicate = arg;

                let mut env = context.env.clone();
                env.insert("__captured_partition_predicate".to_string(), captured_predicate);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_partition_implementation".to_string())),
                })
            }
            "groupBy" => {
                // groupBy(keyFn) returns a function that takes an array
                let captured_key_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_groupby_key_fn".to_string(), captured_key_fn);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_groupby_implementation".to_string())),
                })
            }
            "take" => {
                // take(n) returns a function that takes an array
                let captured_n = arg;

                let mut env = context.env.clone();
                env.insert("__captured_take_n".to_string(), captured_n);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_take_implementation".to_string())),
                })
            }
            "drop" => {
                // drop(n) returns a function that takes an array
                let captured_n = arg;

                let mut env = context.env.clone();
                env.insert("__captured_drop_n".to_string(), captured_n);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_drop_implementation".to_string())),
                })
            }
            "reverse" => {
                // reverse() takes an array directly (no currying needed)
                match arg {
                    Value::Array(mut arr) => {
                        arr.reverse();
                        Ok(Value::Array(arr))
                    }
                    _ => Err(CarbonError::type_error("reverse requires an array"))
                }
            }
            "sort" => {
                // sort(compareFn) returns a function that takes an array
                let captured_compare_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_sort_compare_fn".to_string(), captured_compare_fn);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_sort_implementation".to_string())),
                })
            }
            "unique" => {
                // unique() takes an array directly (no currying needed)
                match arg {
                    Value::Array(arr) => {
                        let mut unique_values = Vec::new();
                        let mut seen = std::collections::HashSet::new();

                        for value in arr {
                            let key = value.to_string();
                            if !seen.contains(&key) {
                                seen.insert(key);
                                unique_values.push(value);
                            }
                        }

                        Ok(Value::Array(unique_values))
                    }
                    _ => Err(CarbonError::type_error("unique requires an array"))
                }
            }
            "zipWith" => {
                // zipWith(combineFn) returns a function that takes array1, then array2
                let captured_combine_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_zipwith_combine_fn".to_string(), captured_combine_fn);

                Ok(Value::Function {
                    env,
                    param: "array1".to_string(),
                    body: Box::new(Expr::FuncExpr(
                        "array2".to_string(),
                        Box::new(Expr::VarExpr("__builtin_zipwith_implementation".to_string()))
                    )),
                })
            }
            "scan" => {
                // scan(fn) returns a function that takes initial value, then array
                let captured_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_scan_fn".to_string(), captured_fn);

                Ok(Value::Function {
                    env,
                    param: "initial".to_string(),
                    body: Box::new(Expr::FuncExpr(
                        "array".to_string(),
                        Box::new(Expr::VarExpr("__builtin_scan_implementation".to_string()))
                    )),
                })
            }
            // New higher order functions
            "concatMap" => {
                // concatMap is an alias for flatMap
                let captured_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_flatmap_fn".to_string(), captured_fn);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_concatmap_implementation".to_string())),
                })
            }
            "chunk" => {
                // chunk(size) returns a function that takes an array
                let captured_size = arg;

                let mut env = context.env.clone();
                env.insert("__captured_chunk_size".to_string(), captured_size);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_chunk_implementation".to_string())),
                })
            }
            "window" => {
                // window(size) returns a function that takes an array
                let captured_size = arg;

                let mut env = context.env.clone();
                env.insert("__captured_window_size".to_string(), captured_size);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_window_implementation".to_string())),
                })
            }
            "intercalate" => {
                // intercalate(separator) returns a function that takes an array
                let captured_separator = arg;

                let mut env = context.env.clone();
                env.insert("__captured_intercalate_separator".to_string(), captured_separator);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_intercalate_implementation".to_string())),
                })
            }
            "transpose" => {
                // transpose() takes a 2D array directly (no currying needed)
                match arg {
                    Value::Array(arrays) => {
                        // Call implementation directly
                        let mut env = context.env.clone();
                        env.insert("arrays".to_string(), Value::Array(arrays));
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_transpose_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("transpose requires an array"))
                }
            }
            "max" => {
                // max() takes an array directly (no currying needed), or optionally a comparator
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_max_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => {
                        // If arg is a function, it's a comparator - return curried function
                        let captured_comparator = arg;
                        let mut env = context.env.clone();
                        env.insert("__captured_max_comparator".to_string(), captured_comparator);

                        Ok(Value::Function {
                            env,
                            param: "array".to_string(),
                            body: Box::new(Expr::VarExpr("__builtin_max_implementation".to_string())),
                        })
                    }
                }
            }
            "min" => {
                // min() takes an array directly (no currying needed), or optionally a comparator
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_min_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => {
                        // If arg is a function, it's a comparator - return curried function
                        let captured_comparator = arg;
                        let mut env = context.env.clone();
                        env.insert("__captured_min_comparator".to_string(), captured_comparator);

                        Ok(Value::Function {
                            env,
                            param: "array".to_string(),
                            body: Box::new(Expr::VarExpr("__builtin_min_implementation".to_string())),
                        })
                    }
                }
            }
            "count" => {
                // count(predicate) returns a function that takes an array
                let captured_predicate = arg;

                let mut env = context.env.clone();
                env.insert("__captured_count_predicate".to_string(), captured_predicate);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_count_implementation".to_string())),
                })
            }
            "sum" => {
                // sum() takes an array directly (no currying needed)
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_sum_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("sum requires an array"))
                }
            }
            "product" => {
                // product() takes an array directly (no currying needed)
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_product_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("product requires an array"))
                }
            }
            "head" => {
                // head() takes an array directly (no currying needed)
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_head_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("head requires an array"))
                }
            }
            "tail" => {
                // tail() takes an array directly (no currying needed)
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_tail_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("tail requires an array"))
                }
            }
            "init" => {
                // init() takes an array directly (no currying needed)
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_init_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("init requires an array"))
                }
            }
            "last" => {
                // last() takes an array directly (no currying needed)
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_last_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("last requires an array"))
                }
            }
            "repeat" => {
                // repeat(element) returns a function that takes count
                let captured_element = arg;

                let mut env = context.env.clone();
                env.insert("__captured_repeat_element".to_string(), captured_element);

                Ok(Value::Function {
                    env,
                    param: "count".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_repeat_implementation".to_string())),
                })
            }
            "range" => {
                // range(start) returns a function that takes end
                let captured_start = arg;

                let mut env = context.env.clone();
                env.insert("__captured_range_start".to_string(), captured_start);

                Ok(Value::Function {
                    env,
                    param: "end".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_range_implementation".to_string())),
                })
            }
            "intersperse" => {
                // intersperse(element) returns a function that takes an array
                let captured_element = arg;

                let mut env = context.env.clone();
                env.insert("__captured_intersperse_element".to_string(), captured_element);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_intersperse_implementation".to_string())),
                })
            }
            "sortBy" => {
                // sortBy(keyFn) returns a function that takes an array
                let captured_key_fn = arg;

                let mut env = context.env.clone();
                env.insert("__captured_sortby_key_fn".to_string(), captured_key_fn);

                Ok(Value::Function {
                    env,
                    param: "array".to_string(),
                    body: Box::new(Expr::VarExpr("__builtin_sortby_implementation".to_string())),
                })
            }
            "nub" => {
                // nub is an alias for unique
                match arg {
                    Value::Array(_) => {
                        let mut env = context.env.clone();
                        env.insert("array".to_string(), arg);
                        let old_env = std::mem::replace(&mut context.env, env);
                        let result = self.builtin_nub_implementation(context);
                        context.env = old_env;
                        result
                    }
                    _ => Err(CarbonError::type_error("nub requires an array"))
                }
            }
            _ => Err(CarbonError::runtime_error(&format!("Unknown built-in function: {}", name))),
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

    // Built-in higher order function implementations

    fn builtin_map_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_fn = context.env.get("__captured_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured function for map"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for map"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut result = Vec::new();
                for value in values {
                    let mapped_value = self.apply_function(context, captured_fn.clone(), value)?;
                    result.push(mapped_value);
                }
                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("map requires an array")),
        }
    }

    fn builtin_filter_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_predicate = context.env.get("__captured_predicate")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured predicate for filter"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for filter"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut result = Vec::new();
                for value in values {
                    let predicate_result = self.apply_function(context, captured_predicate.clone(), value.clone())?;
                    if predicate_result.is_truthy() {
                        result.push(value);
                    }
                }
                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("filter requires an array")),
        }
    }

    fn builtin_fold_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_fn = context.env.get("__captured_fold_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured function for fold"))?
            .clone();

        let initial = context.env.get("initial")
            .ok_or_else(|| CarbonError::runtime_error("Missing initial parameter for fold"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for fold"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut accumulator = initial;
                for value in values {
                    // Apply the function with accumulator and current value
                    let fold_fn_applied = self.apply_function(context, captured_fn.clone(), accumulator)?;
                    accumulator = self.apply_function(context, fold_fn_applied, value)?;
                }
                Ok(accumulator)
            }
            _ => Err(CarbonError::type_error("fold requires an array")),
        }
    }

    fn builtin_foreach_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_fn = context.env.get("__captured_foreach_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured function for forEach"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for forEach"))?
            .clone();

        match array {
            Value::Array(values) => {
                for value in values {
                    self.apply_function(context, captured_fn.clone(), value)?;
                }
                Ok(Value::Null)
            }
            _ => Err(CarbonError::type_error("forEach requires an array")),
        }
    }

    fn builtin_find_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_predicate = context.env.get("__captured_find_predicate")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured predicate for find"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for find"))?
            .clone();

        match array {
            Value::Array(values) => {
                for value in values {
                    let predicate_result = self.apply_function(context, captured_predicate.clone(), value.clone())?;
                    if predicate_result.is_truthy() {
                        return Ok(value);
                    }
                }
                Ok(Value::Null)
            }
            _ => Err(CarbonError::type_error("find requires an array")),
        }
    }

    fn builtin_any_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_predicate = context.env.get("__captured_any_predicate")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured predicate for any"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for any"))?
            .clone();

        match array {
            Value::Array(values) => {
                for value in values {
                    let predicate_result = self.apply_function(context, captured_predicate.clone(), value)?;
                    if predicate_result.is_truthy() {
                        return Ok(Value::Bool(true));
                    }
                }
                Ok(Value::Bool(false))
            }
            _ => Err(CarbonError::type_error("any requires an array")),
        }
    }

    fn builtin_all_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_predicate = context.env.get("__captured_all_predicate")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured predicate for all"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for all"))?
            .clone();

        match array {
            Value::Array(values) => {
                for value in values {
                    let predicate_result = self.apply_function(context, captured_predicate.clone(), value)?;
                    if !predicate_result.is_truthy() {
                        return Ok(Value::Bool(false));
                    }
                }
                Ok(Value::Bool(true))
            }
            _ => Err(CarbonError::type_error("all requires an array")),
        }
    }

    fn builtin_compose_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_f = context.env.get("__captured_compose_f")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured function f for compose"))?
            .clone();

        let g = context.env.get("g")
            .ok_or_else(|| CarbonError::runtime_error("Missing function g for compose"))?
            .clone();

        let x = context.env.get("x")
            .ok_or_else(|| CarbonError::runtime_error("Missing parameter x for compose"))?
            .clone();

        // Apply g to x first, then f to the result: f(g(x))
        let g_result = self.apply_function(context, g, x)?;
        self.apply_function(context, captured_f, g_result)
    }

    fn builtin_zip_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_array1 = context.env.get("__captured_zip_array1")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured array1 for zip"))?
            .clone();

        let array2 = context.env.get("array2")
            .ok_or_else(|| CarbonError::runtime_error("Missing array2 parameter for zip"))?
            .clone();

        match (captured_array1, array2) {
            (Value::Array(values1), Value::Array(values2)) => {
                let mut result = Vec::new();
                let min_length = values1.len().min(values2.len());

                for i in 0..min_length {
                    let tuple = Value::Tuple(vec![values1[i].clone(), values2[i].clone()]);
                    result.push(tuple);
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("zip requires two arrays")),
        }
    }

    // Additional higher order function implementations

    fn builtin_flatmap_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_fn = context.env.get("__captured_flatmap_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured function for flatMap"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for flatMap"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut result = Vec::new();
                for value in values {
                    let mapped_value = self.apply_function(context, captured_fn.clone(), value)?;
                    match mapped_value {
                        Value::Array(inner_values) => {
                            result.extend(inner_values);
                        }
                        _ => return Err(CarbonError::type_error("flatMap function must return arrays")),
                    }
                }
                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("flatMap requires an array")),
        }
    }

    fn builtin_partition_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_predicate = context.env.get("__captured_partition_predicate")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured predicate for partition"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for partition"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut true_values = Vec::new();
                let mut false_values = Vec::new();

                for value in values {
                    let predicate_result = self.apply_function(context, captured_predicate.clone(), value.clone())?;
                    if predicate_result.is_truthy() {
                        true_values.push(value);
                    } else {
                        false_values.push(value);
                    }
                }

                Ok(Value::Tuple(vec![Value::Array(true_values), Value::Array(false_values)]))
            }
            _ => Err(CarbonError::type_error("partition requires an array")),
        }
    }

    fn builtin_groupby_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_key_fn = context.env.get("__captured_groupby_key_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured key function for groupBy"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for groupBy"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut groups: std::collections::HashMap<String, Vec<Value>> = std::collections::HashMap::new();

                for value in values {
                    let key_result = self.apply_function(context, captured_key_fn.clone(), value.clone())?;
                    let key = key_result.to_string();
                    groups.entry(key).or_insert_with(Vec::new).push(value);
                }

                // Convert to array of tuples (key, values)
                let mut result = Vec::new();
                for (key, group_values) in groups {
                    result.push(Value::Tuple(vec![Value::String(key), Value::Array(group_values)]));
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("groupBy requires an array")),
        }
    }

    fn builtin_take_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_n = context.env.get("__captured_take_n")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured n for take"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for take"))?
            .clone();

        match (captured_n, array) {
            (Value::Int(n), Value::Array(values)) => {
                let take_count = (n as usize).min(values.len());
                Ok(Value::Array(values.into_iter().take(take_count).collect()))
            }
            _ => Err(CarbonError::type_error("take requires an integer and an array")),
        }
    }

    fn builtin_drop_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_n = context.env.get("__captured_drop_n")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured n for drop"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for drop"))?
            .clone();

        match (captured_n, array) {
            (Value::Int(n), Value::Array(values)) => {
                let drop_count = (n as usize).min(values.len());
                Ok(Value::Array(values.into_iter().skip(drop_count).collect()))
            }
            _ => Err(CarbonError::type_error("drop requires an integer and an array")),
        }
    }

    fn builtin_reverse_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for reverse"))?
            .clone();

        match array {
            Value::Array(mut values) => {
                values.reverse();
                Ok(Value::Array(values))
            }
            _ => Err(CarbonError::type_error("reverse requires an array")),
        }
    }

    fn builtin_sort_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_compare_fn = context.env.get("__captured_sort_compare_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured compare function for sort"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for sort"))?
            .clone();

        match array {
            Value::Array(mut values) => {
                // Simple bubble sort using the comparison function
                let len = values.len();
                for i in 0..len {
                    for j in 0..len - 1 - i {
                        let compare_fn_applied = self.apply_function(context, captured_compare_fn.clone(), values[j].clone())?;
                        let comparison_result = self.apply_function(context, compare_fn_applied, values[j + 1].clone())?;

                        // If comparison returns positive (> 0), swap elements
                        if let Value::Int(cmp) = comparison_result {
                            if cmp > 0 {
                                values.swap(j, j + 1);
                            }
                        }
                    }
                }
                Ok(Value::Array(values))
            }
            _ => Err(CarbonError::type_error("sort requires an array")),
        }
    }

    fn builtin_unique_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for unique"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut unique_values = Vec::new();
                let mut seen = std::collections::HashSet::new();

                for value in values {
                    let key = value.to_string();
                    if !seen.contains(&key) {
                        seen.insert(key);
                        unique_values.push(value);
                    }
                }

                Ok(Value::Array(unique_values))
            }
            _ => Err(CarbonError::type_error("unique requires an array")),
        }
    }

    fn builtin_zipwith_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_combine_fn = context.env.get("__captured_zipwith_combine_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured combine function for zipWith"))?
            .clone();

        let array1 = context.env.get("array1")
            .ok_or_else(|| CarbonError::runtime_error("Missing array1 parameter for zipWith"))?
            .clone();

        let array2 = context.env.get("array2")
            .ok_or_else(|| CarbonError::runtime_error("Missing array2 parameter for zipWith"))?
            .clone();

        match (array1, array2) {
            (Value::Array(values1), Value::Array(values2)) => {
                let mut result = Vec::new();
                let min_length = values1.len().min(values2.len());

                for i in 0..min_length {
                    let combine_fn_applied = self.apply_function(context, captured_combine_fn.clone(), values1[i].clone())?;
                    let combined_value = self.apply_function(context, combine_fn_applied, values2[i].clone())?;
                    result.push(combined_value);
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("zipWith requires two arrays")),
        }
    }

    fn builtin_scan_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_fn = context.env.get("__captured_scan_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured function for scan"))?
            .clone();

        let initial = context.env.get("initial")
            .ok_or_else(|| CarbonError::runtime_error("Missing initial parameter for scan"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for scan"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut result = vec![initial.clone()];
                let mut accumulator = initial;

                for value in values {
                    let scan_fn_applied = self.apply_function(context, captured_fn.clone(), accumulator)?;
                    accumulator = self.apply_function(context, scan_fn_applied, value)?;
                    result.push(accumulator.clone());
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("scan requires an array")),
        }
    }

    // New higher order function implementations

    fn builtin_concatmap_implementation(&mut self, context: &mut Context) -> Result<Value> {
        // concatMap is an alias for flatMap, so we reuse the flatMap implementation
        self.builtin_flatmap_implementation(context)
    }

    fn builtin_chunk_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_size = context.env.get("__captured_chunk_size")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured size for chunk"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for chunk"))?
            .clone();

        match (captured_size, array) {
            (Value::Int(size), Value::Array(values)) => {
                if size <= 0 {
                    return Err(CarbonError::runtime_error("Chunk size must be positive"));
                }

                let mut result = Vec::new();
                let chunk_size = size as usize;

                for chunk in values.chunks(chunk_size) {
                    result.push(Value::Array(chunk.to_vec()));
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("chunk requires an integer size and an array")),
        }
    }

    fn builtin_window_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_size = context.env.get("__captured_window_size")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured size for window"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for window"))?
            .clone();

        match (captured_size, array) {
            (Value::Int(size), Value::Array(values)) => {
                if size <= 0 {
                    return Err(CarbonError::runtime_error("Window size must be positive"));
                }

                let mut result = Vec::new();
                let window_size = size as usize;

                if values.len() >= window_size {
                    for i in 0..=(values.len() - window_size) {
                        let window = values[i..i + window_size].to_vec();
                        result.push(Value::Array(window));
                    }
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("window requires an integer size and an array")),
        }
    }

    fn builtin_intercalate_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_separator = context.env.get("__captured_intercalate_separator")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured separator for intercalate"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for intercalate"))?
            .clone();

        match array {
            Value::Array(arrays) => {
                let mut result = Vec::new();

                for (i, array_value) in arrays.iter().enumerate() {
                    if let Value::Array(inner_array) = array_value {
                        if i > 0 {
                            result.push(captured_separator.clone());
                        }
                        result.extend(inner_array.clone());
                    } else {
                        return Err(CarbonError::type_error("intercalate requires an array of arrays"));
                    }
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("intercalate requires an array")),
        }
    }

    fn builtin_transpose_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let arrays = context.env.get("arrays")
            .ok_or_else(|| CarbonError::runtime_error("Missing arrays parameter for transpose"))?
            .clone();

        match arrays {
            Value::Array(outer_arrays) => {
                if outer_arrays.is_empty() {
                    return Ok(Value::Array(Vec::new()));
                }

                // Check that all elements are arrays and find the maximum length
                let mut max_len = 0;
                let mut inner_arrays = Vec::new();

                for array_value in outer_arrays {
                    if let Value::Array(inner_array) = array_value {
                        max_len = max_len.max(inner_array.len());
                        inner_arrays.push(inner_array);
                    } else {
                        return Err(CarbonError::type_error("transpose requires an array of arrays"));
                    }
                }

                // Transpose the arrays
                let mut result = Vec::new();
                for i in 0..max_len {
                    let mut column = Vec::new();
                    for inner_array in &inner_arrays {
                        if i < inner_array.len() {
                            column.push(inner_array[i].clone());
                        }
                    }
                    result.push(Value::Array(column));
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("transpose requires an array")),
        }
    }

    fn builtin_max_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for max"))?
            .clone();

        let comparator = context.env.get("__captured_max_comparator").cloned();

        match array {
            Value::Array(values) => {
                if values.is_empty() {
                    return Ok(Value::Null);
                }

                let mut max_value = values[0].clone();

                if let Some(cmp_fn) = comparator {
                    // Use custom comparator
                    for value in values.iter().skip(1) {
                        let cmp_result = self.apply_function(context, cmp_fn.clone(), max_value.clone())?;
                        let final_cmp = self.apply_function(context, cmp_result, value.clone())?;

                        if let Value::Int(cmp) = final_cmp {
                            if cmp < 0 {
                                max_value = value.clone();
                            }
                        }
                    }
                } else {
                    // Use default comparison for numeric values
                    for value in values.iter().skip(1) {
                        match (&max_value, value) {
                            (Value::Int(max), Value::Int(val)) if val > max => max_value = value.clone(),
                            (Value::Real(max), Value::Real(val)) if val > max => max_value = value.clone(),
                            (Value::Int(max), Value::Real(val)) if val > &(*max as f64) => max_value = value.clone(),
                            (Value::Real(max), Value::Int(val)) if (*val as f64) > *max => max_value = value.clone(),
                            _ => {}
                        }
                    }
                }

                Ok(max_value)
            }
            _ => Err(CarbonError::type_error("max requires an array")),
        }
    }

    fn builtin_min_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for min"))?
            .clone();

        let comparator = context.env.get("__captured_min_comparator").cloned();

        match array {
            Value::Array(values) => {
                if values.is_empty() {
                    return Ok(Value::Null);
                }

                let mut min_value = values[0].clone();

                if let Some(cmp_fn) = comparator {
                    // Use custom comparator
                    for value in values.iter().skip(1) {
                        let cmp_result = self.apply_function(context, cmp_fn.clone(), min_value.clone())?;
                        let final_cmp = self.apply_function(context, cmp_result, value.clone())?;

                        if let Value::Int(cmp) = final_cmp {
                            if cmp > 0 {
                                min_value = value.clone();
                            }
                        }
                    }
                } else {
                    // Use default comparison for numeric values
                    for value in values.iter().skip(1) {
                        match (&min_value, value) {
                            (Value::Int(min), Value::Int(val)) if val < min => min_value = value.clone(),
                            (Value::Real(min), Value::Real(val)) if val < min => min_value = value.clone(),
                            (Value::Int(min), Value::Real(val)) if val < &(*min as f64) => min_value = value.clone(),
                            (Value::Real(min), Value::Int(val)) if (*val as f64) < *min => min_value = value.clone(),
                            _ => {}
                        }
                    }
                }

                Ok(min_value)
            }
            _ => Err(CarbonError::type_error("min requires an array")),
        }
    }

    fn builtin_count_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_predicate = context.env.get("__captured_count_predicate")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured predicate for count"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for count"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut count = 0;
                for value in values {
                    let predicate_result = self.apply_function(context, captured_predicate.clone(), value)?;
                    if predicate_result.is_truthy() {
                        count += 1;
                    }
                }
                Ok(Value::Int(count))
            }
            _ => Err(CarbonError::type_error("count requires an array")),
        }
    }

    fn builtin_sum_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for sum"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut sum = 0.0;
                let mut is_int = true;

                for value in values {
                    match value {
                        Value::Int(i) => sum += i as f64,
                        Value::Real(r) => {
                            sum += r;
                            is_int = false;
                        }
                        _ => return Err(CarbonError::type_error("sum requires an array of numbers")),
                    }
                }

                if is_int {
                    Ok(Value::Int(sum as i64))
                } else {
                    Ok(Value::Real(sum))
                }
            }
            _ => Err(CarbonError::type_error("sum requires an array")),
        }
    }

    fn builtin_product_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for product"))?
            .clone();

        match array {
            Value::Array(values) => {
                let mut product = 1.0;
                let mut is_int = true;

                for value in values {
                    match value {
                        Value::Int(i) => product *= i as f64,
                        Value::Real(r) => {
                            product *= r;
                            is_int = false;
                        }
                        _ => return Err(CarbonError::type_error("product requires an array of numbers")),
                    }
                }

                if is_int {
                    Ok(Value::Int(product as i64))
                } else {
                    Ok(Value::Real(product))
                }
            }
            _ => Err(CarbonError::type_error("product requires an array")),
        }
    }

    fn builtin_head_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for head"))?
            .clone();

        match array {
            Value::Array(values) => {
                if values.is_empty() {
                    Ok(Value::Null)
                } else {
                    Ok(values[0].clone())
                }
            }
            _ => Err(CarbonError::type_error("head requires an array")),
        }
    }

    fn builtin_tail_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for tail"))?
            .clone();

        match array {
            Value::Array(values) => {
                if values.is_empty() {
                    Ok(Value::Array(Vec::new()))
                } else {
                    Ok(Value::Array(values[1..].to_vec()))
                }
            }
            _ => Err(CarbonError::type_error("tail requires an array")),
        }
    }

    fn builtin_init_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for init"))?
            .clone();

        match array {
            Value::Array(values) => {
                if values.is_empty() {
                    Ok(Value::Array(Vec::new()))
                } else {
                    Ok(Value::Array(values[..values.len() - 1].to_vec()))
                }
            }
            _ => Err(CarbonError::type_error("init requires an array")),
        }
    }

    fn builtin_last_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for last"))?
            .clone();

        match array {
            Value::Array(values) => {
                if values.is_empty() {
                    Ok(Value::Null)
                } else {
                    Ok(values[values.len() - 1].clone())
                }
            }
            _ => Err(CarbonError::type_error("last requires an array")),
        }
    }

    fn builtin_repeat_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_element = context.env.get("__captured_repeat_element")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured element for repeat"))?
            .clone();

        let count = context.env.get("count")
            .ok_or_else(|| CarbonError::runtime_error("Missing count parameter for repeat"))?
            .clone();

        match count {
            Value::Int(n) => {
                if n < 0 {
                    return Err(CarbonError::runtime_error("Repeat count must be non-negative"));
                }
                let result = vec![captured_element; n as usize];
                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("repeat requires an integer count")),
        }
    }

    fn builtin_range_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_start = context.env.get("__captured_range_start")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured start for range"))?
            .clone();

        let end = context.env.get("end")
            .ok_or_else(|| CarbonError::runtime_error("Missing end parameter for range"))?
            .clone();

        match (captured_start, end) {
            (Value::Int(start), Value::Int(end)) => {
                let mut result = Vec::new();
                if start <= end {
                    for i in start..end {
                        result.push(Value::Int(i));
                    }
                } else {
                    for i in (end..start).rev() {
                        result.push(Value::Int(i));
                    }
                }
                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("range requires two integers")),
        }
    }

    fn builtin_intersperse_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_element = context.env.get("__captured_intersperse_element")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured element for intersperse"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for intersperse"))?
            .clone();

        match array {
            Value::Array(values) => {
                if values.is_empty() {
                    return Ok(Value::Array(Vec::new()));
                }

                let mut result = Vec::new();
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        result.push(captured_element.clone());
                    }
                    result.push(value.clone());
                }

                Ok(Value::Array(result))
            }
            _ => Err(CarbonError::type_error("intersperse requires an array")),
        }
    }

    fn builtin_sortby_implementation(&mut self, context: &mut Context) -> Result<Value> {
        let captured_key_fn = context.env.get("__captured_sortby_key_fn")
            .ok_or_else(|| CarbonError::runtime_error("Missing captured key function for sortBy"))?
            .clone();

        let array = context.env.get("array")
            .ok_or_else(|| CarbonError::runtime_error("Missing array parameter for sortBy"))?
            .clone();

        match array {
            Value::Array(mut values) => {
                // Create pairs of (key, value) for sorting
                let mut keyed_values = Vec::new();
                for value in values {
                    let key = self.apply_function(context, captured_key_fn.clone(), value.clone())?;
                    keyed_values.push((key, value));
                }

                // Sort by key
                keyed_values.sort_by(|(a, _), (b, _)| {
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => a.cmp(b),
                        (Value::Real(a), Value::Real(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
                        (Value::String(a), Value::String(b)) => a.cmp(b),
                        (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
                        _ => std::cmp::Ordering::Equal,
                    }
                });

                // Extract sorted values
                let sorted_values: Vec<Value> = keyed_values.into_iter().map(|(_, v)| v).collect();
                Ok(Value::Array(sorted_values))
            }
            _ => Err(CarbonError::type_error("sortBy requires an array")),
        }
    }

    fn builtin_nub_implementation(&mut self, context: &mut Context) -> Result<Value> {
        // nub is an alias for unique, so we reuse the unique implementation
        self.builtin_unique_implementation(context)
    }
}
