use carbon_interpreter::value::*;
use carbon_interpreter::ast::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn test_value_type_names() {
        assert_eq!(Value::Int(42).type_name(), "number");
        assert_eq!(Value::Real(3.14).type_name(), "number");
        assert_eq!(Value::Bool(true).type_name(), "boolean");
        assert_eq!(Value::String("hello".to_string()).type_name(), "string");
        assert_eq!(Value::Null.type_name(), "null");
        assert_eq!(Value::Array(vec![]).type_name(), "array");
        assert_eq!(Value::Tuple(vec![]).type_name(), "tuple");
        assert_eq!(Value::ArrayList(vec![]).type_name(), "array list");
        assert_eq!(Value::Set(HashSet::new()).type_name(), "set");
        assert_eq!(Value::Map(HashMap::new()).type_name(), "map");
        assert_eq!(Value::Stack(vec![]).type_name(), "stack");
        assert_eq!(Value::Queue(VecDeque::new()).type_name(), "queue");
        assert_eq!(Value::LinkedList(None).type_name(), "linked list");
        assert_eq!(Value::Error("test".to_string()).type_name(), "error");
        assert_eq!(Value::Print("test".to_string()).type_name(), "print");
    }

    #[test]
    fn test_value_is_truthy() {
        // Boolean values
        assert!(Value::Bool(true).is_truthy());
        assert!(!Value::Bool(false).is_truthy());

        // Null
        assert!(!Value::Null.is_truthy());

        // Numbers
        assert!(Value::Int(1).is_truthy());
        assert!(!Value::Int(0).is_truthy());
        assert!(Value::Real(1.0).is_truthy());
        assert!(!Value::Real(0.0).is_truthy());

        // Strings
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());

        // Arrays
        assert!(Value::Array(vec![Value::Int(1)]).is_truthy());
        assert!(!Value::Array(vec![]).is_truthy());

        // Other types default to truthy
        assert!(Value::Error("test".to_string()).is_truthy());
        assert!(Value::Print("test".to_string()).is_truthy());
    }

    #[test]
    fn test_value_is_error() {
        assert!(Value::Error("test".to_string()).is_error());
        assert!(!Value::Int(42).is_error());
        assert!(!Value::Bool(true).is_error());
        assert!(!Value::Null.is_error());
    }

    #[test]
    fn test_value_to_number() {
        assert_eq!(Value::Int(42).to_number().unwrap(), 42.0);
        assert_eq!(Value::Real(3.14).to_number().unwrap(), 3.14);
        assert_eq!(Value::Bool(true).to_number().unwrap(), 1.0);
        assert_eq!(Value::Bool(false).to_number().unwrap(), 0.0);

        // Test error cases
        assert!(Value::String("hello".to_string()).to_number().is_err());
        assert!(Value::Null.to_number().is_err());
        assert!(Value::Array(vec![]).to_number().is_err());
    }

    #[test]
    fn test_value_to_int() {
        assert_eq!(Value::Int(42).to_int().unwrap(), 42);
        assert_eq!(Value::Real(3.14).to_int().unwrap(), 3);
        assert_eq!(Value::Bool(true).to_int().unwrap(), 1);
        assert_eq!(Value::Bool(false).to_int().unwrap(), 0);

        // Test error cases
        assert!(Value::String("hello".to_string()).to_int().is_err());
        assert!(Value::Null.to_int().is_err());
    }

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Int(42).to_string(), "42");
        assert_eq!(Value::Real(3.14).to_string(), "3.14");
        assert_eq!(Value::Real(3.0).to_string(), "3.0");
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::Bool(false).to_string(), "false");
        assert_eq!(Value::String("hello".to_string()).to_string(), "hello");
        assert_eq!(Value::Null.to_string(), "null");
        assert_eq!(Value::Array(vec![Value::Int(1), Value::Int(2)]).to_string(), "[1, 2]");
        assert_eq!(Value::Tuple(vec![Value::Int(1), Value::String("hello".to_string())]).to_string(), "(1, hello)");
        assert_eq!(Value::Error("test error".to_string()).to_string(), "error: test error");
        assert_eq!(Value::Print("hello world".to_string()).to_string(), "hello world");
    }

    #[test]
    fn test_math_constants() {
        let pi = Value::pi();
        let e = Value::e();
        let phi = Value::phi();
        let avogadro = Value::avogadro();

        assert!(matches!(pi, Value::Real(f) if (f - std::f64::consts::PI).abs() < f64::EPSILON));
        assert!(matches!(e, Value::Real(f) if (f - std::f64::consts::E).abs() < f64::EPSILON));
        assert!(matches!(phi, Value::Real(_)));
        assert!(matches!(avogadro, Value::Real(_)));
    }

    #[test]
    fn test_pattern_matching_literals() {
        let mut bindings = HashMap::new();

        // Test integer literal matching
        assert!(Value::Int(42).matches_pattern(&Pattern::LitPat(Literal::IntLit(42)), &mut bindings));
        assert!(!Value::Int(42).matches_pattern(&Pattern::LitPat(Literal::IntLit(43)), &mut bindings));

        // Test real literal matching
        assert!(Value::Real(3.14).matches_pattern(&Pattern::LitPat(Literal::RealLit(3.14)), &mut bindings));

        // Test boolean literal matching
        assert!(Value::Bool(true).matches_pattern(&Pattern::LitPat(Literal::BoolLit(true)), &mut bindings));
        assert!(!Value::Bool(true).matches_pattern(&Pattern::LitPat(Literal::BoolLit(false)), &mut bindings));

        // Test string literal matching
        assert!(Value::String("hello".to_string()).matches_pattern(&Pattern::LitPat(Literal::StringLit("hello".to_string())), &mut bindings));

        // Test null literal matching
        assert!(Value::Null.matches_pattern(&Pattern::LitPat(Literal::NullLit), &mut bindings));
    }

    #[test]
    fn test_pattern_matching_variables() {
        let mut bindings = HashMap::new();

        let value = Value::Int(42);
        assert!(value.matches_pattern(&Pattern::VarPat("x".to_string()), &mut bindings));
        assert_eq!(bindings.get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn test_pattern_matching_wildcard() {
        let mut bindings = HashMap::new();

        assert!(Value::Int(42).matches_pattern(&Pattern::WildcardPat, &mut bindings));
        assert!(Value::String("hello".to_string()).matches_pattern(&Pattern::WildcardPat, &mut bindings));
        assert!(Value::Null.matches_pattern(&Pattern::WildcardPat, &mut bindings));
        assert!(bindings.is_empty()); // Wildcard shouldn't bind anything
    }

    #[test]
    fn test_pattern_matching_arrays() {
        let mut bindings = HashMap::new();

        let array = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        let pattern = Pattern::ArrayPat(vec![
            Pattern::VarPat("a".to_string()),
            Pattern::VarPat("b".to_string()),
            Pattern::VarPat("c".to_string())
        ]);

        assert!(array.matches_pattern(&pattern, &mut bindings));
        assert_eq!(bindings.get("a"), Some(&Value::Int(1)));
        assert_eq!(bindings.get("b"), Some(&Value::Int(2)));
        assert_eq!(bindings.get("c"), Some(&Value::Int(3)));

        // Test length mismatch
        let mut bindings2 = HashMap::new();
        let short_pattern = Pattern::ArrayPat(vec![Pattern::VarPat("x".to_string())]);
        assert!(!array.matches_pattern(&short_pattern, &mut bindings2));
    }

    #[test]
    fn test_pattern_matching_tuples() {
        let mut bindings = HashMap::new();

        let tuple = Value::Tuple(vec![Value::Int(42), Value::String("hello".to_string())]);
        let pattern = Pattern::TuplePat(vec![
            Pattern::VarPat("num".to_string()),
            Pattern::VarPat("text".to_string())
        ]);

        assert!(tuple.matches_pattern(&pattern, &mut bindings));
        assert_eq!(bindings.get("num"), Some(&Value::Int(42)));
        assert_eq!(bindings.get("text"), Some(&Value::String("hello".to_string())));
    }

    #[test]
    fn test_pattern_matching_ranges() {
        let mut bindings = HashMap::new();

        assert!(Value::Int(5).matches_pattern(&Pattern::RangePat(1, 10), &mut bindings));
        assert!(Value::Int(1).matches_pattern(&Pattern::RangePat(1, 10), &mut bindings));
        assert!(Value::Int(10).matches_pattern(&Pattern::RangePat(1, 10), &mut bindings));
        assert!(!Value::Int(0).matches_pattern(&Pattern::RangePat(1, 10), &mut bindings));
        assert!(!Value::Int(11).matches_pattern(&Pattern::RangePat(1, 10), &mut bindings));
    }

    #[test]
    fn test_arraylist_operations() {
        let mut arraylist = Value::ArrayList(vec![]);

        // Test push
        assert!(arraylist.arraylist_push(Value::Int(1)).is_ok());
        assert!(arraylist.arraylist_push(Value::Int(2)).is_ok());

        // Test get
        assert_eq!(arraylist.arraylist_get(0).unwrap(), Value::Int(1));
        assert_eq!(arraylist.arraylist_get(1).unwrap(), Value::Int(2));
        assert!(arraylist.arraylist_get(2).is_err()); // Out of bounds

        // Test set
        assert!(arraylist.arraylist_set(0, Value::Int(10)).is_ok());
        assert_eq!(arraylist.arraylist_get(0).unwrap(), Value::Int(10));

        // Test pop
        assert_eq!(arraylist.arraylist_pop().unwrap(), Value::Int(2));
        assert_eq!(arraylist.arraylist_pop().unwrap(), Value::Int(10));
        assert!(arraylist.arraylist_pop().is_err()); // Empty list

        // Test error on wrong type
        let mut not_arraylist = Value::Int(42);
        assert!(not_arraylist.arraylist_push(Value::Int(1)).is_err());
    }

    #[test]
    fn test_set_operations() {
        let mut set = Value::Set(HashSet::new());

        // Test add
        assert_eq!(set.set_add(Value::String("hello".to_string())).unwrap(), true);
        assert_eq!(set.set_add(Value::String("hello".to_string())).unwrap(), false); // Already exists

        // Test contains
        assert_eq!(set.set_contains(&Value::String("hello".to_string())).unwrap(), true);
        assert_eq!(set.set_contains(&Value::String("world".to_string())).unwrap(), false);

        // Test remove
        assert_eq!(set.set_remove(&Value::String("hello".to_string())).unwrap(), true);
        assert_eq!(set.set_remove(&Value::String("hello".to_string())).unwrap(), false); // Already removed

        // Test error on wrong type
        let mut not_set = Value::Int(42);
        assert!(not_set.set_add(Value::String("test".to_string())).is_err());
    }

    #[test]
    fn test_map_operations() {
        let mut map = Value::Map(HashMap::new());

        // Test put
        assert_eq!(map.map_put(Value::String("key1".to_string()), Value::Int(42)).unwrap(), None);
        assert_eq!(map.map_put(Value::String("key1".to_string()), Value::Int(100)).unwrap(), Some(Value::Int(42)));

        // Test get
        assert_eq!(map.map_get(&Value::String("key1".to_string())).unwrap(), Some(Value::Int(100)));
        assert_eq!(map.map_get(&Value::String("key2".to_string())).unwrap(), None);

        // Test contains_key
        assert_eq!(map.map_contains_key(&Value::String("key1".to_string())).unwrap(), true);
        assert_eq!(map.map_contains_key(&Value::String("key2".to_string())).unwrap(), false);

        // Test error on wrong type
        let mut not_map = Value::Int(42);
        assert!(not_map.map_put(Value::String("key".to_string()), Value::Int(1)).is_err());
    }

    #[test]
    fn test_stack_operations() {
        let mut stack = Value::Stack(vec![]);

        // Test push
        assert!(stack.stack_push(Value::Int(1)).is_ok());
        assert!(stack.stack_push(Value::Int(2)).is_ok());

        // Test peek
        assert_eq!(stack.stack_peek().unwrap(), Value::Int(2));

        // Test pop
        assert_eq!(stack.stack_pop().unwrap(), Value::Int(2));
        assert_eq!(stack.stack_pop().unwrap(), Value::Int(1));
        assert!(stack.stack_pop().is_err()); // Empty stack

        // Test error on wrong type
        let mut not_stack = Value::Int(42);
        assert!(not_stack.stack_push(Value::Int(1)).is_err());
    }

    #[test]
    fn test_queue_operations() {
        let mut queue = Value::Queue(VecDeque::new());

        // Test enqueue
        assert!(queue.queue_enqueue(Value::Int(1)).is_ok());
        assert!(queue.queue_enqueue(Value::Int(2)).is_ok());

        // Test front
        assert_eq!(queue.queue_front().unwrap(), Value::Int(1));

        // Test dequeue
        assert_eq!(queue.queue_dequeue().unwrap(), Value::Int(1));
        assert_eq!(queue.queue_dequeue().unwrap(), Value::Int(2));
        assert!(queue.queue_dequeue().is_err()); // Empty queue

        // Test error on wrong type
        let mut not_queue = Value::Int(42);
        assert!(not_queue.queue_enqueue(Value::Int(1)).is_err());
    }

    #[test]
    fn test_linkedlist_operations() {
        let mut linkedlist = Value::LinkedList(None);

        // Test prepend
        assert!(linkedlist.linkedlist_prepend(Value::Int(1)).is_ok());
        assert!(linkedlist.linkedlist_prepend(Value::Int(2)).is_ok());

        // Test head
        assert_eq!(linkedlist.linkedlist_head().unwrap(), Some(Value::Int(2)));

        // Test error on wrong type
        let mut not_linkedlist = Value::Int(42);
        assert!(not_linkedlist.linkedlist_prepend(Value::Int(1)).is_err());
    }

    #[test]
    fn test_size_operations() {
        assert_eq!(Value::Array(vec![Value::Int(1), Value::Int(2)]).size().unwrap(), 2);
        assert_eq!(Value::ArrayList(vec![Value::Int(1)]).size().unwrap(), 1);
        assert_eq!(Value::Stack(vec![Value::Int(1), Value::Int(2), Value::Int(3)]).size().unwrap(), 3);
        assert_eq!(Value::Queue(VecDeque::from([Value::Int(1)])).size().unwrap(), 1);
        assert_eq!(Value::Tuple(vec![Value::Int(1), Value::Int(2)]).size().unwrap(), 2);

        let mut set = HashSet::new();
        set.insert("test".to_string());
        assert_eq!(Value::Set(set).size().unwrap(), 1);

        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Int(42));
        assert_eq!(Value::Map(map).size().unwrap(), 1);

        // Test error on wrong type
        assert!(Value::Int(42).size().is_err());
    }

    #[test]
    fn test_is_empty_operations() {
        assert_eq!(Value::Array(vec![]).is_empty().unwrap(), true);
        assert_eq!(Value::Array(vec![Value::Int(1)]).is_empty().unwrap(), false);
        assert_eq!(Value::ArrayList(vec![]).is_empty().unwrap(), true);
        assert_eq!(Value::Stack(vec![]).is_empty().unwrap(), true);
        assert_eq!(Value::Queue(VecDeque::new()).is_empty().unwrap(), true);
        assert_eq!(Value::Tuple(vec![]).is_empty().unwrap(), true);
        assert_eq!(Value::Set(HashSet::new()).is_empty().unwrap(), true);
        assert_eq!(Value::Map(HashMap::new()).is_empty().unwrap(), true);
        assert_eq!(Value::LinkedList(None).is_empty().unwrap(), true);

        // Test error on wrong type
        assert!(Value::Int(42).is_empty().is_err());
    }

    #[test]
    fn test_value_equality() {
        assert_eq!(Value::Int(42), Value::Int(42));
        assert_ne!(Value::Int(42), Value::Int(43));
        assert_eq!(Value::Real(3.14), Value::Real(3.14));
        assert_eq!(Value::Bool(true), Value::Bool(true));
        assert_eq!(Value::String("hello".to_string()), Value::String("hello".to_string()));
        assert_eq!(Value::Null, Value::Null);

        let array1 = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        let array2 = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        let array3 = Value::Array(vec![Value::Int(1), Value::Int(3)]);
        assert_eq!(array1, array2);
        assert_ne!(array1, array3);
    }

    #[test]
    fn test_value_cloning() {
        let original = Value::Array(vec![Value::Int(1), Value::String("test".to_string())]);
        let cloned = original.clone();
        assert_eq!(original, cloned);

        let tuple_original = Value::Tuple(vec![Value::Int(42), Value::Bool(true)]);
        let tuple_cloned = tuple_original.clone();
        assert_eq!(tuple_original, tuple_cloned);
    }

    #[test]
    fn test_environment_operations() {
        let mut env = Environment::new();
        env.insert("x".to_string(), Value::Int(42));
        env.insert("y".to_string(), Value::String("hello".to_string()));

        assert_eq!(env.get("x"), Some(&Value::Int(42)));
        assert_eq!(env.get("y"), Some(&Value::String("hello".to_string())));
        assert_eq!(env.get("z"), None);
    }

    #[test]
    fn test_linkedlist_node() {
        let node = LinkedListNode {
            value: Box::new(Value::Int(42)),
            next: None,
        };

        assert_eq!(*node.value, Value::Int(42));
        assert_eq!(node.next, None);

        let node_with_next = LinkedListNode {
            value: Box::new(Value::Int(1)),
            next: Some(Box::new(LinkedListNode {
                value: Box::new(Value::Int(2)),
                next: None,
            })),
        };

        assert_eq!(*node_with_next.value, Value::Int(1));
        assert!(node_with_next.next.is_some());
    }
}
