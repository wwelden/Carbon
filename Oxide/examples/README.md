# Oxide Programming Language Examples

This directory contains 10 comprehensive examples showcasing all major features of the Oxide programming language.

## Quick Start

Run any example with:
```bash
cargo run examples/[filename].ox
```

## Examples Overview

### 1. **01_basics.ox** - Language Fundamentals
- Variables and types (`let`, `var`, `int`, `string`, `bool`)
- Arithmetic and logical operators
- Control flow (`if`, `for`, `while`)
- String operations

### 2. **02_functions.ox** - Functions and Multiple Returns
- Basic function definitions
- **Go-style multiple return values**: `fn name() (type1, type2)`
- Multiple assignment: `let a, b := function_call()`
- Error handling patterns

### 3. **03_data_structures.ox** - Built-in Data Types
- Arrays: `[1, 2, 3]`
- ArrayList: `ArrayList[1, 2, 3]`
- Set: `Set[1, 2, 3]` (duplicates removed)
- Map: `Map["key" -> value]`
- Stack: `Stack[1, 2, 3]` (LIFO)
- Queue: `Queue[1, 2, 3]` (FIFO)
- **Sum function**: Works on all data structures

### 4. **04_classes_oop.ox** - Object-Oriented Programming
- Class definitions and inheritance (`extends`)
- Automatic getter/setter generation
- Method definitions
- Class composition patterns

### 5. **05_bitwise_operations.ox** - Bitwise and Number Systems
- Bitwise operators: `&`, `|`, `^`, `~`, `<<`, `>>`
- Number base conversions: `toBinary()`, `toHex()`, `toDecimal()`
- Bit manipulation patterns
- Advanced bit operations

### 6. **06_pattern_matching.ox** - Pattern Matching and Tuples
- Match expressions with guards
- Tuple creation and destructuring: `(a, b, c)`
- Tuple access: `tuple.0`, `tuple.1`
- Complex pattern matching

### 7. **07_higher_order_functions.ox** - Functional Programming
- Higher-order functions: `map()`, `filter()`, `fold()`
- Function composition
- Functional programming patterns
- Custom higher-order functions

### 8. **08_pointers_memory.ox** - Pointers and Memory
- Pointer creation: `&variable`
- Dereferencing: `*pointer`
- Null pointer safety
- Memory management patterns

### 9. **09_advanced_features.ox** - Advanced Language Features
- Do-while loops
- Complex if expressions
- Advanced pattern matching
- Mathematical constants (`PI`, `E`, `PHI`, `AVOGADRO`)
- Recursive functions

### 10. **10_comprehensive_demo.ox** - Complete Feature Showcase
- All major features in one cohesive example
- Student management system demonstration
- Real-world usage patterns

## Key Features Demonstrated

### ✅ **Go-Style Multiple Return Values**
```oxide
fn divmod(int a, int b) (int, int) {
    return a / b, a % b;
}

let quotient, remainder := divmod(17, 5);
```

### ✅ **Universal Sum Function**
```oxide
let array = [1, 2, 3];
let list = ArrayList[4, 5, 6];
let set = Set[7, 8, 9];

print(toString(sum(array)));  // Works on all data structures
print(toString(list.sum));    // Method syntax also available
```

### ✅ **Object-Oriented Programming**
```oxide
class Animal {
    name
    fn makeSound() { print(name + " makes a sound"); }
}

class Dog extends Animal {
    breed
    fn bark() { print(name + " barks!"); }
}
```

### ✅ **Pattern Matching with Tuples**
```oxide
fn analyzePoint(tuple point) string {
    return match point {
        (0, 0) => "Origin",
        (x, 0) => "On X-axis",
        (x, y) if x == y => "Diagonal",
        _ => "Other location"
    };
}
```

### ✅ **Bitwise Operations**
```oxide
let a = 12;  // 1100 in binary
let b = 10;  // 1010 in binary
print(toString(a & b));      // Bitwise AND
print(toBinary(a << 1));     // Left shift to binary
```

### ✅ **Higher-Order Functions**
```oxide
fn square(int x) int { return x * x; }
let numbers = [1, 2, 3, 4, 5];
let squared = map(numbers, square);
```

### ✅ **Safe Pointer Operations**
```oxide
let value = 42;
let ptr = &value;
*ptr = 100;  // Modify through pointer
```

## Running Examples

```bash
# Run the comprehensive demo (recommended first)
cargo run examples/10_comprehensive_demo.ox

# Run specific feature examples
cargo run examples/02_functions.ox          # Multiple return values
cargo run examples/03_data_structures.ox    # Data structures and sum
cargo run examples/04_classes_oop.ox        # Object-oriented programming
cargo run examples/06_pattern_matching.ox   # Pattern matching and tuples

# Run all examples in order
for i in {01..10}; do
    echo "Running example $i..."
    cargo run examples/${i}_*.ox
    echo "---"
done
```

## Language Features Summary

- **Type System**: `int`, `string`, `bool`, `real`, arrays, custom classes
- **Functions**: Single and multiple return values, Go-style syntax
- **Data Structures**: Array, ArrayList, Set, Map, Stack, Queue with universal `sum()`
- **OOP**: Classes, inheritance, automatic getters/setters
- **Pattern Matching**: Match expressions, guards, tuple destructuring
- **Functional**: Higher-order functions, map/filter/fold
- **Systems**: Pointers, memory management, bitwise operations
- **Advanced**: Do-while loops, mathematical constants, recursion

Oxide combines the best of modern programming languages with safety, expressiveness, and performance.
