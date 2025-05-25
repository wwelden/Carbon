# Carbon & Carbon-Oxide Programming Language

Carbon is a modern programming language that amalgamates syntax from Go, JavaScript, Rust, Haskell, C#, Java, and C. It features clean, expressive syntax with powerful functional programming capabilities including Haskell-inspired pattern matching and Go-style error handling.

This repository contains **two implementations** of the Carbon language:
- **Carbon** - The original implementation written in Haskell
- **Carbon-Oxide (Rust)** - A modern reimplementation written in Rust

## 📖 Project Origins

This started as a fork of British, an impractical programming language made for my programming languages course. Carbon was made with the goal of making British a language I would actually want to use by adding some of my favorite syntax elements from other languages such as Haskell, Rust, Go, Java, C#, C, Javascript, and some new ideas. Oxide is a refactor of Carbon in Rust with additional features. The goal of Oxide will be an efficient versatile language that will be fun to work with. At this current point I am not sure how far I want to go with this, but for now it is a fun summer project.

## 🚀 Quick Start

### Carbon
```bash
cd Carbon
make
bin/carbon
```

### Oxide (Rust)
```bash
cd Oxide
cargo build
cargo run
```

Both implementations support the same Carbon language syntax with enhanced features:
```carbon
// Enhanced for loops (Go-style - no parentheses!)
> let numbers = [1, 2, 3];
> for num in numbers { print(num * 2); }
2
4
6

// While-style loops using 'for'
> let i = 0;
> for i < 3 { print(i); i = i + 1; }
0
1
2

// Function declarations with return types
> fn add(a: int, b: int) -> int { return a + b; }
> add(5, 3);
8

// Advanced pattern matching with guards and ranges
> match 25 {
    x if x > 50 => "large",
    18..30 => "young adult",
    _ => "other"
  };
"young adult"
```

## Language Features

Carbon combines the best features from multiple programming languages with modern enhancements:

### ✅ Core Features

- **Multi-language Syntax Inspiration**:
  - **JavaScript**: `let`/`const` declarations, arrow functions (`x => x + 1`)
  - **Go**: For loops without parentheses, clean syntax (`for condition { }`)
  - **Rust**: Pattern matching syntax with guards and ranges
  - **Haskell**: Functional programming patterns, advanced pattern matching
  - **C/Java**: Familiar operators and expressions

- **Enhanced Data Types**:
  - Primitives: integers, floats, booleans, strings, null
  - Collections: arrays with pattern matching support
  - Tuples: multi-value expressions `(x, y, z)`
  - Custom classes with methods and fields

- **Advanced Control Flow**:
  - Go-style for loops: `for item in collection { }` and `for condition { }`
  - If expressions without parentheses: `if x > 0 { "positive" } else { "negative" }`
  - Pattern matching with guards: `match x { n if n > 10 => "big", _ => "small" }`

- **Function System**:
  - Arrow functions: `let f = x => x * 2`
  - Function declarations with type annotations: `fn add(a: int, b: int) -> int`
  - Curried functions and higher-order functions

- **Object-Oriented Programming**:
  - Class declarations with fields and methods
  - Constructor syntax with `new ClassName()`
  - Method calls and field access

- **Advanced Pattern Matching**:
  - Literal patterns: `42`, `"hello"`, `true`
  - Variable binding: `x`, `name`
  - Array destructuring: `[first, second, ...rest]`
  - Tuple patterns: `(x, y)`
  - Guard patterns: `x if x > 10`
  - Range patterns: `1..10`, `'a'..'z'`
  - Wildcard: `_`

- **Error Handling**: Go-style explicit error handling with `err()` and `isErr()`

### Example Code

```carbon
// Variables and type annotations
let x = 42;
const message = "Hello Carbon";
int y = 100;

// Enhanced for loops (corrected syntax)
let numbers = [1, 2, 3, 4, 5];
for num in numbers {
    print(num * num);
}

// While-style loops using for
let counter = 0;
for counter < 10 {
    print(counter);
    counter = counter + 1;
}

// Function declarations with return types
fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

// Advanced pattern matching
fn describe_number(x: int) -> string {
    match x {
        0 => "zero",
        n if n < 0 => "negative",
        1..10 => "single digit",
        11..100 => "double digit",
        _ => "large number"
    }
}

// Class definitions
class Point {
    x;
    y;

    distance() {
        return Math.sqrt(this.x * this.x + this.y * this.y);
    }
}

let p = new Point(3, 4);
print(p.distance()); // 5

// Tuple destructuring
let coordinates = (10, 20);
match coordinates {
    (x, y) => print("Point at:", x, y)
}

// Error handling
fn safe_divide(a: int, b: int) -> int {
    if b == 0 {
        return err("Division by zero");
    }
    return a / b;
}

let result = safe_divide(10, 2);
if isErr(result) {
    print("Error occurred!");
} else {
    print("Result:", result);
}
```

## Implementation Status

### Carbon - Status: ✅ Mostly Working
Located in `Carbon/`

**Working Features:**
- ✅ For-in loops: `for item in collection { }`
- ✅ Function declarations with return types
- ✅ Advanced pattern matching (literals, variables, arrays, wildcards)
- ✅ Class support with methods and fields
- ✅ Enhanced error handling

**In Progress:**
- ⚠️ While-style for loops: `for condition { }` (parser conflicts being resolved)

### Oxide (Rust) - Status: ✅ Fully Working
Located in `Carbon-Oxide/`

**Working Features:**
- ✅ Both for-in and while-style for loops
- ✅ Function declarations with return types
- ✅ Advanced pattern matching including:
  - Guard patterns: `x if condition`
  - Range patterns: `1..10`
  - Tuple patterns: `(a, b)`
- ✅ Class support with inheritance
- ✅ Enhanced error handling
- ✅ Comprehensive test suite

## Implementation Comparison

| Feature | Carbon | Carbon-Oxide |
|---------|--------|-------|
| **For-in loops** | ✅ Working | ✅ Working |
| **While-style for loops** | ⚠️ In progress | ✅ Working |
| **Function return types** | ✅ Working | ✅ Working |
| **Advanced patterns** | ✅ Working | ✅ Working |
| **Class support** | ✅ Working | ✅ Working |
| **Performance** | Good (lazy evaluation) | Excellent (zero-cost abstractions) |
| **Memory Safety** | Garbage collected | Memory safe without GC |
| **Build Speed** | Slower (GHC compilation) | Faster (incremental compilation) |
| **Error Messages** | Basic | Colored and detailed |

### Carbon - Original Implementation

**Architecture:**
- `Lexer.x` - Alex-based lexical analyzer
- `Parser.y` - Happy-based parser generator with enhanced grammar
- `Evaluator.hs` - Pure functional evaluation engine with pattern matching
- `Main.hs` - REPL and file execution

**Recent Enhancements:**
- Enhanced for loop grammar supporting both patterns
- Function return type annotations in AST
- Advanced pattern matching infrastructure
- Class and method support

**Building:**
```bash
cd Carbon
make            # Build interpreter
make clean      # Clean build artifacts
make install    # Build and install to bin/
bin/carbon examples/for_loops.cb  # Test enhanced loops
```

### Oxide (Rust) - Modern Reimplementation

**Architecture:**
- `lexer.rs` - Hand-written lexer with enhanced token support
- `parser.rs` - Recursive descent parser with lookahead for loop disambiguation
- `evaluator.rs` - Environment-based evaluator with advanced pattern matching
- `error.rs` - Comprehensive error handling with colored output
- `value.rs` - Enhanced value type system
- `ast.rs` - Extended AST with new language constructs

**Recent Enhancements:**
- Intelligent for loop parsing with `peek_for_in()` function
- Complete pattern matching system with guards and ranges
- Function signature parsing with return types
- Enhanced class system with method calls
- Comprehensive example suite

**Building:**
```bash
cd Carbon-Oxide
cargo build --release  # Optimized build
cargo run           # Run REPL
cargo run examples/for_loops.cb      # Test both loop types
cargo run examples/advanced_patterns.cb  # Test pattern matching
cargo test          # Run comprehensive test suite
```

## Example Files

Both implementations include enhanced example files:

### Core Examples
- `demo.cb` - Basic language features
- `pattern_matching.cb` - Pattern matching showcase
- `error_handling.cb` - Go-style error handling
- `if_expressions.cb` - Control flow examples

### New Enhanced Examples
- `for_loops.cb` - Both for-in and while-style loops
- `advanced_patterns.cb` - Guards, ranges, and tuple patterns
- `enhanced_classes.cb` - Object-oriented programming
- `function_types.cb` - Function declarations with type annotations

## Usage

### Interactive REPL

Both implementations provide an enhanced interactive REPL:

```bash
# Haskell version
cd Carbon && bin/carbon

# Rust version
cd Carbon-Oxide && cargo run
```

### Running Carbon Files

```bash
# Haskell version
cd Carbon && bin/carbon examples/enhanced_classes.cb

# Rust version
cd Carbon-Oxide && cargo run examples/advanced_patterns.cb
```

## Development Status

### ✅ Recently Implemented
- **Enhanced For Loops**: Go-style syntax without parentheses
- **Function Return Types**: Full type annotation support
- **Advanced Pattern Matching**: Guards, ranges, tuples
- **Class System**: Object-oriented programming features
- **Enhanced Error Handling**: Better error messages and handling

### 🚧 In Development
- **Haskell parser optimization**: Resolving while-loop conflicts
- **Module system**: Import/export functionality
- **Standard library expansion**: More built-in functions
- **Type inference**: Automatic type deduction

### 🎯 Future Goals
- **Static type checking**: Compile-time type verification
- **Compile-time optimizations**: Performance improvements
- **Package management**: Dependency management system
- **IDE integration**: Language server protocol support
- **WebAssembly compilation target**: Browser execution support

## Language Philosophy

Carbon aims to be:
- **Familiar**: Syntax elements from popular languages, enhanced with modern features
- **Safe**: Memory safety and explicit error handling
- **Expressive**: Powerful pattern matching and functional features
- **Practical**: Real-world applicability with excellent performance
- **Modern**: Contemporary language design principles with advanced type systems
- **Clean**: Go-style syntax without unnecessary parentheses or ceremony
