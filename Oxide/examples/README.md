# Oxide Examples

This directory contains organized examples demonstrating Oxide language features.

## Quick Start

```bash
# Run the main demo showcasing all features
cargo run examples/demo.ox

# Run the comprehensive do-while example
cargo run examples/do_while_working.ox

# Run the automatic methods demo
cargo run examples/automatic_methods_demo.ox

# Run the tuple and pattern matching demo
cargo run examples/tuple_guard_demo.ox

# Run the new features demo
cargo run examples/working_features_demo.ox
```

## Directory Structure

```
examples/
├── demo.ox                      # Main comprehensive demo
├── do_while_working.ox          # Do-while loop examples
├── automatic_methods_demo.ox    # OOP with automatic method generation
├── tuple_guard_demo.ox          # Advanced pattern matching with tuples
├── working_features_demo.ox     # New features: bitwise, conversions, sum, inheritance
├── basic_examples/              # Core language features
├── language_features/           # Advanced language features
├── data_structures/             # Built-in data structures
├── higher_order_functions/      # Functional programming
├── stdlib_tests/                # Standard library tests
├── demos/                       # Feature demonstrations
└── debugging/                   # Debug examples and known issues
```

## Key Examples

### Core Features
- **`demo.ox`** - Comprehensive showcase of all major features
- **`basic_examples/comprehensive_new_syntax_demo.ox`** - Complete syntax overview
- **`basic_examples/syntax_features_summary.ox`** - Syntax feature summary

### Object-Oriented Programming
- **`automatic_methods_demo.ox`** - Automatic getter/setter generation
- **`language_features/enhanced_classes.ox`** - Advanced class features

### Functional Programming
- **`higher_order_functions/`** - Map, filter, fold, compose examples
- **`basic_examples/test_haskell_style_map.ox`** - Haskell-style function composition

### Pattern Matching
- **`tuple_guard_demo.ox`** - Advanced tuple patterns with guards
- **`language_features/pattern_matching/`** - Comprehensive pattern matching examples

### Data Structures
- **`data_structures/`** - ArrayList, Set, Map, Stack, Queue examples
- **`basic_examples/test_map_vs_Map.ox`** - Map function vs Map data structure

## Running Examples

```bash
# Basic syntax and features
cargo run examples/basic_examples/comprehensive_new_syntax_demo.ox

# Pattern matching
cargo run examples/language_features/pattern_matching/basic_match.ox

# Data structures
cargo run examples/data_structures/arraylist_demo.ox

# Higher-order functions
cargo run examples/higher_order_functions/map_filter_fold.ox

# Standard library
cargo run examples/stdlib_tests/math_functions.ox
```

## Features Demonstrated

- ✅ Variable declarations (`var`, `let`, typed)
- ✅ Arithmetic and compound operators
- ✅ Control flow (if expressions, for loops)
- ✅ Functions and arrow functions
- ✅ Pattern matching with guards
- ✅ Object-oriented programming
- ✅ Automatic method generation
- ✅ Data structures (ArrayList, Set, Map, etc.)
- ✅ Higher-order functions
- ✅ Error handling
- ✅ String and math functions
- ✅ Type checking and conversions
- ✅ Bitwise operations (`&`, `|`, `^`, `~`, `<<`, `>>`)
- ✅ Number base conversions (`toBinary`, `toHex`, `toDecimal`)
- ✅ Sum function for data structures
- ✅ Basic inheritance with `extends` keyword
