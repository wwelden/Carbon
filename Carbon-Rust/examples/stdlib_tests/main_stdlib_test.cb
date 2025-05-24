// Comprehensive Carbon Standard Library Test Suite
// Working with current parser capabilities

print("=== Carbon Standard Library Comprehensive Test ===")

// ============ Math Functions ============
print("--- Math Functions ---")
print("abs(-5): " + toString(abs(-5)))
print("abs(3.14): " + toString(abs(3.14)))
print("floor(3.7): " + toString(floor(3.7)))
print("ceil(3.2): " + toString(ceil(3.2)))
print("round(3.6): " + toString(round(3.6)))
print("sqrt(16): " + toString(sqrt(16)))
print("pow(2)(3): " + toString(pow(2)(3)))
print("sin(0): " + toString(sin(0)))
print("cos(0): " + toString(cos(0)))
print("tan(0): " + toString(tan(0)))
print("log(2.718): " + toString(log(2.718)))
print("log10(100): " + toString(log10(100)))
print("exp(1): " + toString(exp(1)))

// ============ String Functions ============
print("--- String Functions ---")
let str = "Hello World"
print("Original string: " + str)
print("length: " + toString(length(str)))
print("toUpperCase: " + toUpperCase(str))
print("toLowerCase: " + toLowerCase(str))
print("trim('  hello  '): '" + trim("  hello  ") + "'")

let splitResult = split(" ")(str)
print("split result:")
print(splitResult)

let joinResult = join("-")(["a", "b", "c"])
print("join result: " + joinResult)

print("substring(str)(0)(5): " + substring(str)(0)(5))
print("indexOf(str)('World'): " + toString(indexOf(str)("World")))
print("replace(str)('World')('Carbon'): " + replace(str)("World")("Carbon"))
print("startsWith(str)('Hello'): " + toString(startsWith(str)("Hello")))
print("endsWith(str)('World'): " + toString(endsWith(str)("World")))

// ============ Array Functions ============
print("--- Array Functions ---")
let arr = [1, 2, 3, 4, 5]
print("Original array:")
print(arr)

print("empty():")
print(empty())

let concatResult = concat(arr)([6, 7])
print("concat result:")
print(concatResult)

print("includes(arr)(3): " + toString(includes(arr)(3)))
print("includes(arr)(10): " + toString(includes(arr)(10)))

let sliceResult = slice(arr)(1)(4)
print("slice(arr)(1)(4):")
print(sliceResult)

let flatResult = flatten([[1, 2], [3, 4], [5]])
print("flatten([[1, 2], [3, 4], [5]]):")
print(flatResult)

// ============ Type Checking Functions ============
print("--- Type Checking Functions ---")
print("isNumber(42): " + toString(isNumber(42)))
print("isNumber('hello'): " + toString(isNumber("hello")))
print("isString('hello'): " + toString(isString("hello")))
print("isString(42): " + toString(isString(42)))
print("isBool(true): " + toString(isBool(true)))
print("isBool(42): " + toString(isBool(42)))
print("isArray([1,2,3]): " + toString(isArray([1,2,3])))
print("isArray(42): " + toString(isArray(42)))
print("isNull(null): " + toString(isNull(null)))
print("isNull(42): " + toString(isNull(42)))

// Test function detection
let func = x => x * 2
print("isFunction(func): " + toString(isFunction(func)))

// ============ Random Functions ============
print("--- Random Functions ---")
print("random(): " + toString(random()))
print("random(): " + toString(random()))
print("randomInt(10): " + toString(randomInt(10)))
print("randomInt(100): " + toString(randomInt(100)))

// ============ Type Conversion Functions ============
print("--- Type Conversion Functions ---")
print("toInt('42'): " + toString(toInt("42")))
print("toInt('3.14'): " + toString(toInt("3.14")))
print("toFloat('3.14'): " + toString(toFloat("3.14")))
print("toFloat('42'): " + toString(toFloat("42")))
print("toBool(1): " + toString(toBool(1)))
print("toBool(0): " + toString(toBool(0)))
print("toBool('true'): " + toString(toBool("true")))
print("toBool(''): " + toString(toBool("")))

// ============ Higher-Order Functions with Current Syntax ============
print("--- Higher-Order Functions ---")
let numbers = [1, 2, 3, 4, 5]
print("Original numbers:")
print(numbers)

// Using single-parameter arrow functions that the parser supports
let doubleFunc = x => x * 2
let mappedNumbers = map(doubleFunc)(numbers)
print("map(doubleFunc):")
print(mappedNumbers)

let filterFunc = x => x > 3
let filteredNumbers = filter(filterFunc)(numbers)
print("filter(filterFunc):")
print(filteredNumbers)

// For fold, we may need to test if it works without complex lambdas
// Let's try with a simple operation
let sumFunc = acc => x => acc + x
let sumResult = fold(sumFunc)(0)(numbers)
print("fold result: " + toString(sumResult))

// ============ Advanced Array Operations ============
print("--- Advanced Array Operations ---")
let reversedNumbers = reverse(numbers)
print("reverse(numbers):")
print(reversedNumbers)

let uniqueArray = unique([1,2,2,3,3,4])
print("unique([1,2,2,3,3,4]):")
print(uniqueArray)

let takeResult = take(3)(numbers)
print("take(3)(numbers):")
print(takeResult)

let dropResult = drop(2)(numbers)
print("drop(2)(numbers):")
print(dropResult)

// ============ String and Array Interop ============
print("--- String and Array Interoperability ---")
let words = ["hello", "carbon", "world"]
print("Original words:")
print(words)

let upperWords = map(toUpperCase)(words)
print("map(toUpperCase) on words:")
print(upperWords)

let concatenatedString = join(" ")(upperWords)
print("join(' ') result: " + concatenatedString)

// ============ Complex Nested Operations ============
print("--- Complex Operations ---")
let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

// Chain operations: filter even numbers, then double them
let evenFunc = x => x % 2 == 0
let evens = filter(evenFunc)(data)
let doubledEvens = map(doubleFunc)(evens)
print("Even numbers from 1-10, doubled:")
print(doubledEvens)

print("=== All Standard Library Tests Completed Successfully! ===")
