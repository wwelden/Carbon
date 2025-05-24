# Pattern Matching and Tuple Examples Organization

This document summarizes the organization of pattern matching and tuple examples created during the debugging session.

## New Directory Structure

### `examples/language_features/pattern_matching/`
Contains working examples of pattern matching features:
- `basic_match.cb` - Simple single-pattern matching
- `multiline_match.cb` - Multiple match cases without commas
- `guard_patterns.cb` - Guard conditions with `if` clauses
- `nested_guard_patterns.cb` - Nested tuples with guards
- `wildcard_patterns.cb` - Using `_` wildcard patterns

### `examples/language_features/tuples/`
Contains working examples of tuple functionality:
- `basic_tuples.cb` - Basic tuple creation and pattern matching
- `triple_tuples.cb` - Three-element tuples with mixed types
- `nested_tuple_patterns.cb` - Simple nested tuple patterns
- `nested_wildcard_patterns.cb` - Nested tuples with wildcards

### `examples/debugging/`
Contains examples of known bugs for testing:
- `multiple_nested_tuples_bug.cb` - Parser bug with multiple nested patterns

### `examples/basic_examples/`
Enhanced with:
- `simple_match.cb` - Moved from root directory
- `basic_tuples_demo.cb` - Simple tuple demonstration

## Fixed Issues

### Match Case Parsing
- **Issue**: Parser required commas between match cases and stopped parsing when none found
- **Fix**: Made commas optional in `match_cases` function
- **Result**: Multiline match expressions now work correctly

### Known Remaining Issue
- **Issue**: Parser fails with multiple nested tuple patterns in same match expression
- **Workaround**: Use different pattern structures or separate match expressions
- **Status**: Documented in debugging directory for future fixes

## Usage Examples

### Working Pattern Matching
```carbon
// Guard patterns
match value {
    x if x > 10 => "large"
    x if x > 0 => "small positive"
    _ => "other"
}

// Nested tuples with guards
match data {
    ("user", (name, age)) if age >= 18 => "Adult: " + name
    _ => "Other"
}
```

### Working Tuples
```carbon
// Basic tuples
let coord = (10, 20)
let person = ("Alice", 25, true)

// Pattern matching
match coord {
    (x, y) => x + y
}
```

### Known Bug (Avoid)
```carbon
// This currently fails:
match data {
    ("a", (x, y)) => x
    ("a", (z, w)) => z  // Parser error here
}
```

## Testing
All working examples have been tested and function correctly. The debugging example is expected to fail and demonstrates the known parser limitation.
