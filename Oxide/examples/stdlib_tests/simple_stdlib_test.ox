// Simple Standard Library Test

print("=== Testing New Standard Library Functions ===")

// Math functions
print("abs(-5): " + toString(abs(-5)))
print("sqrt(16): " + toString(sqrt(16)))
print("pow(2)(3): " + toString(pow(2)(3)))

// String functions
print("toUpperCase(\"hello\"): " + toUpperCase("hello"))
print("trim(\"  test  \"): '" + trim("  test  ") + "'")

// Array manipulation
let arr = [1, 2, 3]
print("Original array: " + toString(arr))
print("concat([1,2,3])([4,5]): " + toString(concat(arr)([4, 5])))

// Type checking
print("isNumber(42): " + toString(isNumber(42)))
print("isString(\"hello\"): " + toString(isString("hello")))

// Conversion
print("toInt(\"123\"): " + toString(toInt("123")))
print("toBool(1): " + toString(toBool(1)))

print("=== All tests completed ===")
