# Carbon Programming Language

Carbon is a modern programming language that amalgamates syntax from Go, JavaScript, Rust, Haskell, C#, Java, and C. It features clean, expressive syntax with powerful functional programming capabilities including Haskell-inspired pattern matching.

## 🚀 Quick Start

```bash
# Build Carbon
make

# Run the REPL
bin/carbon

# Try pattern matching
> match 42 { 42 => "Hello Carbon!", _ => "not found" };
"Hello Carbon!"
```

## Features

### ✅ Working Features

- **Basic Data Types**: integers, floats, booleans, strings, null, arrays
- **Arithmetic Operations**: `+`, `-`, `*`, `/`, `%`, `**` (exponentiation)
- **Comparison Operations**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Boolean Operations**: `&&`, `||`, `!`
- **Variable Declarations**: `let` and `const` (JavaScript style)
- **If Expressions**: `if condition { then_value } else { else_value }` (Go/Python style - no parentheses!)
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
- **Go-Style Error Handling**: Explicit error values and checking (no exceptions)
  - Error creation: `err("something went wrong")`
  - Error checking: `isErr(value)` returns boolean
  - Tuples for multiple returns: `(result, error)` pattern
  - Explicit error handling: `if (isErr(result)) { ... } else { ... }`

### 🚧 In Development

- Function declarations with type annotations
- Object-oriented programming (classes, methods)
- More loop constructs (while, for-range)
- Multi-parameter arrow functions

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
# Build Carbon interpreter
make

# Clean build artifacts
make clean

# Build and install to bin/
make install

# Development build with clean
make dev

# Show build help
make help
```

This will build the Carbon interpreter and place it in `bin/carbon`.

## Usage

### Interactive REPL

```bash
bin/carbon
```

### Running Examples

Explore the `examples/` directory for comprehensive Carbon code samples:

```bash
# View available examples
ls examples/

# Key examples:
# - pattern_matching.cb    # Comprehensive pattern matching demo
# - pattern_demo.cb        # Interactive pattern examples
# - demo.cb               # General language features
# - test_working_features.cb # All working features
```

### Example Code

```carbon
// Variables and basic operations
let x = 42;           // Type inferred
int y = 10;           // Explicit type
string message = "Hello Carbon";

// Array types
int[] numbers = [1, 2, 3, 4, 5];
bool[] flags = [true, false, true];

// Arithmetic
x + y;          // 52
x ** 2;         // 1764

// Compound assignment and increment/decrement
x += 10;        // x = 52
x *= 2;         // x = 104
x /= 4;         // x = 26
x -= 6;         // x = 20
x %= 7;         // x = 6
x++;            // x = 7
x--;            // x = 6

// Comparisons and ternary
x > y ? "greater" : "lesser";  // "greater"

// If expressions (no parentheses needed!)
let result = if x > y { "x wins" } else { "y wins" };
let nested = if x > 0 { if y > 0 { "both positive" } else { "mixed" } } else { "x negative" };

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

// Complex pattern matching
match [1, x, 3] {
    [1, 2, 3] => "exact match",
    [1, y, 3] => y * 100,  // Binds y=2, returns 200
    _ => 0
};

// Go-style error handling
let divide = x => y => {
    if (y == 0) {
        err("division by zero")
    } else {
        x / y
    }
};

let result = divide(10)(2);  // 5
let error = divide(10)(0);   // error: "division by zero"

isErr(result);  // false
isErr(error);   // true

// Tuples for multiple returns
let operation = x => (x * 2, null);  // (result, error)
let tuple = operation(21);           // (42, null)

// Error checking pattern
if (isErr(result)) {
    print("Error occurred");
} else {
    print("Success!");
}
```

## Language Design

Carbon combines the best features from multiple languages:

- **Go**: Simple, clean syntax for control flow (`for item in array`)
- **JavaScript**: Dynamic typing, arrow functions (`x => x + 1`), ternary operator
- **Rust**: Modern pattern matching syntax (`match value { ... }`)
- **Haskell**: Pattern matching semantics (destructuring, variable binding)
- **C/Java**: Basic syntax structure and familiar operators
- **C#**: Object-oriented features (planned)
- **C**: Low-level control (planned)

## Implementation

Carbon is implemented in Haskell using:
- **Alex** for lexical analysis (generates `src/Lexer.hs`)
- **Happy** for parsing (generates `src/Parser.hs`)
- Custom evaluator with environment-based evaluation (`src/Evaluator.hs`)
- REPL with error handling (`src/Main.hs`)

**Parser Status**: 2 shift/reduce conflicts, 1 reduce/reduce conflict (down from 184/3 - 99% reduction!)

## Contributing

The Carbon language is actively being developed. Current priorities:

1. **Eliminate Remaining Conflicts**: Fix the last 2 shift/reduce and 1 reduce/reduce conflict
2. **Multi-Parameter Functions**: Support for `(x, y) => x + y` syntax
3. **Function Declarations**: Type annotations and complex signatures
4. **Object-Oriented Features**: Classes, inheritance, methods
5. **Enhanced Pattern Matching**: Add guards, nested patterns, or expressions
6. **Standard Library**: Built-in functions and utilities

### Development Workflow

```bash
# Make changes to src/
# Build and test
make clean && make

# Test features
bin/carbon
> your_test_code_here

# Run examples
cat examples/pattern_matching.cb | bin/carbon
```

## Recent Accomplishments

- ✅ **Clean If Syntax**: Removed parentheses requirement - `if condition { ... } else { ... }` (Go/Python style)
- ✅ **Parser Conflict Resolution**: Reduced conflicts from 184→2 shift/reduce, 3→1 reduce/reduce (99% improvement!)
- ✅ **Type System Preservation**: Maintained static typing while fixing grammar conflicts
- ✅ **Compound Assignment Operators**: `+=`, `-=`, `*=`, `/=`, `%=` for concise variable updates
- ✅ **Increment/Decrement Operators**: `++` and `--` for convenient counting operations
- ✅ **Go-Style Error Handling**: Explicit error values with `err()` and `isErr()` functions
- ✅ **Tuples**: Multi-value returns with `(value1, value2, ...)` syntax
- ✅ **Pattern Matching**: Full implementation with literal, variable, array, and wildcard patterns
- ✅ **Arrow Functions**: JavaScript-style `x => x + 1` syntax
- ✅ **For-In Loops**: Go-style `for item in array { ... }`
- ✅ **Print Function**: Working output with proper null handling
- ✅ **Clean Architecture**: Organized project structure
- ✅ **Build System**: Complete Makefile with multiple targets

## License

[Add your license here]
