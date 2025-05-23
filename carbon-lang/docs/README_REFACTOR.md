# Programming Language Refactor: British → Carbon

This repository contains a complete refactor of the original "British" programming language, transforming its esoteric British terminology into Carbon - a modern language with familiar JavaScript-like syntax while preserving 100% of the original functionality.

## 🎯 What Changed

### Before (British Syntax)
```british
-- Creating a counter class
noble Counter {
    serfs count innit
    decree increment (X) is oneself.count = oneself.count + 1 innit
    decree get (X) is oneself.count innit
} innit

-- Using the class
hearye MYCOUNTER is a Counter(0) innit
MYCOUNTER.get(MYCOUNTER) innit
MYCOUNTER.increment(MYCOUNTER) innit
MYCOUNTER.get(MYCOUNTER) innit
```

### After (Carbon Syntax)
```carbon
// Creating a counter class
class Counter {
    count;
    increment() { return this.count = this.count + 1; }
    get() { return this.count; }
}

// Using the class
let myCounter = new Counter(0);
myCounter.get();
myCounter.increment();
myCounter.get();
```

## 🚀 Quick Start

### Build the Carbon Version
```bash
make -f Makefile_JS
./carbon
```

### Build the Original British Version
```bash
make
./british
```

## 📋 Feature Comparison

| Feature | British | Carbon | Status |
|---------|---------|----------------|--------|
| Variables | `hearye x is 42 innit` | `let x = 42;` | ✅ |
| Booleans | `ace`, `rank` | `true`, `false` | ✅ |
| Classes | `noble Counter { ... } innit` | `class Counter { ... }` | ✅ |
| Methods | `decree get (X) is ... innit` | `get() { return ...; }` | ✅ |
| Objects | `a Counter(10)` | `new Counter(10)` | ✅ |
| Functions | `cheers X => X * 2` | `(x) => x * 2` | ✅ |
| Conditionals | `supposing ... hence ... otherwise` | `if (...) { ... } else { ... }` | ✅ |
| Comments | `-- Comment` | `// Comment` | ✅ |

## 🎉 Benefits

1. **Familiar Syntax**: No need to learn esoteric British terms
2. **Better Tooling**: Standard syntax works with existing IDEs
3. **Easier Onboarding**: Developers can start immediately
4. **All Features Preserved**: Every capability maintained
5. **Modern Conventions**: Follows industry standards

## 📖 Documentation

- [`JAVASCRIPT_LANGUAGE_SPECIFICATION.md`](JAVASCRIPT_LANGUAGE_SPECIFICATION.md) - Complete language specification
- [`JAVASCRIPT_IMPLEMENTATION_STATUS.md`](JAVASCRIPT_IMPLEMENTATION_STATUS.md) - Implementation status and migration guide
- [`BRITISH_LANGUAGE_SPECIFICATION.md`](BRITISH_LANGUAGE_SPECIFICATION.md) - Original language specification

## 🧪 Example Programs

### Basic Arithmetic
```carbon
let result = 2 + 3 * 4;  // 14
Math.sqrt(16);           // 4.0
```

### Higher-Order Functions
```carbon
let applyTwice = (f) => (x) => f(f(x));
let addOne = (x) => x + 1;
applyTwice(addOne)(5);  // 7
```

### Object-Oriented Programming
```carbon
class Calculator {
    value;

    add(x) {
        this.value = this.value + x;
        return this;
    }

    multiply(x) {
        this.value = this.value * x;
        return this;
    }

    getValue() {
        return this.value;
    }
}

let calc = new Calculator(10);
calc.add(5).multiply(2).getValue();  // 30
```

## 🔄 Migration

The refactor maintains complete backward compatibility in terms of functionality. All programs that worked in the British syntax can be directly translated to Carbon syntax using the mapping table in the implementation status document.

## 🏗️ Architecture

Both versions share the same underlying architecture:
- **Lexer**: Tokenizes the source code
- **Parser**: Builds an abstract syntax tree
- **Evaluator**: Interprets the AST
- **REPL**: Interactive shell for testing

The refactor only changes the surface syntax - the semantic model and evaluation remain identical.

---

**Result**: Carbon - a modern, approachable programming language that retains all the power of the original while being immediately familiar to developers.
