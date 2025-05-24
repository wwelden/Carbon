# Carbon Language Syntax Guide

This guide documents the correct syntax for Carbon language examples in this repository.

## Basic Syntax Rules

### 1. Semicolon Rules
Carbon generally does not require semicolons at the end of statements, with one important exception:

```carbon
// ✅ Correct - No semicolons for statements
let x = 42
print("Hello")
let result = if (x > 0) { "positive" } else { "non-positive" }

// ❌ Incorrect - Don't add semicolons to statements
let x = 42;
print("Hello");

// ✅ EXCEPTION: C-style for loops require semicolons as separators
for i = 0; i < arr.len; i++ {
    print(arr[i])
}

// ❌ Incorrect - Missing semicolons in C-style for loop
for i = 0 i < arr.len i++ {
    print(arr[i])
}
```

### 2. Variable Declarations
Carbon supports multiple variable declaration syntaxes:

```carbon
// ✅ Correct - Typed variables (mutable)
int x = 42
string name = "Carbon"
int[] arr = [1, 2, 3]
bool flag = true

// ✅ Correct - Type inference with mutability
var x = 42          // Inferred as int, mutable
var name = "Carbon"  // Inferred as string, mutable
var arr = [1, 2, 3] // Inferred as array, mutable

// ✅ Correct - Type inference (immutable)
let x = 42          // Inferred as int, immutable
let name = "Carbon" // Inferred as string, immutable

// ✅ Correct - Typed constants (immutable)
let int x = 42      // Explicitly typed constant
let string name = "Carbon"  // Explicitly typed constant

// ❌ Incorrect - These are not supported
array arr = [1, 2, 3]  // Use int[] instead
```

### 3. Function Declarations
Use the format `fn name(type param) returnType { ... }`:

```carbon
// ✅ Correct
fn add(int x, int y) int {
    return x + y
}

fn greet(string name) string {
    return "Hello " + name
}

// ❌ Incorrect
fn add(int x, int y) -> int {
    return x + y
}

function add(x, y) {
    return x + y
}
```

### 4. Lambda Functions
Use arrow syntax for lambda functions:

```carbon
// ✅ Correct - Single parameter
let double = x => x * 2

// ✅ Correct - Multiple expressions
let abs = x => {
    if (x >= 0) { x } else { -x }
}

// ✅ Correct - Curried functions
let add = x => y => x + y

// ✅ Correct - Multi-parameter lambdas (now supported!)
let add = (x, y) => x + y
let multiply = (a, b, c) => a * b * c

// Multi-parameter lambdas work with higher-order functions
let sum = fold((acc, x) => acc + x)(0)(numbers)
```

### 5. Control Flow
Standard C-style control flow without semicolons, except for C-style for loops:

```carbon
// ✅ Correct - If statements
if (condition) {
    // do something
} else {
    // do something else
}

// ✅ Correct - For-in loops (no semicolons)
for item in array {
    print(item)
}

// ✅ Correct - While-like for loops (no semicolons)
for i < 10 {
    print(i)
    i++
}

// ✅ Correct - C-style for loops (semicolons required)
for i = 0; i < arr.len; i++ {
    print(arr[i])
}
```

### 6. Arrays and Data Structures
JavaScript-style array syntax:

```carbon
// ✅ Correct
let numbers = [1, 2, 3, 4, 5]
let mixed = ["hello", 42, true]
let nested = [[1, 2], [3, 4]]

// Data structures
let list = ArrayList[1, 2, 3]
let set = Set{"apple", "banana"}
let map = Map{"key": "value"}
```

### 7. String Operations
Use built-in string functions:

```carbon
// ✅ Correct
let str = "Hello World"
let upper = toUpperCase(str)
let len = length(str)
let sub = substring(str)(0)(5)
```

### 8. Higher-Order Functions
Use curried function style:

```carbon
// ✅ Correct
let numbers = [1, 2, 3, 4, 5]
let doubled = map(x => x * 2)(numbers)
let filtered = filter(x => x > 3)(numbers)
let sum = fold(acc => x => acc + x)(0)(numbers)
```

### 9. Type Checking
Use built-in type checking functions:

```carbon
// ✅ Correct
isNumber(42)        // true
isString("hello")   // true
isArray([1, 2, 3])  // true
isBool(true)        // true
```

### 10. Comments
Use C-style comments:

```carbon
// Single line comment

/*
 * Multi-line comment
 */
```

## Standard Library Functions

### Math Functions
- `abs(x)` - Absolute value
- `floor(x)`, `ceil(x)`, `round(x)` - Rounding
- `sqrt(x)`, `pow(base)(exp)` - Power functions
- `sin(x)`, `cos(x)`, `tan(x)` - Trigonometric
- `log(x)`, `log10(x)`, `exp(x)` - Logarithmic

### String Functions
- `length(str)` - String length
- `toUpperCase(str)`, `toLowerCase(str)` - Case conversion
- `trim(str)` - Remove whitespace
- `split(delimiter)(str)` - Split string
- `join(delimiter)(array)` - Join array to string
- `substring(str)(start)(end)` - Extract substring
- `indexOf(str)(search)` - Find substring
- `replace(str)(old)(new)` - Replace substring
- `startsWith(str)(prefix)`, `endsWith(str)(suffix)` - String tests

### Array Functions
- `empty()` - Create empty array
- `concat(arr1)(arr2)` - Concatenate arrays
- `includes(arr)(item)` - Check if array contains item
- `slice(arr)(start)(end)` - Extract subarray
- `flatten(nestedArr)` - Flatten nested arrays
- `reverse(arr)` - Reverse array
- `unique(arr)` - Remove duplicates
- `take(n)(arr)` - Take first n elements
- `drop(n)(arr)` - Drop first n elements

### Higher-Order Functions
- `map(fn)(arr)` - Transform each element
- `filter(fn)(arr)` - Filter elements
- `fold(fn)(initial)(arr)` - Reduce array to single value

### Type Functions
- `isNumber(x)`, `isString(x)`, `isBool(x)`, `isArray(x)`, `isNull(x)`, `isFunction(x)`
- `toInt(x)`, `toFloat(x)`, `toBool(x)`, `toString(x)`

### Utility Functions
- `random()` - Random float 0-1
- `randomInt(max)` - Random integer 0 to max-1
- `print(x)` - Print to console

## File Organization

Examples are organized into directories:

- `basic_examples/` - Basic language features and simple examples
- `stdlib_tests/` - Standard library function tests
- `language_features/` - Advanced language features (patterns, loops, etc.)
- `data_structures/` - Data structure examples
- `higher_order_functions/` - Functional programming examples
- `demos/` - Comprehensive demonstrations

## Current Parser Limitations

1. Function syntax with arrows `fn name() -> type` should be `fn name() type`
2. `fn` function declarations appear to have parsing issues in current implementation
3. Complex pattern matching may have limited support
4. Some advanced type annotations may not be fully implemented

## Recently Added Features ✅

1. **Multi-parameter lambda functions** - `(x, y) => x + y` now fully supported
2. **Variable type inference** - `var x = 42` for mutable variables with type inference
3. **Immutable type inference** - `let x = 42` for immutable variables with type inference
4. **Typed constants** - `let int x = 42` for explicitly typed immutable variables

## Currently Working Syntax

Based on testing with the current Carbon-Rust implementation:

```carbon
// ✅ Working - All variable declaration types
int x = 42              // Typed mutable
string name = "Carbon"  // Typed mutable
int[] arr = [1, 2, 3]   // Typed array
bool flag = true        // Typed boolean

var x = 42             // Type inference mutable
var name = "Carbon"    // Type inference mutable

let x = 42             // Type inference immutable
let name = "Carbon"    // Type inference immutable

let int x = 42         // Typed immutable
let string name = "Carbon"  // Typed immutable

// ✅ Working - Lambda functions (all types)
let single = x => x * 2
let multi = (x, y) => x + y
let complex = (a, b, c) => a * b * c

// ✅ Working - Higher-order functions with lambdas
let doubled = map(x => x * 2)(numbers)
let sum = fold((acc, x) => acc + x)(0)(numbers)

// ✅ Working - For loops (semicolons in C-style for loops)
for item in arr {
    print(item)
}

for i = 0; i < arr.len; i++ {
    print(arr[i])  // Semicolons required here
}

// ✅ Working - Standard library functions
print("Hello")
toString(42)
abs(-5)
sqrt(16)

// 🚧 Still may have issues
// fn function declarations (parsing issues)
```

Always test examples with the current Carbon-Rust implementation to ensure compatibility.
