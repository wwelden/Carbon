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

### 🚧 In Development

- Function declarations with type annotations
- Object-oriented programming (classes, methods)
- More loop constructs (while, for-range)
- Pattern matching
- Error handling

## Directory Structure

```
carbon-lang/
├── src/           # Source code (Haskell implementation)
├── examples/      # Carbon code examples
├── tests/         # Test files
├── docs/          # Documentation
├── bin/           # Compiled binary
└── README.md      # This file
```

## Building

From the `src/` directory:

```bash
make clean && make
```

This will generate the `carbon` executable.

## Usage

### Interactive REPL

```bash
./carbon
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
```

## Language Design

Carbon combines the best features from multiple languages:

- **Go**: Simple, clean syntax for control flow
- **JavaScript**: Dynamic typing, arrow functions, array methods
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
