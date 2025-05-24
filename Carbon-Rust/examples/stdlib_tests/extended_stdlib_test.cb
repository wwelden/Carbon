// Extended Carbon Standard Library Test
// Testing stdlib functions systematically

print("=== Extended Carbon Standard Library Test ===")

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
print("exp(1): " + toString(exp(1)))

// ============ String Functions ============
print("--- String Functions ---")
let str = "Hello World"
print("Original string: " + str)
print("length: " + toString(length(str)))
print("toUpperCase: " + toUpperCase(str))
print("toLowerCase: " + toLowerCase(str))
print("trim('  hello  '): '" + trim("  hello  ") + "'")

// Test split and join
let words = split(" ")(str)
print("split result: " + toString(words))

let joined = join("-")(["a", "b", "c"])
print("join result: " + joined)

print("substring(str)(0)(5): " + substring(str)(0)(5))
print("indexOf(str)('World'): " + toString(indexOf(str)("World")))
print("replace(str)('World')('Carbon'): " + replace(str)("World")("Carbon"))
print("startsWith(str)('Hello'): " + toString(startsWith(str)("Hello")))
print("endsWith(str)('World'): " + toString(endsWith(str)("World")))

// ============ Array Functions ============
print("--- Array Functions ---")
let arr = [1, 2, 3, 4, 5]
print("Original array: " + toString(arr))

print("empty(): " + toString(empty()))
print("concat(arr)([6, 7]): " + toString(concat(arr)([6, 7])))
print("includes(arr)(3): " + toString(includes(arr)(3)))
print("includes(arr)(10): " + toString(includes(arr)(10)))
print("slice(arr)(1)(4): " + toString(slice(arr)(1)(4)))

// ============ Type Checking Functions ============
print("--- Type Checking Functions ---")
print("isNumber(42): " + toString(isNumber(42)))
print("isString('hello'): " + toString(isString("hello")))
print("isBool(true): " + toString(isBool(true)))
print("isArray([1,2,3]): " + toString(isArray([1,2,3])))
print("isNull(null): " + toString(isNull(null)))

// ============ Random Functions ============
print("--- Random Functions ---")
print("random(): " + toString(random()))
print("randomInt(10): " + toString(randomInt(10)))

// ============ Type Conversion Functions ============
print("--- Type Conversion Functions ---")
print("toInt('42'): " + toString(toInt("42")))
print("toFloat('3.14'): " + toString(toFloat("3.14")))
print("toBool(1): " + toString(toBool(1)))
print("toBool(0): " + toString(toBool(0)))

// ============ Higher-Order Functions ============
print("--- Higher-Order Functions ---")
let numbers = [1, 2, 3, 4, 5]
print("Original numbers: " + toString(numbers))

let doubled = map(x => x * 2)(numbers)
print("map(x => x * 2): " + toString(doubled))

let filtered = filter(x => x > 3)(numbers)
print("filter(x => x > 3): " + toString(filtered))

let sum = fold((acc, x) => acc + x)(0)(numbers)
print("fold((acc, x) => acc + x)(0): " + toString(sum))

// ============ Advanced Array Operations ============
print("--- Advanced Array Operations ---")
print("reverse(numbers): " + toString(reverse(numbers)))
print("unique([1,2,2,3,3,4]): " + toString(unique([1,2,2,3,3,4])))
print("take(3)(numbers): " + toString(take(3)(numbers)))
print("drop(2)(numbers): " + toString(drop(2)(numbers)))

print("=== Extended stdlib test complete ===")
