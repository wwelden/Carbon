// Standard Library Demo - Comprehensive showcase of Carbon's expanded standard library
// This demonstrates the new math, string, array, type checking, random, and conversion functions

print("=== Carbon Standard Library Demo ===")

// ============ Math Functions ============
print("--- Math Functions ---")
let numbers = [-5, 3.14, -2.7, 10]

print("abs(-5): " + toString(abs(-5)))
print("floor(3.14): " + toString(floor(3.14)))
print("ceil(2.7): " + toString(ceil(2.7)))
print("round(3.14): " + toString(round(3.14)))
print("sqrt(16): " + toString(sqrt(16)))
print("pow(2)(8): " + toString(pow(2)(8)));  // 2^8 = 256
print("sin(PI/2): " + toString(sin(PI / 2)))
print("cos(0): " + toString(cos(0)))
print("tan(PI/4): " + toString(tan(PI / 4)))
print("log(E): " + toString(log(E)))
print("log10(100): " + toString(log10(100)))
print("exp(1): " + toString(exp(1)))

// ============ String Functions ============
print("--- String Functions ---")
let text = "  Hello, Carbon World!  "

print("Original: '" + text + "'")
print("Length: " + toString(length(text)))
print("Trimmed: '" + trim(text) + "'")
print("Upper case: " + toUpperCase(text))
print("Lower case: " + toLowerCase(text))

// String manipulation with curried functions
let words = split("Hello,Carbon,World")(",")
print("Split 'Hello,Carbon,World' by ',': ")
print(words)

let joined = join(" - ")(words)
print("Joined with ' - ': " + joined)

let greeting = "Hello World"
print("substring(greeting)(0)(5): " + substring(greeting)(0)(5))
print("indexOf(greeting)('World'): " + toString(indexOf(greeting)("World")))
print("replace(greeting)('World')('Carbon'): " + replace(greeting)("World")("Carbon"))
print("startsWith(greeting)('Hello'): " + toString(startsWith(greeting)("Hello")))
print("endsWith(greeting)('World'): " + toString(endsWith(greeting)("World")))

// ============ Array Functions ============
print("--- Array Functions ---")
let arr1 = [1, 2, 3]
let arr2 = [4, 5, 6]

print("Array 1: ")
print(arr1)
print("Array 2: ")
print(arr2)

print("empty(arr1): " + toString(empty(arr1)))
print("empty([]): " + toString(empty([])))

let concatenated = concat(arr1)(arr2)
print("concat(arr1)(arr2): ")
print(concatenated)

let nested = [[1, 2], [3, 4], [5]]
let flattened = flatten(nested)
print("flatten([[1, 2], [3, 4], [5]]): ")
print(flattened)

print("includes(arr1)(2): " + toString(includes(arr1)(2)))
print("includes(arr1)(7): " + toString(includes(arr1)(7)))

print("indexOf(arr1)(3): " + toString(indexOf(arr1)(3)))
print("indexOf(arr1)(7): " + toString(indexOf(arr1)(7)))

let sliced = slice(concatenated)(1)(4)
print("slice(concatenated)(1)(4): ")
print(sliced)

// ============ Type Checking Functions ============
print("--- Type Checking Functions ---")
let testValues = [42, "hello", true, [1, 2, 3], null]

print("Testing values: ")
print(testValues)

for value in testValues {
    print("Value: " + toString(value))
    print("  isNumber: " + toString(isNumber(value)))
    print("  isString: " + toString(isString(value)))
    print("  isBool: " + toString(isBool(value)))
    print("  isArray: " + toString(isArray(value)))
    print("  isNull: " + toString(isNull(value)))
}

// Test function detection
let func = x => x * 2
print("Function isFunction: " + toString(isFunction(func)))

// ============ Random Functions ============
print("--- Random Functions ---")
print("random(): " + toString(random()))
print("random(): " + toString(random()));  // Different value
print("randomInt(10): " + toString(randomInt(10)(0)))
print("randomInt(100): " + toString(randomInt(100)(0)))

// ============ Conversion Functions ============
print("--- Conversion Functions ---")
let mixedValues = ["123", 45.6, true, false, 0, "3.14"]

print("Converting values: ")
print(mixedValues)

for value in mixedValues {
    print("Value: " + toString(value))
    print("  toInt: " + toString(toInt(value)))
    print("  toFloat: " + toString(toFloat(value)))
    print("  toBool: " + toString(toBool(value)))
}

// ============ Complex Examples ============
print("--- Complex Examples ---")

// Text processing pipeline
let sentence = "the quick brown fox jumps"
let processed = toUpperCase(
    join(" ")(
        map(word => replace(word)("o")("0"))(
            split(sentence)(" ")
        )
    )
)
print("Text processing pipeline: " + processed)

// Mathematical calculations
let coordinates = [[1, 2], [3, 4], [5, 6]]
let distances = map(coord =>
    sqrt(pow(coord[0])(2) + pow(coord[1])(2))
)(coordinates)
print("Distances from origin: ")
print(distances)

// Data analysis
let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let evens = filter(x => x % 2 == 0)(data)
let odds = filter(x => x % 2 != 0)(data)
let evenSum = fold((a, b) => a + b)(0)(evens)
let oddSum = fold((a, b) => a + b)(0)(odds)

print("Data analysis:")
print("Evens: ")
print(evens)
print("Odds: ")
print(odds)
print("Even sum: " + toString(evenSum))
print("Odd sum: " + toString(oddSum))

// String manipulation with higher-order functions
let phrases = ["hello world", "carbon language", "functional programming"]
let transformed = map(phrase =>
    join("")(
        map(toUpperCase)(
            split(phrase)(" ")
        )
    )
)(phrases)
print("Transformed phrases: ")
print(transformed)

print("=== Demo Complete ===")
