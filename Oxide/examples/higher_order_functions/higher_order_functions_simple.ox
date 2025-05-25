// Higher Order Functions Demo - Carbon Language
// Simple version demonstrating built-in higher order functions

print("=== Higher Order Functions Demo ===")

// Test data
let numbers = [1, 2, 3, 4, 5]
print("Original numbers:")
print(numbers)

// MAP - Transform each element
print("\n=== MAP ===")
let double = x => x * 2
let doubled = map(double)(numbers)
print("Doubled numbers:")
print(doubled)

// FILTER - Select elements that match a predicate
print("\n=== FILTER ===")
let isEven = x => x % 2 == 0
let evenNumbers = filter(isEven)(numbers)
print("Even numbers:")
print(evenNumbers)

// FOLD/REDUCE - Accumulate values
print("\n=== FOLD/REDUCE ===")
let add = x => y => x + y
let sum = fold(add)(0)(numbers)
print("Sum of numbers:")
print(sum)

// FOREACH - Execute function for each element
print("\n=== FOREACH ===")
let printItem = x => print("Item: " + toString(x))
print("Printing each number:")
forEach(printItem)(numbers)

// FIND - Find first element matching predicate
print("\n=== FIND ===")
let isThree = x => x == 3
let foundThree = find(isThree)(numbers)
print("Found number 3:")
print(foundThree)

// ANY/SOME - Check if any element matches predicate
print("\n=== ANY/SOME ===")
let anyEven = any(isEven)(numbers)
print("Any even numbers?")
print(anyEven)

// ALL/EVERY - Check if all elements match predicate
print("\n=== ALL/EVERY ===")
let isPositive = x => x > 0
let allPositive = all(isPositive)(numbers)
print("All numbers positive?")
print(allPositive)

// COMPOSE - Function composition
print("\n=== COMPOSE ===")
let addOne = x => x + 1
let multiplyByThree = x => x * 3
let composed = compose(multiplyByThree)(addOne)
let result = composed(5)
print("(5 + 1) * 3 =")
print(result)

// ZIP - Combine two arrays
print("\n=== ZIP ===")
let letters = ["a", "b", "c"]
let zipped = zip(numbers)(letters)
print("Zipped arrays:")
print(zipped)

print("\n=== Demo Complete ===")
