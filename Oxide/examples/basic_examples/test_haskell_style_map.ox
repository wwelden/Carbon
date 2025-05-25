// Demonstrate Haskell-style map function usage patterns
print("=== Haskell-Style map Function Demo ===")

// Basic map usage (like Haskell's map)
print("1. Basic map usage:")
let numbers = [1, 2, 3, 4, 5]
let square = x => x * x
let squares = map(square)(numbers)
print("map(square)([1,2,3,4,5]) = " + toString(squares))

// Partial application (currying) - very Haskell-like
print("2. Partial application:")
let mapSquare = map(square);  // Partially applied function
let moreNumbers = [6, 7, 8]
let moreSquares = mapSquare(moreNumbers)
print("mapSquare = map(square)")
print("mapSquare([6,7,8]) = " + toString(moreSquares))

// Function composition with map
print("3. Function composition:")
let increment = x => x + 1
let double = x => x * 2
let data = [1, 2, 3]

// Apply multiple transformations
let step1 = map(increment)(data)
let step2 = map(double)(step1)
print("map(double)(map(increment)([1,2,3])) = " + toString(step2))

// Using with different data types
print("4. Map with different types:")
let words = ["hello", "world", "carbon"]
let getLength = s => s.len;  // Using our .len property!
let lengths = map(getLength)(words)
print("map(getLength)(words) = " + toString(lengths))

// Higher-order function composition
print("5. Higher-order combinations:")
let isEven = x => x % 2 == 0
let numbersData = [1, 2, 3, 4, 5, 6]

// First map to squares, then filter evens (using other higher-order functions)
let squaredNumbers = map(square)(numbersData)
let evenSquares = filter(isEven)(squaredNumbers)
print("Original: " + toString(numbersData))
print("Squared: " + toString(squaredNumbers))
print("Even squares: " + toString(evenSquares))

// Demonstrate that Map data structure is unaffected
print("6. Map data structure still works:")
let myMap = Map{"key1": "value1", "key2": "value2"}
print("Map data structure: " + toString(myMap))

print("=== All Haskell-style patterns work! ===")
