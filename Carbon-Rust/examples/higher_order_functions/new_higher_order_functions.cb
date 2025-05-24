// New Higher Order Functions Demo - Carbon Language
// Demonstrating all the newly implemented higher order functions

print("=== New Higher Order Functions Demo ===")

// Test data
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let words = ["apple", "banana", "cherry", "date"]
let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

print("Original numbers:")
print(numbers)
print("Original words:")
print(words)
print("Original matrix:")
print(matrix)

// CONCATMAP - Alias for flatMap
print("\n=== CONCATMAP ===")
let duplicate = x => [x, x]
let concatMapped = concatMap(duplicate)(numbers)
print("ConcatMap duplicate each number:")
print(concatMapped)

// CHUNK - Split array into chunks
print("\n=== CHUNK ===")
let chunked = chunk(3)(numbers)
print("Chunk numbers into groups of 3:")
print(chunked)

// WINDOW - Sliding window
print("\n=== WINDOW ===")
let windowed = window(3)(numbers)
print("Sliding window of size 3:")
print(windowed)

// INTERCALATE - Insert element between array elements (for arrays of arrays)
print("\n=== INTERCALATE ===")
let arrays = [[1, 2], [3, 4], [5, 6]]
let intercalated = intercalate([0])(arrays)
print("Intercalate [0] between arrays [[1,2], [3,4], [5,6]]:")
print(intercalated)

// TRANSPOSE - Transpose 2D array
print("\n=== TRANSPOSE ===")
let transposed = transpose(matrix)
print("Transpose matrix:")
print(transposed)

// MAX - Find maximum
print("\n=== MAX ===")
let maximum = max(numbers)
print("Maximum number:")
print(maximum)

// MIN - Find minimum
print("\n=== MIN ===")
let minimum = min(numbers)
print("Minimum number:")
print(minimum)

// COUNT - Count elements matching predicate
print("\n=== COUNT ===")
let isEven = x => x % 2 == 0
let evenCount = count(isEven)(numbers)
print("Count of even numbers:")
print(evenCount)

// SUM - Sum all numbers
print("\n=== SUM ===")
let total = sum(numbers)
print("Sum of all numbers:")
print(total)

// PRODUCT - Product of all numbers
print("\n=== PRODUCT ===")
let productResult = product(numbers)
print("Product of all numbers:")
print(productResult)

// HEAD - Get first element
print("\n=== HEAD ===")
let firstElement = head(numbers)
print("First element:")
print(firstElement)

// TAIL - Get all but first element
print("\n=== TAIL ===")
let tailElements = tail(numbers)
print("Tail (all but first):")
print(tailElements)

// INIT - Get all but last element
print("\n=== INIT ===")
let initElements = init(numbers)
print("Init (all but last):")
print(initElements)

// LAST - Get last element
print("\n=== LAST ===")
let lastElement = last(numbers)
print("Last element:")
print(lastElement)

// REPEAT - Create array by repeating element
print("\n=== REPEAT ===")
let repeated = repeat(42)(5)
print("Repeat 42 five times:")
print(repeated)

// RANGE - Create range of numbers
print("\n=== RANGE ===")
let rangeResult = range(1)(10)
print("Range from 1 to 10:")
print(rangeResult)

// INTERSPERSE - Insert element between every pair
print("\n=== INTERSPERSE ===")
let interspersed = intersperse(0)(numbers)
print("Intersperse 0 between every pair:")
print(interspersed)

// SORTBY - Sort with custom key function
print("\n=== SORTBY ===")
let getLength = s => s.len
let sortedByLength = sortBy(getLength)(words)
print("Sort words by length:")
print(sortedByLength)

// NUB - Remove duplicates (alias for unique)
print("\n=== NUB ===")
let duplicates = [1, 2, 2, 3, 3, 3, 4, 5]
let nubResult = nub(duplicates)
print("Remove duplicates with nub:")
print("Original: [1, 2, 2, 3, 3, 3, 4, 5]")
print("Nub result:")
print(nubResult)

// COMPLEX CHAINING EXAMPLE
print("\n=== COMPLEX CHAINING EXAMPLE ===")
print("Complex operation: numbers -> chunk(3) -> transpose -> sum each row")

let step1 = chunk(3)(numbers)
print("Step 1 - Chunk into groups of 3:")
print(step1)

let step2 = transpose(step1)
print("Step 2 - Transpose:")
print(step2)

// Sum each row (we'll do this manually since we don't have map over the result yet)
print("Step 3 - Sum each row:")
let row1Sum = sum(head(step2))
let row2Sum = sum(head(tail(step2)))
let row3Sum = sum(head(tail(tail(step2))))
print("Row sums: [" + toString(row1Sum) + ", " + toString(row2Sum) + ", " + toString(row3Sum) + "]")

// Another complex example
print("\n=== ANOTHER COMPLEX EXAMPLE ===")
print("Filter evens, take first 3, repeat each twice, then sum:")

let evens = filter(isEven)(numbers)
print("Evens:")
print(evens)

let firstThreeEvens = take(3)(evens)
print("First 3 evens:")
print(firstThreeEvens)

let doubleEach = x => [x, x]
let repeated_evens = flatMap(doubleEach)(firstThreeEvens)
print("Repeat each twice:")
print(repeated_evens)

let finalSum = sum(repeated_evens)
print("Final sum:")
print(finalSum)

print("\n=== Demo Complete ===")
