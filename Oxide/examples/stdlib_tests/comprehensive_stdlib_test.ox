// Comprehensive Carbon Standard Library Test Suite
// Testing all available stdlib functions with proper Carbon syntax

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

let emptyArr = empty()
print("empty():")
print(emptyArr)

let concatResult = concat(arr)([6, 7])
print("concat result:")
print(concatResult)

print("includes(arr)(3): " + toString(includes(arr)(3)))
print("includes(arr)(10): " + toString(includes(arr)(10)))

let sliceResult = slice(arr)(1)(4)
print("slice(arr)(1)(4):")
print(sliceResult)

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

// ============ Higher-Order Functions ============
print("--- Higher-Order Functions ---")
let numbers = [1, 2, 3, 4, 5]
print("Original numbers:")
print(numbers)

let mappedNumbers = map(x => x * 2)(numbers)
print("map(x => x * 2):")
print(mappedNumbers)

let filteredNumbers = filter(x => x > 3)(numbers)
print("filter(x => x > 3):")
print(filteredNumbers)

let sumResult = fold((acc, x) => acc + x)(0)(numbers)
print("fold((acc, x) => acc + x)(0): " + toString(sumResult))

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

// ============ Complex Test Cases ============
print("--- Complex Test Cases ---")

// Test nested operations
let complexArray = [[1, 2], [3, 4], [5, 6]]
print("Complex array operations:")
print(complexArray)

let flattenedArray = flatten(complexArray)
print("flatten(complexArray):")
print(flattenedArray)

// Test function composition
let compositeOp = map(x => x * 2)(filter(x => x > 2)(numbers))
print("Composite operation - map(x => x * 2)(filter(x => x > 2)):")
print(compositeOp)

// Test string and array interop
let words = ["hello", "carbon", "world"]
let upperWords = map(toUpperCase)(words)
print("map(toUpperCase) on words:")
print(upperWords)

let concatenatedString = join(" ")(upperWords)
print("join(' ') result: " + concatenatedString)

print("=== All Standard Library Tests Completed Successfully! ===")
