# Tuple Examples

This directory contains examples demonstrating Carbon's tuple functionality.

## Files

- `basic_tuples.cb` - Basic tuple creation and pattern matching
- `triple_tuples.cb` - Working with three-element tuples
- `nested_tuple_patterns.cb` - Pattern matching with nested tuples
- `nested_wildcard_patterns.cb` - Nested tuples with wildcard patterns

## Features Demonstrated

- Tuple creation with multiple types
- Tuple pattern matching and destructuring
- Nested tuple structures
- Wildcard patterns in tuples
- Accessing tuple elements through pattern matching

## Tuple Syntax

```carbon
// Creating tuples
let pair = (10, 20)
let triple = ("Alice", 25, true)
let nested = ("user", ("Bob", 30))

// Pattern matching
match pair {
    (x, y) => x + y
}

// With wildcards
match triple {
    (name, _, isActive) => name
}
```
