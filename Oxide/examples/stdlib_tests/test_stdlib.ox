// Test file for Carbon standard library functions

// Math functions
print("=== Math Functions ===")
print(abs(-5))
print(abs(3.14))
print(floor(3.7))
print(ceil(3.2))
print(round(3.6))
print(sqrt(16))
print(pow(2)(3))
print(sin(0))
print(cos(0))
print(log(2.718))
print(exp(1))

// String functions
print("=== String Functions ===")
let str = "Hello World"
print(str)
print(length(str))
print(toUpperCase(str))
print(toLowerCase(str))
print(trim("  hello  "))
print(split(" ")(str))
print(join("-")(["a", "b", "c"]))
print(substring(str)(0)(5))
print(indexOf(str)("World"))
print(replace(str)("World")("Carbon"))
print(startsWith(str)("Hello"))
print(endsWith(str)("World"))

// Array functions
print("=== Array Functions ===")
let arr = [1, 2, 3, 4, 5]
print(arr)
print(empty())
print(concat(arr)([6, 7]))
print(includes(arr)(3))
print(slice(arr)(1)(4))

// Type checking functions
print("=== Type Checking Functions ===")
print(isNumber(42))
print(isString("hello"))
print(isBool(true))
print(isArray([1,2,3]))
print(isNull(null))

// Random functions
print("=== Random Functions ===")
print(random())
print(randomInt(10))

// Type conversion functions
print("=== Type Conversion Functions ===")
print(toInt("42"))
print(toFloat("3.14"))
print(toBool(1))
print(toBool(0))

// Higher-order functions
print("=== Higher-Order Functions ===")
let numbers = [1, 2, 3, 4, 5]
print(numbers)
print(map(fn(x) -> x * 2)(numbers))
print(filter(fn(x) -> x > 3)(numbers))
print(fold(fn(acc, x) -> acc + x)(0)(numbers))

// Test some advanced array operations
print("=== Advanced Array Operations ===")
print(reverse(numbers))
print(unique([1,2,2,3,3,4]))
print(take(3)(numbers))
print(drop(2)(numbers))

print("All tests completed!")
