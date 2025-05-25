// For-loop examples (Go-style where 'for' is used for both for-in and while-like loops)

// Basic for loop (while-like)
let i = 0
for i < 5 {
    print(i)
    i++
}

// For loop with condition (while-like)
let count = 10
for count > 0 {
    print("Countdown: " + toString(count))
    count--
}

// Function with return type annotation (enhanced feature)
fn factorial(int n) int {
    let result = 1
    let i = 1
    for i <= n {
        result *= i
        i++
    }
    return result
}

// Test the function
let fact5 = factorial(5)
print("5! = " + toString(fact5))

// Fibonacci with while loop
fn fibonacci(int n) int {
    if n <= 1 {
        return n
    }

    let a = 0
    let b = 1
    let i = 2

    for i <= n {
        let temp = a + b
        a = b
        b = temp
        i++
    }

    return b
}

// Test fibonacci
for i in [1, 2, 3, 4, 5, 6, 7, 8] {
    print("fib(" + toString(i) + ") = " + toString(fibonacci(i)))
}

// For loop with complex condition (while-like)
let x = 100
for x > 1 && x % 2 == 0 {
    print("Even number: " + toString(x))
    x = x / 2
}

print("Final value: " + toString(x))
