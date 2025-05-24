# Carbon Programming Language

Carbon is a modern programming language that amalgamates syntax from Go, JavaScript, Rust, Haskell, C#, Java, and C. It features clean, expressive syntax with powerful functional programming capabilities including Haskell-inspired pattern matching and Go-style error handling.

This repository contains **two implementations** of the Carbon language:
- **Carbon(Haskell)** - The original implementation written in Haskell
- **Rusty Carbon(Rust)** - A modern reimplementation written in Rust

## 🚀 Quick Start

### Carbon(Haskell)
```bash
cd Carbon-Haskell
make
bin/carbon
```

### Rusty Carbon(Rust)
```bash
cd Carbon-Rust
cargo build
cargo run
```

Both implementations support the same Carbon language syntax:
```carbon
// Try pattern matching
> match 42 { 42 => "Hello Carbon!", _ => "not found" };
"Hello Carbon!"

// Arrow functions and arrays
> let double = x => x * 2;
> let numbers = [1, 2, 3];
> for num in numbers { print(double(num)); }
2
4
6
```

## Language Features

Carbon combines the best features from multiple programming languages:

### ✅ Core Features

- **Multi-language Syntax Inspiration**:
  - **JavaScript**: `let`/`const` declarations, arrow functions (`x => x + 1`)
  - **Go**: If expressions without parentheses, for-in loops
  - **Rust**: Pattern matching syntax
  - **Haskell**: Functional programming patterns
  - **C/Java**: Familiar operators and expressions

- **Data Types**: integers, floats, booleans, strings, null, arrays
- **Operations**: Full arithmetic (`+`, `-`, `*`, `/`, `%`, `**`), comparisons, boolean logic
- **Control Flow**: If expressions, ternary operator, for-in loops
- **Functions**: Arrow functions with lexical scoping
- **Pattern Matching**: Comprehensive Haskell-inspired pattern matching
- **Error Handling**: Go-style explicit error handling (no exceptions)

### Example Code

```carbon
// Variables (JavaScript-style)
let x = 42;
const message = "Hello Carbon";

// If expressions (Go-style - no parentheses!)
let result = if x > 40 { "large" } else { "small" };

// Arrow functions (JavaScript-style)
let square = x => x * x;
let add = x => y => x + y;  // Curried functions

// Arrays and for-in loops (Go-style)
let numbers = [1, 2, 3, 4, 5];
for num in numbers {
    print(square(num));
}

// Pattern matching (Rust/Haskell-style)
match numbers {
    [a, b, c, d, e] => a + b + c + d + e,
    [first, ...rest] => first * 10,
    _ => 0
};

// Error handling (Go-style)
let divide = x => y => {
    if (y == 0) {
        err("division by zero")
    } else {
        x / y
    }
};

let result = divide(10)(2);  // 5
if (isErr(result)) {
    print("Error occurred!");
} else {
    print("Result:", result);
}
```

## Implementation Comparison

| Feature | Carbon(Haskell) | Rusty Carbon(Rust) |
|---------|----------------|-------------------|
| **Language** | Haskell | Rust |
| **Build Tool** | GHC + Make | Cargo |
| **Dependencies** | Alex, Happy | anyhow, clap, rustyline, regex |
| **Performance** | Good (lazy evaluation) | Excellent (zero-cost abstractions) |
| **Memory Safety** | Garbage collected | Memory safe without GC |
| **Build Speed** | Slower (GHC compilation) | Faster (incremental compilation) |
| **Portability** | Requires GHC runtime | Self-contained binary |
| **Development** | Functional paradigm | Systems programming approach |

### Carbon(Haskell) - Original Implementation

Located in `Carbon-Haskell/`

**Advantages:**
- **Functional Foundation**: Built with Haskell's strong functional programming principles
- **Pattern Matching**: Natural implementation of Carbon's pattern matching using Haskell's native patterns
- **Type Safety**: Leverages Haskell's type system for interpreter correctness
- **Mathematical Operations**: Excellent support for mathematical computations

**Architecture:**
- `Lexer.x` - Alex-based lexical analyzer
- `Parser.y` - Happy-based parser generator
- `Evaluator.hs` - Pure functional evaluation engine
- `Main.hs` - REPL and file execution

**Building:**
```bash
cd Carbon-Haskell
make            # Build interpreter
make clean      # Clean build artifacts
make install    # Build and install to bin/
```

### Rusty Carbon(Rust) - Modern Reimplementation

Located in `Carbon-Rust/`

**Advantages:**
- **Performance**: Zero-cost abstractions and efficient memory management
- **Modern Tooling**: Cargo package manager, integrated testing, documentation
- **Safety**: Memory safety without garbage collection
- **Ecosystem**: Access to crates.io ecosystem
- **Error Handling**: Superior error messages with colored output

**Architecture:**
- `lexer.rs` - Hand-written lexer with regex support
- `parser.rs` - Recursive descent parser
- `evaluator.rs` - Environment-based evaluator
- `error.rs` - Comprehensive error handling
- `value.rs` - Value type system
- `ast.rs` - Abstract syntax tree definitions

**Building:**
```bash
cd Carbon-Rust
cargo build         # Debug build
cargo build --release  # Optimized build
cargo run           # Run REPL
cargo run file.cb   # Run Carbon file
cargo test          # Run tests
```

## Usage

### Interactive REPL

Both implementations provide an interactive REPL:

```bash
# Haskell version
cd Carbon-Haskell && bin/carbon

# Rust version
cd Carbon-Rust && cargo run
```

### Running Carbon Files

```bash
# Haskell version
cd Carbon-Haskell && bin/carbon examples/demo.cb

# Rust version
cd Carbon-Rust && cargo run examples/demo.cb
```

### Examples

Both implementations include the same set of example files in their `examples/` directories:

- `demo.cb` - Basic language features
- `pattern_matching.cb` - Pattern matching showcase
- `error_handling.cb` - Go-style error handling
- `if_expressions.cb` - Control flow examples
- `comprehensive_demo.cb` - Full feature demonstration

## Development Status

### ✅ Implemented Features
- Basic data types and operations
- Variable declarations and scoping
- Control flow (if expressions, loops)
- Arrow functions
- Pattern matching
- Error handling
- Arrays and array operations
- Mathematical constants and functions

### 🚧 In Development
- Function declarations with type annotations
- Object-oriented programming features
- More advanced pattern matching
- Module system
- Standard library expansion

### 🎯 Future Goals
- Static type checking
- Compile-time optimizations
- Package management
- IDE integration
- WebAssembly compilation target

<!-- ## Contributing -->
<!---->
<!-- 1. Choose your preferred implementation (Haskell or Rust) -->
<!-- 2. Fork the repository -->
<!-- 3. Create a feature branch -->
<!-- 4. Make your changes -->
<!-- 5. Add tests and examples -->
<!-- 6. Submit a pull request -->

## Language Philosophy

Carbon aims to be:
- **Familiar**: Syntax elements from popular languages
- **Safe**: Memory safety and explicit error handling
- **Expressive**: Powerful pattern matching and functional features
- **Practical**: Real-world applicability with good performance
- **Modern**: Contemporary language design principles

