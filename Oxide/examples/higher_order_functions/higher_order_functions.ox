// Higher Order Functions Demo - Carbon Language
// This demonstrates the built-in higher order functions: map, filter, fold, forEach, find, any, all, compose, zip

print("=== Higher Order Functions Demo ===")

// Test data
let numbers = [1, 2, 3, 4, 5]
let words = ["hello", "world", "carbon", "language"]

print("Original numbers:")
print(numbers)
print("Original words:")
print(words)

// MAP - Transform each element
print("\n=== MAP ===")
let double = x => x * 2
let square = x => x * x
let uppercase = s => s; // Note: Carbon doesn't have string methods yet

let doubled = map(double)(numbers)
let squared = map(square)(numbers)

print("Doubled numbers:")
print(doubled)
print("Squared numbers:")
print(squared)

// FILTER - Select elements that match a predicate
print("\n=== FILTER ===")
let isEven = x => x % 2 == 0
let isGreaterThan3 = x => x > 3

let evenNumbers = filter(isEven)(numbers)
let bigNumbers = filter(isGreaterThan3)(numbers)

print("Even numbers:")
print(evenNumbers)
print("Numbers greater than 3:")
print(bigNumbers)

// FOLD/REDUCE - Accumulate values
print("\n=== FOLD/REDUCE ===")
let add = x => y => x + y
let multiply = x => y => x * y

let sum = fold(add)(0)(numbers)
let product = reduce(multiply)(1)(numbers)

print("Sum of numbers:")
print(sum)
print("Product of numbers:")
print(product)

// FOREACH - Execute function for each element (side effects)
print("\n=== FOREACH ===")
let printWithPrefix = x => print("Item: " + toString(x))
print("Printing each number:")
forEach(printWithPrefix)(numbers)

// FIND - Find first element matching predicate
print("\n=== FIND ===")
let isThree = x => x == 3
let isNegative = x => x < 0

let foundThree = find(isThree)(numbers)
let foundNegative = find(isNegative)(numbers)

print("Found number 3:")
print(foundThree)
print("Found negative number:")
print(foundNegative)

// ANY/SOME - Check if any element matches predicate
print("\n=== ANY/SOME ===")
let anyEven = any(isEven)(numbers)
let anyNegative = some(isNegative)(numbers)

print("Any even numbers?")
print(anyEven)
print("Any negative numbers?")
print(anyNegative)

// ALL/EVERY - Check if all elements match predicate
print("\n=== ALL/EVERY ===")
let isPositive = x => x > 0
let isLessThan10 = x => x < 10

let allPositive = all(isPositive)(numbers)
let allSmall = every(isLessThan10)(numbers)

print("All numbers positive?")
print(allPositive)
print("All numbers less than 10?")
print(allSmall)

// COMPOSE - Function composition
print("\n=== COMPOSE ===")
let addOne = x => x + 1
let multiplyByThree = x => x * 3

// compose(f)(g)(x) = f(g(x))
let addOneThenMultiplyByThree = compose(multiplyByThree)(addOne)
let result1 = addOneThenMultiplyByThree(5); // (5 + 1) * 3 = 18

let multiplyByThreeThenAddOne = compose(addOne)(multiplyByThree)
let result2 = multiplyByThreeThenAddOne(5); // (5 * 3) + 1 = 16

print("(5 + 1) * 3 =")
print(result1)
print("(5 * 3) + 1 =")
print(result2)

// ZIP - Combine two arrays into tuples
print("\n=== ZIP ===")
let letters = ["a", "b", "c", "d", "e"]
let zipped = zip(numbers)(letters)

print("Zipped numbers and letters:")
print(zipped)

// COMPLEX EXAMPLE - Chaining higher order functions
print("\n=== COMPLEX EXAMPLE ===")
print("Processing numbers: filter evens, double them, then sum:")

let processNumbers = arr => {
    let evens = filter(isEven)(arr)
    let doubled = map(double)(evens)
    fold(add)(0)(doubled)
}

let result = processNumbers(numbers)
print("Result:")
print(result)

// Another complex example
print("\nFind first even number, square it:")
let firstEvenSquared = x => {
    let firstEven = find(isEven)(x)
    if (firstEven == null) {
        null
    } else {
        firstEven * firstEven
    }
}

let squaredResult = firstEvenSquared(numbers)
print("First even number squared:")
print(squaredResult)

print("\n=== Demo Complete ===")
