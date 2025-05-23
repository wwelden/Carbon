# The Carbon Programming Language - Complete Specification

*A functional programming language with JavaScript-like syntax and object-oriented capabilities*

## Table of Contents

1. [Introduction](#introduction)
2. [Basic Syntax](#basic-syntax)
3. [Data Types](#data-types)
4. [Literals and Constants](#literals-and-constants)
5. [Variables and Declarations](#variables-and-declarations)
6. [Operators](#operators)
7. [Expressions](#expressions)
8. [Control Structures](#control-structures)
9. [Functions](#functions)
10. [Object-Oriented Programming](#object-oriented-programming)
11. [Built-in Functions and References](#built-in-functions-and-references)
12. [Grammar Reference](#grammar-reference)
13. [Examples and Use Cases](#examples-and-use-cases)

## Introduction

Carbon is a functional programming language with object-oriented capabilities that uses JavaScript-like syntax while maintaining the unique features of the original British language. It emphasizes familiar syntax patterns while providing powerful functional programming constructs.

### Language Philosophy
- **Familiarity**: JavaScript-like syntax for easy adoption
- **Clarity**: Clean, readable code structure
- **Expressiveness**: Rich set of operators and control structures
- **Object-Oriented**: Full OOP support with familiar syntax
- **Type Safety**: Strong typing with clear error reporting

## Basic Syntax

### Statement Termination
Every statement ends with a semicolon `;`:

```carbon
2 + 3;
let x = 42;
class MyClass { value; }
```

### Comments
Comments begin with `//` and continue to the end of the line:

```carbon
// This is a comment
2 + 3;  // This is also a comment
```

### Case Sensitivity
The language is case-sensitive with these conventions:
- **Keywords**: lowercase (`class`, `let`, `const`)
- **Class names**: PascalCase (`Stack`, `Counter`, `MyClass`)
- **Field/method names**: camelCase (`getValue`, `count`, `myField`)
- **Variables**: camelCase (`x`, `myVar`, `counter`)

## Data Types

### Primitive Types

#### Numbers
- **Integers**: Whole numbers (e.g., `42`, `-17`, `0`)
- **Real numbers**: Floating-point (e.g., `3.14`, `-2.5`, `0.0`)

#### Booleans
- **`true`**: Boolean true value
- **`false`**: Boolean false value

#### Strings
- Enclosed in double quotes: `"Hello, world!"`

#### Unit Type
- **`null`**: Represents the unit value (similar to void or null)

### Composite Types

#### Arrays
- **Syntax**: `[expr1, expr2]`
- **Example**: `[42, true]` creates an array of (42, true)

#### Objects
- **Class instances**: Created with `new ClassName(args)`
- **Fields**: Accessed with dot notation `object.field`
- **Methods**: Called with dot notation `object.method(args)`

## Literals and Constants

### Mathematical Constants
- **`E`**: Euler's number (e ≈ 2.718)
- **`PI`**: Pi (π ≈ 3.14159)
- **`PHI`**: Golden ratio (φ ≈ 1.618)
- **`AVOGADRO`**: Avogadro's number (≈ 6.022 × 10²³)

### Examples
```carbon
E;           // Outputs: 2.718281828459045
PI * 2;      // Outputs: 6.283185307179586
PHI + 1;     // Outputs: 2.618033988749895
```

## Variables and Declarations

### Variable Declaration
Use `let` to declare mutable variables:

```carbon
let x = 42;
let name = "Hello";
let flag = true;
```

### Immutable Bindings
Use `const` for immutable bindings:

```carbon
const PI_VALUE = 3.14159;
const GREETING = "Good day!";
```

### Variable Naming
- Variables use camelCase
- Can contain letters, numbers, and underscores
- Must start with a letter or underscore

## Operators

### Arithmetic Operators
- **`+`**: Addition
- **`-`**: Subtraction
- **`*`**: Multiplication
- **`/`**: Division
- **`**`**: Exponentiation
- **`%`**: Modulo

### Comparison Operators
- **`==`**: Equality
- **`>`**: Greater than
- **`<`**: Less than
- **`>=`**: Greater than or equal
- **`<=`**: Less than or equal
- **`!=`**: Not equal

### Logical Operators
- **`&&`**: Logical AND
- **`||`**: Logical OR
- **`!`**: Logical NOT

### Special Operators
- **`Math.sqrt`**: Square root function
- **`;`**: Sequence operator (evaluate left, then right)
- **`=`**: Assignment/mutation
- **`*expr`**: Dereference
- **`&expr`**: Create reference

## Expressions

### Arithmetic Expressions
```carbon
2 + 3 * 4;         // Outputs: 14
Math.sqrt(16);     // Outputs: 4.0
2 ** 8;            // Outputs: 256
```

### Boolean Expressions
```carbon
true && false;     // Outputs: false
true || false;     // Outputs: true
!false;            // Outputs: true
```

### Comparison Expressions
```carbon
5 > 3;             // Outputs: true
10 <= 10;          // Outputs: true
"hello" == "world"; // Outputs: false
```

### Array Expressions
```carbon
[42, "hello"];     // Creates array [42, "hello"]
```

## Control Structures

### Conditional Expressions

#### If-Then-Else
```carbon
if (condition) { trueExpr } else { falseExpr }
```

Example:
```carbon
let age = 18;
if (age >= 18) { "adult" } else { "minor" };
// Outputs: "adult"
```

#### Ternary Operator
```carbon
condition ? trueExpr : falseExpr
```

Example:
```carbon
let result = (age >= 18) ? "adult" : "minor";
```

### Loops

#### For Loops
```carbon
for (let variable = init; condition; increment) {
    body
}
```

Example:
```carbon
for (let i = 1; i <= 5; i++) {
    console.log(i);
}
```

#### While Loops
```carbon
while (condition) {
    body
}
```

Example:
```carbon
let count = 0;
while (count < 5) {
    count = count + 1;
}
```

## Functions

### Function Definition
Use arrow functions or function declarations:

```carbon
// Arrow function
let double = (x) => x * 2;

// Function declaration
function double(x) {
    return x * 2;
}
```

### Function Application
```carbon
double(21);        // Outputs: 42
```

### Examples

#### Simple Function
```carbon
let abs = (x) => {
    if (x >= 0) { x } else { -x }
};
abs(-5);          // Outputs: 5
```

### Higher-Order Functions
Functions can accept and return other functions:

```carbon
let applyTwice = (f) => (x) => f(f(x));
let addOne = (x) => x + 1;
applyTwice(addOne)(5);  // Outputs: 7
```

## Object-Oriented Programming

### Class Declaration
Use `class` to declare classes:

```carbon
class ClassName {
    field1;
    field2;

    method1(param) {
        return expression;
    }

    method2(param) {
        return expression;
    }
}
```

### Field Declaration
Fields are declared directly in the class body:

```javascript
class Example {
    fieldName;
}
```

### Method Declaration
Methods are declared as functions within the class:

```javascript
class Example {
    methodName(parameter) {
        return expression;
    }
}
```

### Object Instantiation
Use `new` to create objects:

```javascript
let object = new ClassName(arg1, arg2);
```

### Field Access
Use dot notation to access fields:

```javascript
object.fieldName;
```

### Method Calls
Use dot notation to call methods:

```javascript
object.methodName(argument);
```

### The `this` Keyword
Within methods, `this` refers to the current object:

```javascript
class Person {
    name;

    greet() {
        return "Hello, " + this.name;
    }

    getSelf() {
        return this;
    }
}
```

### Complete OOP Example

```carbon
// Define a Counter class
class Counter {
    count;

    increment() {
        this.count = this.count + 1;
    }

    decrement() {
        this.count = this.count - 1;
    }

    get() {
        return this.count;
    }

    reset() {
        this.count = 0;
    }
}

// Create a counter object
let myCounter = new Counter(10);

// Use the counter
myCounter.get();        // Outputs: 10
myCounter.increment();  // Increment
myCounter.get();        // Outputs: 11
myCounter.reset();      // Reset
myCounter.get();        // Outputs: 0
```

## Built-in Functions and References

### Mathematical Functions
- **`Math.sqrt(expr)`**: Square root
- **`-expr`**: Unary minus (negation)

### Type Conversion and Inspection
- **`toString(expr)`**: Convert to string representation
- **`typeof(expr)`**: Type inspection

### Reference Operations
- **`&expr`**: Create a reference to a value
- **`*expr`**: Dereference a reference
- **`expr = value`**: Assign/mutate a reference

### Example with References
```javascript
let x = 42;
let ref = &x;          // Create reference to x
*ref;                  // Outputs: 42 (dereference)
ref = 100;             // Assign 100 to the reference
*ref;                  // Outputs: 100
```

## Grammar Reference

### Formal Grammar (Simplified BNF)

```bnf
Program ::= Statement+

Statement ::= Declaration ";"

Declaration ::= ExpressionDecl
             | VariableDecl
             | ImmutableDecl
             | ClassDecl

ExpressionDecl ::= Expression

VariableDecl ::= "let" VARIABLE "=" Expression

ImmutableDecl ::= "const" VARIABLE "=" Expression

ClassDecl ::= "class" CLASSNAME "{" ClassMember* "}"

ClassMember ::= FieldDecl | MethodDecl

FieldDecl ::= FIELDNAME ";"

MethodDecl ::= METHODNAME "(" PARAMETER ")" "{" "return" Expression "}"
```

## Examples and Use Cases

### Basic Arithmetic
```carbon
let result = 2 + 3 * 4;  // 14
Math.sqrt(16);           // 4.0
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

### Higher-Order Functions
```carbon
let map = (fn, arr) => {
    // Map implementation would be built-in
    return mappedArray;
};

let numbers = [1, 2, 3, 4];
let doubled = map((x) => x * 2, numbers);  // [2, 4, 6, 8]
```

This specification maintains all the functionality of the original British language while providing a familiar JavaScript-like syntax that developers will find much more approachable. Carbon combines the power of functional programming with the familiarity of modern syntax.
