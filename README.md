# Carbon Programming Language

A modern programming language that amalgamates syntax from Go, JavaScript, Rust, Haskell, C#, Java, and C.

## 📁 Project Structure

The Carbon language implementation is located in the `carbon-lang/` directory:

```
carbon-lang/
├── src/           # Haskell implementation (lexer, parser, evaluator)
├── examples/      # Carbon code examples and demos
├── tests/         # Test files
├── docs/          # Documentation
├── bin/           # Compiled binary
└── README.md      # Complete documentation
```

## 🚀 Quick Start

```bash
cd carbon-lang
make
./bin/carbon
```

## ✨ Features

- **Pattern Matching**: Haskell-inspired with modern syntax
- **Arrow Functions**: JavaScript-style `x => x + 1`
- **For-In Loops**: Go-style `for item in array { ... }`
- **Ternary Operator**: C-style `condition ? value1 : value2`
- **Arrays & Primitives**: Dynamic typing with type checking

## 📖 Full Documentation

See [`carbon-lang/README.md`](carbon-lang/README.md) for complete documentation, examples, and usage instructions.

## 🎯 Pattern Matching Examples

```carbon
// Literal patterns
match 42 { 42 => "found", _ => "not found" }

// Array destructuring
match [1,2,3] { [a,b,c] => a+b+c, _ => 0 }

// Variable binding
match value { x => x * 2 }
```

---

**For the complete Carbon language experience, navigate to the [`carbon-lang/`](carbon-lang/) directory.**
