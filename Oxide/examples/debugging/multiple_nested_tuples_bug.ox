// This file demonstrates a parser bug with multiple nested tuple patterns
// Error: Parser error: Expected RightParen, got Comma at line 1, column 1
//
// The issue occurs when there are multiple patterns with the same nested tuple structure
// in a single match expression. Each pattern individually works fine, but combining
// them causes the parser to fail.

match ("a", ("b", 1)) {
    ("a", (x, y)) => x
    ("a", (z, w)) => z
}
