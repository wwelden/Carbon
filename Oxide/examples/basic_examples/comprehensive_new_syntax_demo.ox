// Comprehensive demonstration of new Carbon syntax features

print("=== Carbon New Syntax Features Demo ===")

print("\n1. Variable Declaration Styles:")

// Typed mutable variables (original syntax)
int age = 25
string city = "San Francisco"
bool isActive = true
int[] scores = [85, 92, 78, 96]

print("Typed variables: age=" + toString(age) + ", city=" + city)

// Type inference with mutability
var temperature = 72.5
var username = "carbon_user"
var items = ["apple", "banana", "cherry"]

print("Var inference: temp=" + toString(temperature) + ", user=" + username)

// Type inference immutable
let pi = 3.14159
let greeting = "Hello, Carbon!"
let primes = [2, 3, 5, 7, 11]

print("Let inference: pi=" + toString(pi) + ", greeting=" + greeting)

// Typed immutable constants
let int maxUsers = 1000
let string version = "1.0.0"
let bool debugMode = false

print("Typed constants: maxUsers=" + toString(maxUsers) + ", version=" + version)

print("\n2. Lambda Function Varieties:")

// Single parameter lambdas
let square = x => x * x
let negate = x => -x

print("square(5) = " + toString(square(5)))
print("negate(7) = " + toString(negate(7)))

// Multi-parameter lambdas
let add = (x, y) => x + y
let multiply = (a, b, c) => a * b * c
let average = (x, y) => (x + y) / 2

print("add(10, 15) = " + toString(add(10)(15)))
print("multiply(2, 3, 4) = " + toString(multiply(2)(3)(4)))
print("average(8, 12) = " + toString(average(8)(12)))

print("\n3. Higher-Order Functions with Multi-Parameter Lambdas:")

let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

// Using multi-parameter lambdas with fold
let sum = fold((acc, x) => acc + x)(0)(numbers)
let product = fold((acc, x) => acc * x)(1)(numbers)

print("Sum of 1-10: " + toString(sum))
print("Product of 1-10: " + toString(product))

// Complex lambda with conditional operation
let processNumbers = fold((result, num) => result + (num % 2 == 0 ? num * 2 : num))(0)(numbers)

print("Processed numbers result: " + toString(processNumbers))

print("\n4. Combining All Features:")

// Mix of variable types with complex operations
var data = [10, 20, 30, 40, 50]
let multiplier = 3
let int threshold = 25

// Filter and transform using multi-parameter lambda
let filtered = filter(x => x > threshold)(data)
let transformed = map(x => x * multiplier)(filtered)
let finalSum = fold((total, value) => total + value)(0)(transformed)

print("Original data: " + toString(data))
print("Filtered (>" + toString(threshold) + "): " + toString(filtered))
print("Transformed (*" + toString(multiplier) + "): " + toString(transformed))
print("Final sum: " + toString(finalSum))

print("\n5. String Processing with New Syntax:")

var text = "Carbon Programming Language"
let words = split(" ")(text)
let wordCount = length(words)

// Process each word with multi-parameter lambda
let processedWords = map(word => toUpperCase(word))(words)
let result = join(" | ")(processedWords)

print("Original: " + text)
print("Word count: " + toString(wordCount))
print("Processed: " + result)

print("\n=== All New Syntax Features Working! ===")
