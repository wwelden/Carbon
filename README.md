# Carbon Programming Language

Carbon is a modern programming language that amalgamates syntax from Go, JavaScript, Rust, C#, Java, and C. It features a clean, expressive syntax with powerful functional and object-oriented programming capabilities.

## Features

### ✅ Working Features

- **Basic Data Types**: integers, floats, booleans, strings, null, arrays
- **Arithmetic Operations**: `+`, `-`, `*`, `/`, `%`, `**` (exponentiation)
- **Comparison Operations**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Boolean Operations**: `&&`, `||`, `!`
- **Variable Declarations**: `let` and `const` (JavaScript style)
- **Ternary Operator**: `condition ? value1 : value2` (C/JavaScript style)
- **Arrow Functions**: `x => x + 1` (JavaScript style)
- **For-In Loops**: `for item in array { ... }` (Go style)
- **Arrays**: `[1, 2, 3]` with `.len` property
- **Type Checking**: `typeof(value)`
- **String Conversion**: `toString(value)`
- **Print Function**: `print(value)`
- **Mathematical Constants**: `PI`, `E`, `PHI`, `AVOGADRO`
- **Pattern Matching**: Haskell-inspired pattern matching with modern syntax
  - Literal patterns: `match x { 42 => "found", _ => "not found" }`
  - Variable binding: `match value { x => x * 2 }`
  - Array destructuring: `match [1,2,3] { [a,b,c] => a+b+c, _ => 0 }`
  - Mixed patterns: `match [1,x,3] { [1,2,3] => "exact", [1,y,3] => y*10, _ => 0 }`
  - Boolean/String patterns: `match flag { true => "on", false => "off" }`
  - Null patterns: `match value { null => "empty", x => "has value" }`
  - Wildcard patterns: `_` matches anything

### 🚧 In Development

- Function declarations with type annotations
- Object-oriented programming (classes, methods)
- More loop constructs (while, for-range)
- Error handling

## Directory Structure

```
Carbon/
├── src/           # Source code (Haskell implementation)
├── examples/      # Carbon code examples
├── tests/         # Test files
├── docs/          # Documentation
├── bin/           # Compiled binary
└── README.md      # This file
```

## Building

From the project root:

```bash
make
```

This will build the Carbon interpreter and place it in `bin/carbon`.

## Usage

### Interactive REPL

```bash
bin/carbon
```

### Example Code

```carbon
// Variables and basic operations
let x = 42;
let y = 10;
let message = "Hello Carbon";

// Arithmetic
x + y;          // 52
x ** 2;         // 1764

// Comparisons and ternary
x > y ? "greater" : "lesser";  // "greater"

// Arrays
let numbers = [1, 2, 3, 4, 5];
numbers.len;    // 5

// Arrow functions
let double = x => x * 2;
double(21);     // 42

// For-in loops
for num in numbers {
    print(num);
}

// Type checking
typeof(x);      // "number"
typeof(message); // "string"

// Mathematical constants
PI;             // 3.141592653589793
E;              // 2.718281828459045

// Pattern matching
match 42 { 42 => "found", _ => "not found" };  // "found"
match [1,2,3] { [a,b,c] => a+b+c, _ => 0 };    // 6
match true { true => "yes", false => "no" };   // "yes"
```

## Language Design

Carbon combines the best features from multiple languages:

- **Go**: Simple, clean syntax for control flow
- **JavaScript**: Dynamic typing, arrow functions, array methods
- **Haskell**: Pattern matching with modern syntax
- **Rust**: Memory safety concepts (planned)
- **C#**: Object-oriented features (planned)
- **Java**: Strong typing system (planned)
- **C**: Low-level control (planned)

## Implementation

Carbon is implemented in Haskell using:
- **Alex** for lexical analysis
- **Happy** for parsing
- Custom evaluator with environment-based evaluation

## Contributing

The Carbon language is actively being developed. Current priorities:

1. Reduce parser conflicts (currently 76 shift/reduce conflicts)
2. Implement function declarations with type annotations
3. Add object-oriented programming features
4. Improve error messages and debugging

## License

[Add your license here]
