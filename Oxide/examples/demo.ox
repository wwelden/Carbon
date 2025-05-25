// Oxide Language Demo - Comprehensive Feature Showcase

// Variable declarations with different syntaxes
var x = 42;                    // Type inferred, mutable
let y = 10;                    // Type inferred, immutable
let int z = 5;                 // Explicitly typed constant
string message = "Hello Oxide!";

print("=== Basic Operations ===");
print(message);
print("x + y = " + toString(x + y));

// Arithmetic and compound operators
x += 10;                       // x = 52
x *= 2;                        // x = 104
x++;                           // x = 105
print("After operations: x = " + toString(x));

// Arrays and for-in loops
let numbers = [1, 2, 3, 4, 5];
print("\n=== Arrays and Loops ===");
print("Numbers: " + toString(numbers));
print("Array length: " + toString(numbers.len));

for num in numbers {
    print("Number: " + toString(num));
}

// Arrow functions and higher-order functions
print("\n=== Functional Programming ===");
let double = x => x * 2;
let add = (x, y) => x + y;

let doubled = map(double)(numbers);
let evens = filter(x => x % 2 == 0)(numbers);
let sum = fold(add)(0)(numbers);

print("Doubled: " + toString(doubled));
print("Evens: " + toString(evens));
print("Sum: " + toString(sum));

// Pattern matching
print("\n=== Pattern Matching ===");
let value = 42;
let result = match value {
    42 => "Found the answer!",
    x if x > 50 => "Big number",
    x if x > 10 => "Medium number",
    _ => "Small number"
};
print("Pattern match result: " + result);

// Array pattern matching
let pair = [10, 20];
let pairSum = match pair {
    [a, b] => a + b,
    _ => 0
};
print("Pair sum: " + toString(pairSum));

// Object-oriented programming with automatic methods
print("\n=== Object-Oriented Programming ===");
class Person {
    name
    age
}

let person = new Person();
person.setName("Alice");
person.setAge(30);

print("Person name: " + toString(person.getName()));
print("Person age: " + toString(person.getAge()));

// Data structures
print("\n=== Data Structures ===");
let list = ArrayList[1, 2, 3];
list.push(4);
print("ArrayList size: " + toString(list.size()));

let fruits = Set{"apple", "banana"};
fruits.add("cherry");
print("Set contains apple: " + toString(fruits.contains("apple")));

let person_map = Map{"name": "Bob", "age": 25};
person_map.put("city", "New York");
print("Map name: " + toString(person_map.get("name")));

// Math functions
print("\n=== Math Functions ===");
print("sqrt(16) = " + toString(sqrt(16)));
print("abs(-5) = " + toString(abs(-5)));
print("pow(2)(3) = " + toString(pow(2)(3)));

// String functions
print("\n=== String Functions ===");
let text = "  Hello World  ";
print("Original: '" + text + "'");
print("Trimmed: '" + trim(text) + "'");
print("Uppercase: '" + toUpperCase(trim(text)) + "'");

let words = split("apple,banana,cherry")(",");
print("Split result: " + toString(words));
print("Joined: " + join("-")(words));

// Error handling
print("\n=== Error Handling ===");
let divide = x => y => {
    if (y == 0) {
        err("Division by zero")
    } else {
        x / y
    }
};

let good_result = divide(10)(2);
let bad_result = divide(10)(0);

print("10/2 is error: " + toString(isErr(good_result)));
print("10/0 is error: " + toString(isErr(bad_result)));

if (!isErr(good_result)) {
    print("Division result: " + toString(good_result));
}

print("\n=== Demo Complete ===");
print("Oxide combines the best of Go, JavaScript, Rust, and Haskell!");
