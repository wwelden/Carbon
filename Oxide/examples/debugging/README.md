# Debugging Examples

This directory contains examples used for debugging and testing Oxide features:

- `multiple_nested_tuples_bug.ox` - Demonstrates a parser bug with multiple nested tuple patterns

## Running Examples

```bash
# Run the nested tuples bug example
cargo run -- examples/debugging/multiple_nested_tuples_bug.ox
```

## Known Issues

### Multiple Nested Tuple Patterns Bug

The parser currently fails when a match expression contains multiple nested tuple patterns with the same structure. For example:

```oxide
// This causes a parser error:
match data {
    ("a", (x, y)) => x    // First nested tuple pattern works
    ("a", (z, w)) => z    // Second similar pattern causes "Expected RightParen, got Comma"
}
```

**Workaround**: Avoid multiple nested tuple patterns in the same match expression. Use different pattern structures or separate match expressions.

**Error**: `Parser error: Expected RightParen, got Comma at line 1, column 1`

## Running Debug Examples

These examples are expected to fail and are provided for:
- Bug reproduction
- Regression testing
- Development reference

To test if bugs are fixed:
```bash
cargo run -- examples/debugging/multiple_nested_tuples_bug.ox
```
