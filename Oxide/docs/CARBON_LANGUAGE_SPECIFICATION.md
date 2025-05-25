# Carbon Language Specification

## Table of Contents
1. [Overview](#overview)
2. [Lexical Structure](#lexical-structure)
3. [Data Types](#data-types)
4. [Operators](#operators)
5. [Variables and Constants](#variables-and-constants)
6. [Expressions](#expressions)
7. [Control Flow](#control-flow)
8. [Pattern Matching](#pattern-matching)
9. [Functions](#functions)
10. [Error Handling](#error-handling)
11. [Built-in Functions](#built-in-functions)
12. [Examples](#examples)

## Overview

Carbon is a modern programming language that combines syntax elements from multiple popular programming languages:

- **Go**: Clean control flow syntax without parentheses
- **JavaScript**: Dynamic typing, arrow functions, let/const declarations
- **Rust**: Pattern matching syntax and memory safety (implementation)
- **Haskell**: Pattern matching semantics and functional programming
- **C/Java**: Familiar operators and basic syntax structure

**Design Goals:**
- Familiar syntax for developers from different language backgrounds
- Memory safety through Rust implementation
- Powerful pattern matching capabilities
- Clean, readable code structure
- Zero parser conflicts

## Lexical Structure

### Comments
```carbon
// Single-line comments start with //
// This is a comment
let x = 42; // Comment at end of line
```

### Keywords
```
let, const, if, else, match, for, in, true, false, null, typeof, toString, print, err, isErr
```

### Mathematical Constants
```
PI, E, PHI, AVOGADRO
```

### Identifiers
- Start with letter or underscore
- Followed by letters, digits, or underscores
- Case-sensitive

```carbon
let validName = 42;
let _underscore = true;
let camelCase = "string";
```

### Operators
**Arithmetic:** `+`, `-`, `*`, `/`, `%`, `**`
**Assignment:** `=`, `+=`, `-=`, `*=`, `/=`, `%=`
**Increment/Decrement:** `++`, `--`
**Comparison:** `==`, `!=`, `>`, `<`, `>=`, `<=`
**Logical:** `&&`, `||`, `!`
**Ternary:** `?`, `:`
**Arrow:** `=>`
**Pattern Matching:** `=>`

### Literals

#### Integers
```carbon
42          // Positive integer
-17         // Negative integer
0           // Zero
```

#### Floating Point
```carbon
3.14        // Decimal
-2.718      // Negative decimal
0.5         // Fractional
```

#### Booleans
```carbon
true        // Boolean true
false       // Boolean false
```

#### Strings
```carbon
"Hello"     // String literal
"World!"    // String with punctuation
""          // Empty string
```

#### Null
```carbon
null        // Null value
```

#### Arrays
```carbon
[1, 2, 3]       // Integer array
["a", "b", "c"] // String array
[]              // Empty array
[true, false]   // Boolean array
```

#### Tuples
```carbon
(42, "hello", true)  // Mixed-type tuple
(1, 2)               // Pair
()                   // Empty tuple (unit)
```

## Data Types

### Primitive Types

#### Number
- Includes both integers and floating-point numbers
- Automatic type promotion when needed
- Examples: `42`, `3.14`, `-17`

#### Boolean
- Two values: `true` and `false`
- Used in conditional expressions
- Result of comparison operations

#### String
- Sequence of characters
- Immutable
- Double-quoted literals

#### Null
- Represents absence of value
- Single value: `null`
- Used for optional values

### Compound Types

#### Array
- Ordered collection of values
- Can contain mixed types
- Access via index or iteration
- Property: `.len` for length

#### Tuple
- Fixed-size collection of values
- Can contain mixed types
- Used for multiple return values
- Destructurable in pattern matching

## Operators

### Arithmetic Operators

```carbon
let a = 10;
let b = 3;

a + b    // 13 (addition)
a - b    // 7  (subtraction)
a * b    // 30 (multiplication)
a / b    // 3  (division)
a % b    // 1  (modulo)
a ** b   // 1000 (exponentiation)
```

### Assignment Operators

```carbon
let x = 10;

x += 5;  // x = 15 (add and assign)
x -= 3;  // x = 12 (subtract and assign)
x *= 2;  // x = 24 (multiply and assign)
x /= 4;  // x = 6  (divide and assign)
x %= 4;  // x = 2  (modulo and assign)
```

### Increment and Decrement

```carbon
let counter = 5;
counter++;  // counter = 6
counter--;  // counter = 5
```

### Comparison Operators

```carbon
10 == 10    // true  (equality)
10 != 5     // true  (inequality)
10 > 5      // true  (greater than)
5 < 10      // true  (less than)
10 >= 10    // true  (greater or equal)
5 <= 10     // true  (less or equal)
```

### Logical Operators

```carbon
true && false   // false (logical AND)
true || false   // true  (logical OR)
!true           // false (logical NOT)
```

### Ternary Operator

```carbon
let result = condition ? "true value" : "false value";
let max = a > b ? a : b;
```

## Variables and Constants

### Variable Declaration

```carbon
let x = 42;              // Mutable variable
let name = "Carbon";     // String variable
let isActive = true;     // Boolean variable
let items = [1, 2, 3];   // Array variable
```

### Constant Declaration

```carbon
const PI = 3.14159;      // Mathematical constant
const VERSION = "1.0";   // String constant
const MAX_SIZE = 100;    // Numeric constant
```

### Type Inference

Carbon automatically infers types:

```carbon
let num = 42;           // Inferred as number
let text = "hello";     // Inferred as string
let flag = true;        // Inferred as boolean
let list = [1, 2, 3];   // Inferred as array
```

## Expressions

### Primary Expressions

```carbon
42              // Literal
x               // Variable
true            // Boolean literal
"string"        // String literal
[1, 2, 3]       // Array literal
(a, b, c)       // Tuple literal
```

### Function Call Expressions

```carbon
print("hello");         // Built-in function call
typeof(42);            // Type checking function
func(arg1, arg2);      // User function call
```

### Property Access

```carbon
array.len              // Array length property
```

### Mathematical Expressions

```carbon
2 + 3 * 4              // 14 (precedence: * before +)
(2 + 3) * 4            // 20 (parentheses change precedence)
2 ** 3 ** 2            // 512 (right-associative)
```

## Control Flow

### If Expressions

Carbon uses Go-style if syntax without parentheses around conditions:

```carbon
// Basic if-else
let result = if x > 10 { "big" } else { "small" };

// Nested if
let category = if x > 100 {
    "large"
} else {
    if x > 10 { "medium" } else { "small" }
};

// If without else (returns null if condition false)
let check = if valid { "ok" };
```

### Ternary Expressions

```carbon
let max = a > b ? a : b;
let status = isActive ? "online" : "offline";
let grade = score >= 90 ? "A" : score >= 80 ? "B" : "C";
```

### For-In Loops

```carbon
let numbers = [1, 2, 3, 4, 5];

for num in numbers {
    print(num);
}

for item in ["apple", "banana", "cherry"] {
    print(item);
}
```

## Pattern Matching

Carbon features powerful Haskell-inspired pattern matching:

### Basic Patterns

```carbon
// Literal matching
match value {
    42 => "found the answer",
    0 => "zero",
    _ => "something else"
}

// Boolean matching
match flag {
    true => "enabled",
    false => "disabled"
}

// String matching
match name {
    "Alice" => "Hello Alice!",
    "Bob" => "Hi Bob!",
    _ => "Hello stranger"
}
```

### Variable Binding

```carbon
// Bind matched value to variable
match number {
    x => x * 2  // Binds number to x, returns x * 2
}

// Conditional binding
match value {
    x if x > 10 => "large number",
    x => "small number"
}
```

### Array Destructuring

```carbon
// Array pattern matching
match [1, 2, 3] {
    [a, b, c] => a + b + c,  // Returns 6
    _ => 0
}

// Mixed patterns
match [1, x, 3] {
    [1, 2, 3] => "exact match",
    [1, y, 3] => y * 100,    // Binds middle element to y
    _ => 0
}

// Partial matching
match array {
    [first, _] => first,     // Get first element of 2-element array
    _ => null
}
```

### Null Pattern Matching

```carbon
match maybeValue {
    null => "no value",
    x => "has value: " + toString(x)
}
```

### Wildcard Patterns

```carbon
match anything {
    _ => "matches everything"
}
```

## Functions

### Arrow Functions

```carbon
// Single parameter
let double = x => x * 2;
let square = n => n * n;

// Function application
double(21);     // Returns 42
square(5);      // Returns 25

// Higher-order functions
let apply = f => x => f(x);
let addOne = y => y + 1;
apply(addOne)(5);  // Returns 6
```

### Function Composition

```carbon
let compose = f => g => x => f(g(x));
let add10 = x => x + 10;
let multiply2 = x => x * 2;

let combined = compose(add10)(multiply2);
combined(5);  // Returns 20 (5 * 2 + 10)
```

## Error Handling

Carbon uses Go-style explicit error handling:

### Error Creation

```carbon
let errorValue = err("something went wrong");
let divisionError = err("division by zero");
```

### Error Checking

```carbon
let result = someOperation();

if (isErr(result)) {
    print("Error occurred");
} else {
    print("Success: " + toString(result));
}
```

### Error Patterns

```carbon
// Function that may return error
let divide = x => y => {
    if (y == 0) {
        err("division by zero")
    } else {
        x / y
    }
};

let result = divide(10)(2);  // Returns 5
let error = divide(10)(0);   // Returns error

isErr(result);  // false
isErr(error);   // true
```

### Tuples for Multiple Returns

```carbon
// Return (result, error) tuple
let operation = x => {
    if (x < 0) {
        (null, err("negative number"))
    } else {
        (x * 2, null)
    }
};

let (value, error) = operation(42);
if (isErr(error)) {
    print("Error: " + toString(error));
} else {
    print("Result: " + toString(value));
}
```

## Built-in Functions

### Type Functions

```carbon
typeof(42)          // "number"
typeof("hello")     // "string"
typeof(true)        // "boolean"
typeof(null)        // "null"
typeof([1, 2, 3])   // "array"
```

### Conversion Functions

```carbon
toString(42)        // "42"
toString(true)      // "true"
toString([1, 2, 3]) // "[1, 2, 3]"
```

### I/O Functions

```carbon
print("Hello Carbon");      // Output to console
print(42);                  // Print number
print([1, 2, 3]);          // Print array
```

### Error Functions

```carbon
err("message")              // Create error value
isErr(value)               // Check if value is error
```

### Mathematical Constants

```carbon
PI          // 3.141592653589793
E           // 2.718281828459045
PHI         // 1.618033988749895 (Golden ratio)
AVOGADRO    // 6.02214076e23
```

## Examples

### Basic Arithmetic and Variables

```carbon
let a = 10;
let b = 5;

print(a + b);    // 15
print(a - b);    // 5
print(a * b);    // 50
print(a / b);    // 2
print(a % b);    // 0
print(a ** b);   // 100000

let result = a > b ? "a is greater" : "b is greater";
print(result);   // "a is greater"
```

### Pattern Matching Examples

```carbon
// Number classification
let classify = num => match num {
    0 => "zero",
    x if x > 0 => "positive",
    x => "negative"
};

print(classify(42));   // "positive"
print(classify(0));    // "zero"
print(classify(-5));   // "negative"

// Array processing
let processArray = arr => match arr {
    [] => "empty",
    [x] => "single: " + toString(x),
    [first, second] => "pair: " + toString(first) + ", " + toString(second),
    _ => "multiple elements"
};

print(processArray([]));         // "empty"
print(processArray([42]));       // "single: 42"
print(processArray([1, 2]));     // "pair: 1, 2"
print(processArray([1, 2, 3]));  // "multiple elements"
```

### Functional Programming

```carbon
// Higher-order functions
let map = func => arr => {
    let result = [];
    for item in arr {
        // Note: array building not yet implemented
        // This is conceptual
    }
    result
};

let double = x => x * 2;
let numbers = [1, 2, 3, 4, 5];

// Conceptual: map(double)(numbers) would return [2, 4, 6, 8, 10]
```

### Error Handling Example

```carbon
let safeDivide = x => y => {
    if (y == 0) {
        err("Cannot divide by zero")
    } else {
        x / y
    }
};

let result1 = safeDivide(10)(2);
let result2 = safeDivide(10)(0);

if (isErr(result1)) {
    print("Error in result1");
} else {
    print("Result1: " + toString(result1));  // "Result1: 5"
}

if (isErr(result2)) {
    print("Error in result2: " + toString(result2));  // "Error in result2: Cannot divide by zero"
} else {
    print("Result2: " + toString(result2));
}
```

### Complex Pattern Matching

```carbon
let analyzeData = data => match data {
    null => "no data",
    [] => "empty list",
    [x] => "single item: " + toString(x),
    [first, rest] => "first: " + toString(first) + ", has more",
    x if typeof(x) == "number" => "single number: " + toString(x),
    x if typeof(x) == "string" => "text: " + x,
    _ => "unknown data type"
};

print(analyzeData(null));        // "no data"
print(analyzeData([]));          // "empty list"
print(analyzeData([42]));        // "single item: 42"
print(analyzeData([1, 2, 3]));   // "first: 1, has more"
print(analyzeData(42));          // "single number: 42"
print(analyzeData("hello"));     // "text: hello"
```

## Language Grammar Summary

```
Program     := Statement*

Statement   := VarDecl | ConstDecl | Expression ";"

VarDecl     := "let" IDENTIFIER "=" Expression ";"
ConstDecl   := "const" IDENTIFIER "=" Expression ";"

Expression  := Ternary

Ternary     := LogicalOr ("?" Expression ":" Expression)?

LogicalOr   := LogicalAnd ("||" LogicalAnd)*

LogicalAnd  := Equality ("&&" Equality)*

Equality    := Comparison (("==" | "!=") Comparison)*

Comparison  := Addition ((">" | "<" | ">=" | "<=") Addition)*

Addition    := Multiplication (("+" | "-") Multiplication)*

Multiplication := Exponent (("*" | "/" | "%") Exponent)*

Exponent    := Unary ("**" Unary)*

Unary       := ("!" | "-") Unary | Postfix

Postfix     := Primary ("++" | "--" | "." IDENTIFIER)*

Primary     := NUMBER | STRING | BOOLEAN | "null" | IDENTIFIER
             | "(" Expression ")"
             | "[" (Expression ("," Expression)*)? "]"
             | "(" (Expression ("," Expression)*)? ")"
             | IfExpr | MatchExpr | ForExpr | ArrowFunc

IfExpr      := "if" Expression "{" Expression "}" ("else" "{" Expression "}")?

MatchExpr   := "match" Expression "{" (Pattern "=>" Expression ("," Pattern "=>" Expression)*)? "}"

Pattern     := Literal | IDENTIFIER | "[" (Pattern ("," Pattern)*)? "]" | "_"

ForExpr     := "for" IDENTIFIER "in" Expression "{" Statement* "}"

ArrowFunc   := IDENTIFIER "=>" Expression
```

This specification defines the complete Carbon language as currently implemented, providing developers with a comprehensive guide to syntax, semantics, and capabilities.
