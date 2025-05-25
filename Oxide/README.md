# Oxide Programming Language
(an Extention of carbon implemented in Rust)

Oxide is a modern programming language that amalgamates syntax from Go, JavaScript, Rust, Haskell, C#, Java, and C. It features clean, expressive syntax with powerful functional programming capabilities including Haskell-inspired pattern matching.

## 🚀 Quick Start

```bash
# Build Oxide
make

# Run the REPL
bin/oxide

# Try pattern matching
> match 42 { 42 => "Hello Oxide!", _ => "not found" };
"Hello Oxide!"
```

## Features

### ✅ Working Features

- **Basic Data Types**: integers, floats, booleans, strings, null, arrays
- **Arithmetic Operations**: `+`, `-`, `*`, `/`, `%`, `**` (exponentiation)
- **Comparison Operations**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Boolean Operations**: `&&`, `||`, `!`
- **Variable Declarations**: Multiple syntax styles for flexible coding
  - **Type inference (mutable)**: `var x = 42` - automatic type detection with mutability
  - **Type inference (immutable)**: `let x = 42` - automatic type detection, immutable
  - **Typed constants**: `let int x = 42` - explicitly typed immutable variables
  - **Typed variables**: `int x = 42` - explicitly typed mutable variables (original syntax)
- **If Expressions**: `if condition { then_value } else { else_value }` (Go/Python style - no parentheses!)
- **Ternary Operator**: `condition ? value1 : value2` (C/JavaScript style)
- **Arrow Functions**: Multiple parameter support for enhanced functional programming
  - **Single parameter**: `x => x + 1` (JavaScript style)
  - **Multi-parameter**: `(x, y) => x + y` - supports multiple parameters for complex operations
  - **Complex expressions**: `(a, b, c) => a * b * c` - works with higher-order functions
- **For-In Loops**: `for item in array { ... }` (Go style)
- **While-Style Loops**: `for condition { ... }` (Go-style unified for loops)
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
  - **Pattern Guards**: `match x { y if y > 10 => "big", _ => "small" }` - conditional pattern matching
- **Go-Style Error Handling**: Explicit error values and checking (no exceptions)
  - Error creation: `err("something went wrong")`
  - Error checking: `isErr(value)` returns boolean
  - Tuples for multiple returns: `(result, error)` pattern
  - Explicit error handling: `if (isErr(result)) { ... } else { ... }`
- **Compound Assignment Operators**: `+=`, `-=`, `*=`, `/=`, `%=` for concise variable updates
- **Increment/Decrement Operators**: `++` and `--` for convenient counting operations
- **Function Declarations with Type Annotations**:
  - Full function syntax: `fn functionName(type param) -> returnType { ... }`
  - Parameter type annotations: `int`, `string`, `bool`, `float`
  - Return type annotations with `->` syntax
  - Multi-parameter functions with typed parameters
- **Object-Oriented Programming**:
  - Class declarations: `class ClassName { ... }`
  - Field declarations within classes
  - Method declarations with parameters
  - Object instantiation: `new ClassName()`
  - Method calls: `object.method(args)`
  - `this` keyword for accessing object fields and methods
  - Constructor-like `init` methods
  - **Automatic Method Generation**: Oxide automatically generates getters, setters, and init methods
    - **Auto Getters**: `getFieldName()` - returns field value (e.g., `getName()`, `getAge()`)
    - **Auto Setters**: `setFieldName(value)` - sets field and returns value (e.g., `setName("John")`)
    - **Auto Init**: `init(value)` - initialization method for objects
    - **Manual Override**: User-defined methods take precedence over automatic ones
    - **Zero-argument Support**: Getters work without parameters: `obj.getName()`
- **Advanced Data Structures**: Built-in collection types with methods
  - **ArrayList**: `ArrayList[1, 2, 3]` with `push()`, `pop()`, `get()`, `set()` methods
  - **Set**: `Set{"item1", "item2"}` with `add()`, `contains()`, `remove()` methods
  - **Map/Dictionary**: `Map{"key": "value"}` with `put()`, `get()`, `containsKey()` methods
  - **Stack**: `Stack[1, 2, 3]` with `push()`, `pop()`, `peek()` methods (LIFO)
  - **Queue**: `Queue["first", "second"]` with `enqueue()`, `dequeue()`, `front()` methods (FIFO)
  - **LinkedList**: `LinkedList[1, 2, 3]` with `prepend()`, `head()` methods
  - **Universal Methods**: All collections support `size()`, `isEmpty()` methods
- **Higher-Order Functions**: Comprehensive functional programming support
  - **map**: `map(fn)(array)` - Transform each element
  - **filter**: `filter(predicate)(array)` - Select elements matching condition
  - **fold/reduce**: `fold(fn)(initial)(array)` - Accumulate values
  - **forEach**: `forEach(fn)(array)` - Execute function for each element (side effects)
  - **find**: `find(predicate)(array)` - Find first matching element
  - **any/some**: `any(predicate)(array)` - Check if any element matches
  - **all/every**: `all(predicate)(array)` - Check if all elements match
  - **compose**: `compose(f)(g)` - Function composition `f(g(x))`
  - **zip**: `zip(array1)(array2)` - Combine arrays into tuples
  - **flatMap**: `flatMap(fn)(array)` - Map and flatten results
  - **zipWith**: `zipWith(fn)(array1)(array2)` - Combine with custom function
  - **concatMap**: Alias for `flatMap`
- **Math Functions**: Comprehensive mathematical operations
  - **abs**: `abs(number)` - Absolute value
  - **floor**: `floor(number)` - Round down to nearest integer
  - **ceil**: `ceil(number)` - Round up to nearest integer
  - **round**: `round(number)` - Round to nearest integer
  - **sqrt**: `sqrt(number)` - Square root
  - **pow**: `pow(base)(exponent)` - Power function
  - **sin/cos/tan**: `sin(number)` - Trigonometric functions
  - **log/log10**: `log(number)` - Natural and base-10 logarithms
  - **exp**: `exp(number)` - Exponential function
- **String Functions**: Powerful string manipulation
  - **length**: `length(string)` - String length
  - **toUpperCase/toLowerCase**: `toUpperCase(string)` - Case conversion
  - **trim**: `trim(string)` - Remove whitespace
  - **split**: `split(string)(delimiter)` - Split string into array
  - **join**: `join(delimiter)(array)` - Join array into string
  - **substring**: `substring(string)(start)(end)` - Extract substring
  - **indexOf**: `indexOf(string)(substring)` - Find substring position
  - **replace**: `replace(string)(old)(new)` - Replace occurrences
  - **startsWith/endsWith**: `startsWith(string)(prefix)` - String tests
- **Array Utilities**: Additional array manipulation functions
  - **empty**: `empty(array)` - Check if array is empty
  - **concat**: `concat(array1)(array2)` - Concatenate arrays
  - **flatten**: `flatten(array)` - Flatten nested arrays
  - **includes**: `includes(array)(element)` - Check array membership
  - **slice**: `slice(array)(start)(end)` - Extract array slice
- **Type Checking**: Runtime type inspection
  - **isNumber/isString/isBool**: `isNumber(value)` - Type predicates
  - **isArray/isFunction/isNull/isObject**: Advanced type checking
- **Random Functions**: Pseudo-random number generation
  - **random**: `random()` - Random float between 0 and 1
  - **randomInt**: `randomInt(max)` - Random integer from 0 to max-1
- **Conversion Functions**: Type conversion utilities
  - **toInt/toFloat/toBool**: `toInt(value)` - Convert between types

### ✅ Recently Implemented

- **Bitwise Operators**: `&`, `|`, `^`, `~`, `<<`, `>>` for bit manipulation
- **Binary/Hex/Decimal Conversions**: `toBinary()`, `toHex()`, `toDecimal()` functions
- **Sum Function**: `sum()` function that sums numeric data structures
- **Basic Inheritance**: `class Child extends Parent` syntax with method overriding
- **Basic Generics**: Generic type parameters (T, U, V) for functions
- **Advanced Pointer System**: Comprehensive pointer support with safety features
  - **Pointer Operations**: Address-of (`&`) and dereference (`*`) operators
  - **Pointer Assignment**: `*ptr = value` syntax for indirect assignment
  - **Pointer Arithmetic**: `ptr + offset`, `ptr - offset` operations
  - **Pointer Comparison**: `ptr1 == ptr2`, `ptr1 != ptr2` equality checks
  - **Null Pointers**: `nullptr` literal with safe error handling
  - **Type Safety**: Proper type checking and runtime error prevention

### 🚧 In Development

- Module system and imports
- Performance optimizations
- Do while loops
- Advanced generics with constraints
- True pointer modification (reference semantics)


## Directory Structure

```
Oxide/
├── src/           # Rust source code (lexer, parser, evaluator)
├── examples/      # Oxide code examples and demos
│   ├── basic_examples/          # Basic language features
│   ├── stdlib_tests/           # Standard library tests
│   ├── language_features/      # Advanced language features
│   ├── data_structures/        # Data structure examples
│   ├── higher_order_functions/ # Functional programming
│   └── demos/                  # Feature demonstrations
├── docs/          # Documentation
│   └── SYNTAX_GUIDE.md        # Comprehensive syntax reference
├── target/        # Rust build artifacts
├── Cargo.toml     # Rust project configuration
└── README.md      # This file
```

## Building

This project uses Rust and Cargo for building:

```bash
# Build Oxide interpreter
cargo build

# Build with optimizations (release mode)
cargo build --release

# Run directly with Cargo
cargo run

# Run with a Oxide file
cargo run examples/demo.ox

# Clean build artifacts
cargo clean
```

## Usage

### Interactive REPL

```bash
# Run with Cargo
cargo run

# Or build first then run the binary
cargo build
./target/debug/oxide
```

### Running Oxide Files

```bash
# Run an Oxide file directly
cargo run examples/demo.ox

# Or use the compiled binary
./target/debug/oxide examples/demo.ox
```

### Running Examples

Explore the organized `examples/` directory for comprehensive Oxide code samples:

```bash
# View organized examples
ls examples/

# Basic language features
cargo run examples/basic_examples/new_syntax_test.ox
cargo run examples/basic_examples/comprehensive_new_syntax_demo.ox
cargo run examples/basic_examples/syntax_features_summary.ox

# Automatic method generation
cargo run examples/auto_methods_simple.ox
cargo run examples/test_auto_methods.ox

# Standard library tests
cargo run examples/stdlib_tests/

# Language features
cargo run examples/language_features/

# Data structures
cargo run examples/data_structures/

# Functional programming
cargo run examples/higher_order_functions/

# Feature demonstrations
cargo run examples/demos/

# Key examples:
# - basic_examples/new_syntax_test.ox           # New syntax features demo
# - basic_examples/comprehensive_new_syntax_demo.ox  # Complete new syntax showcase
# - auto_methods_simple.ox                     # Automatic getter/setter demo
# - test_auto_methods.ox                       # Comprehensive automatic methods test
# - stdlib_tests/                              # Standard library comprehensive tests
# - higher_order_functions/                    # Functional programming examples
# - data_structures/                           # Advanced data structures
# - language_features/                         # Pattern matching, OOP, etc.
# - demos/                                     # Feature demonstrations
```

### Example Code

```oxide
// Variable declarations (multiple syntaxes)
var x = 42;           // Type inferred, mutable
let y = 10;           // Type inferred, immutable
let int z = 5;        // Explicitly typed constant
int w = 15;           // Explicitly typed variable
string message = "Hello Oxide";

// Multi-parameter lambda functions
let add = (x, y) => x + y;
let multiply = (a, b, c) => a * b * c;

// Works with higher-order functions
let numbers = [1, 2, 3, 4, 5];
let sum = fold((acc, x) => acc + x)(0)(numbers);

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

// While-style loops (Go syntax)
let counter = 5;
for counter > 0 {
    print(counter);
    counter--;
}

// Function declarations with type annotations
fn factorial(int n) -> int {
    let result = 1;
    let i = 1;
    for i <= n {
        result *= i;
        i++;
    }
    return result;
}

// Object-oriented programming with automatic methods
class Person {
    name
    age
}

let person = new Person();

// Automatic getters (generated automatically)
print(person.getName());     // null (initial value)
print(person.getAge());      // null (initial value)

// Automatic setters (generated automatically)
person.setName("Alice");     // Returns "Alice"
person.setAge(30);           // Returns 30

// Verify the values were set
print(person.getName());     // "Alice"
print(person.getAge());      // 30

// Automatic init method
person.init("Bob");          // Returns the object

// Custom class with manual methods
class Calculator {
    value

    // Manual methods override automatic ones
    getValue() {
        return this.value || 0;
    }

    add(int x) {
        this.value += x;
        return this;
    }
}

let calc = new Calculator();
// Automatic setValue() still available, but getValue() uses manual method
calc.setValue(10);
calc.add(5);
print(calc.getValue());  // 15

// Advanced data structures
let list = ArrayList[1, 2, 3];
list.push(4);
print(list.size());  // 4

let fruits = Set{"apple", "banana", "cherry"};
fruits.add("orange");
print(fruits.contains("apple"));  // true

let person = Map{"name": "Alice", "age": 30};
person.put("city", "New York");
print(person.get("name"));  // "Alice"

let stack = Stack[1, 2, 3];
stack.push(4);
print(stack.pop());  // 4

// Higher-order functions
let evens = filter(x => x % 2 == 0)(numbers);
let doubled = map(x => x * 2)(evens);
let sum = fold((x, y) => x + y)(0)(doubled);

print(evens);    // [2, 4]
print(doubled);  // [4, 8]
print(sum);      // 12

// Function composition
let addOne = x => x + 1;
let multiplyByTwo = x => x * 2;
let composed = compose(multiplyByTwo)(addOne);
print(composed(5));  // 12 (5 + 1) * 2

// Math functions
print(abs(-5));        // 5
print(sqrt(16));       // 4
print(pow(2)(8));      // 256
print(sin(PI / 2));    // 1
print(log(E));         // 1

// String functions
let text = "  Hello World  ";
print(trim(text));                    // "Hello World"
print(toUpperCase(text));             // "  HELLO WORLD  "
let words = split("a,b,c")(",");      // ["a", "b", "c"]
print(join("-")(words));              // "a-b-c"
print(substring("Hello")(1)(4));      // "ell"
print(indexOf("Hello")("ll"));        // 2
print(replace("Hello")("l")("x"));    // "Hexxo"

// Array utilities
let arr1 = [1, 2, 3];
let arr2 = [4, 5, 6];
print(concat(arr1)(arr2));            // [1, 2, 3, 4, 5, 6]
print(flatten([[1, 2], [3, 4]]));     // [1, 2, 3, 4]
print(includes(arr1)(2));             // true
print(slice(arr1)(0)(2));             // [1, 2]

// Type checking
print(isNumber(42));                  // true
print(isString("hello"));             // true
print(isArray([1, 2, 3]));           // true

// Random and conversion
print(random());                      // random float 0-1
print(randomInt(100)(0));             // random int 0-99
print(toInt("42"));                   // 42
print(toBool(""));                    // false

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

// Complex pattern matching with guards
match [1, x, 3] {
    [1, 2, 3] => "exact match",
    [1, y, 3] if y > 5 => "large value",  // Guard condition
    [1, y, 3] => y * 100,  // Binds y=2, returns 200
    _ => 0
};

// Guard patterns for conditional matching
match 42 {
    x if x > 50 => "big number",
    x if x > 10 => "medium number",
    _ => "small number"
};  // "medium number"

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

// Advanced pointer operations
let data = [1, 2, 3, 4, 5];
let ptr = &data;

// Pointer arithmetic
let next_ptr = ptr + 1;
let prev_ptr = ptr - 1;

// Pointer comparison
let same_ptr = &data;
print(toString(ptr == same_ptr));  // true (same pointed value)

// Null pointer safety
let nullPtr = nullptr;
print(typeof(nullPtr));            // "null pointer"
print(toString(nullPtr == nullptr)); // true

// Safe error handling
// print(toString(*nullPtr));      // Error: Cannot dereference null pointer

// Pointer assignment syntax (conceptual)
let value = 42;
let valuePtr = &value;
*valuePtr = 100;  // Syntax supported, but doesn't modify original
```

## Language Design

Oxide combines the best features from multiple languages:

- **Go**: Simple, clean syntax for control flow (`for item in array`)
- **JavaScript**: Dynamic typing, arrow functions (`x => x + 1`), ternary operator
- **Rust**: Modern pattern matching syntax (`match value { ... }`)
- **Haskell**: Pattern matching semantics (destructuring, variable binding)
- **C/Java**: Basic syntax structure and familiar operators
- **C#**: Object-oriented features (planned)
- **C**: Low-level control (planned)

## Implementation

Oxide is implemented in Rust using:
- **Hand-written lexer** for tokenization (`src/lexer.rs`)
- **Recursive descent parser** for syntax analysis (`src/parser.rs`)
- **Tree-walking evaluator** with environment-based evaluation (`src/evaluator.rs`)
- **Command-line interface** with REPL support (`src/main.rs`)
- **Comprehensive error handling** with detailed error messages (`src/error.rs`)
- **Value system** supporting all Oxide data types (`src/value.rs`)
- **Abstract syntax tree** definitions (`src/ast.rs`)

**Key Features**: Memory-safe implementation, fast compilation, zero parser conflicts, comprehensive pattern matching support.

## Documentation

For detailed syntax reference and language features, see:
- **[Syntax Guide](docs/SYNTAX_GUIDE.md)** - Comprehensive language syntax reference
- **[Examples](examples/)** - Organized code examples by category
- **[Basic Examples](examples/basic_examples/)** - New syntax features and basic usage
- **[Standard Library Tests](examples/stdlib_tests/)** - Complete stdlib functionality tests

## Contributing

The Oxide language is actively being developed. Current priorities:

1. **~~Enhanced Pattern Matching~~**: ✅ **COMPLETED** - Pattern guards, nested patterns, and complex expressions
2. **Module System**: Import/export functionality for code organization
3. **~~Standard Library Expansion~~**: ✅ **COMPLETED** - Comprehensive stdlib with data structures, higher-order functions, math, strings, arrays, type checking, and utilities
4. **Performance Optimizations**: Bytecode compilation and runtime improvements
5. **~~Multi-Parameter Arrow Functions~~**: ✅ **COMPLETED** - Support for `(x, y) => x + y` syntax
6. **Generic Types**: Type parameters for functions and data structures
7. **Advanced Error Handling**: Result types and advanced error propagation
8. **Concurrency Features**: Async/await and parallel processing

### Development Workflow

```bash
# Make changes to src/ Rust files
# Test your changes
cargo run examples/demo.ox

# Run the REPL
cargo run

# Build optimized version
cargo build --release
```

## Recent Accomplishments

- ✅ **NEW: Automatic Method Generation**: Oxide now automatically generates getters, setters, and init methods for all class fields
  - Auto-generated `getFieldName()` getters that return field values
  - Auto-generated `setFieldName(value)` setters that set and return values
  - Auto-generated `init(value)` method for object initialization
  - Manual methods take precedence over automatic ones
  - Zero-argument method calls now supported (`obj.getName()`)
  - Comprehensive test suite with 8 tests validating all automatic method features
- ✅ **COMPLETED: Comprehensive Standard Library**: Oxide now has a complete standard library implementation
  - Advanced data structures (ArrayList, Set, Map, Stack, Queue, LinkedList)
  - Complete higher-order functional programming suite (map, filter, fold, compose, zip, etc.)
  - Mathematical operations (trigonometric, logarithmic, power functions)
  - String processing (case conversion, trimming, splitting, joining, substring operations)
  - Array utilities (concatenation, flattening, slicing, membership testing)
  - Runtime type checking and conversion functions
  - Random number generation and comprehensive type conversions
- ✅ **NEW: Enhanced Variable Declarations**: Added support for `var` (mutable type inference), `let` (immutable type inference), and `let type` (typed constants)
- ✅ **NEW: Multi-Parameter Lambda Functions**: Full support for `(x, y) => x + y` syntax with seamless higher-order function integration
- ✅ **NEW: Comprehensive Documentation**: Organized examples directory and detailed syntax guide in `docs/SYNTAX_GUIDE.md`
- ✅ **Complete Rust Conversion**: Successfully converted from Haskell to Rust implementation
- ✅ **Memory Safety**: All Oxide code now runs with Rust's memory safety guarantees
- ✅ **Zero Parser Conflicts**: Hand-written recursive descent parser with no ambiguities
- ✅ **Fast Compilation**: Rust builds much faster than the original Haskell implementation
- ✅ **Object-Oriented Programming**: Full class system with fields, methods, and inheritance
- ✅ **Function Declarations**: Type-annotated functions with `fn name(type param) -> returnType` syntax
- ✅ **Advanced Data Structures**: ArrayList, Set, Map, Stack, Queue, LinkedList with comprehensive methods
- ✅ **Higher-Order Functions**: Complete functional programming suite (map, filter, fold, compose, etc.)
- ✅ **Enhanced Loop Constructs**: Go-style unified `for` syntax for both iteration and while-loops
- ✅ **Comprehensive Math Library**: Trigonometric, logarithmic, and power functions (abs, sqrt, sin, cos, tan, log, exp, pow)
- ✅ **Advanced String Processing**: String manipulation with split, join, substring, replace, case conversion
- ✅ **Array Utilities**: Additional array operations (concat, flatten, slice, includes, indexOf)
- ✅ **Type Checking System**: Runtime type inspection (isNumber, isString, isArray, etc.)
- ✅ **Random Number Generation**: Pseudo-random functions for numbers and integers
- ✅ **Type Conversion Functions**: Robust conversion between types (toInt, toFloat, toBool)
- ✅ **Compound Assignment Operators**: `+=`, `-=`, `*=`, `/=`, `%=` for concise variable updates
- ✅ **Increment/Decrement Operators**: `++` and `--` for convenient counting operations
- ✅ **Go-Style Error Handling**: Explicit error values with `err()` and `isErr()` functions
- ✅ **Tuples**: Multi-value returns with `(value1, value2, ...)` syntax
- ✅ **Pattern Matching**: Full implementation with literal, variable, array, wildcard, and guard patterns
- ✅ **Arrow Functions**: JavaScript-style `x => x + 1` syntax
- ✅ **For-In Loops**: Go-style `for item in array { ... }`
- ✅ **Logical Operators**: Fixed `||` and `&&` tokenization and evaluation
- ✅ **Mathematical Constants**: `PI`, `E`, `PHI`, `AVOGADRO` support
- ✅ **Method Chaining**: Support for fluent interfaces and chained method calls
- ✅ **REPL Support**: Interactive development environment with history
- ✅ **Comprehensive Examples**: Working demonstration files showcasing all features

## License

[Add your license here]
