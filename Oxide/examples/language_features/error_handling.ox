// Go-style Error Handling in Carbon
// This demonstrates explicit error checking and handling

// Function that might fail (simulated with conditions)
let divide = x => y => {
    // In a real implementation, we'd return (result, error) tuple
    if (y == 0) {
        err("division by zero")
    } else {
        x / y
    }
}

// Test successful division
let result1 = divide(10)(2)
print("Result 1:")
print(result1)
print("Is error?")
print(isErr(result1))

// Test division by zero (error case)
let result2 = divide(10)(0)
print("Result 2:")
print(result2)
print("Is error?")
print(isErr(result2))

// Error checking pattern (Go-style)
let checkAndUse = result => {
    if (isErr(result)) {
        print("Error occurred:")
        print(result)
    } else {
        print("Success! Result is:")
        print(result)
    }
}

print("=== Checking Results ===")
checkAndUse(result1)
checkAndUse(result2)

// Tuple example for returning (result, error)
let safeOperation = x => {
    if (x < 0) {
        (null, err("negative input not allowed"))
    } else {
        (x * 2, null)
    }
}

print("=== Tuple Examples ===")
let tuple1 = safeOperation(5)
print("Tuple 1:")
print(tuple1)

let tuple2 = safeOperation(-3)
print("Tuple 2:")
print(tuple2)

// Creating different types of errors
print("=== Different Error Types ===")
let validationErr = err("validation failed")
let networkErr = err("network timeout")
let dbErr = err("database connection lost")

print("Validation error:")
print(validationErr)
print("Network error:")
print(networkErr)
print("Database error:")
print(dbErr)

print("=== Error checking ===")
print("Is validation error?")
print(isErr(validationErr))
print("Is number an error?")
print(isErr(42))
print("Is string an error?")
print(isErr("hello"))
