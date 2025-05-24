// Extended Higher Order Functions Demo - Carbon Language
// Demonstrating the new higher order functions: flatMap, partition, groupBy, take, drop, reverse, sort, unique, zipWith, scan

print("=== Extended Higher Order Functions Demo ===")

// Test data
let numbers = [1, 2, 3, 4, 5]
let duplicates = [1, 2, 2, 3, 3, 3, 4, 5]
let words = ["apple", "banana", "cherry"]

print("Original numbers:")
print(numbers)
print("Array with duplicates:")
print(duplicates)

// FLATMAP - Map and flatten nested arrays
print("\n=== FLATMAP ===")
let duplicate = x => [x, x]
let flattened = flatMap(duplicate)(numbers)
print("Duplicate each number and flatten:")
print(flattened)

// PARTITION - Split array into two based on predicate
print("\n=== PARTITION ===")
let isEven = x => x % 2 == 0
let partitioned = partition(isEven)(numbers)
print("Partition into even and odd:")
print(partitioned)

// GROUPBY - Group elements by a key function
print("\n=== GROUPBY ===")
let getLength = s => s.len
let grouped = groupBy(getLength)(words)
print("Group words by length:")
print(grouped)

// TAKE - Take first n elements
print("\n=== TAKE ===")
let firstThree = take(3)(numbers)
print("Take first 3 elements:")
print(firstThree)

// DROP - Drop first n elements
print("\n=== DROP ===")
let dropTwo = drop(2)(numbers)
print("Drop first 2 elements:")
print(dropTwo)

// REVERSE - Reverse an array
print("\n=== REVERSE ===")
let reversed = reverse(numbers)
print("Reversed array:")
print(reversed)

// UNIQUE - Remove duplicates
print("\n=== UNIQUE ===")
let uniqueValues = unique(duplicates)
print("Remove duplicates:")
print(uniqueValues)

// SORT - Sort with comparison function
print("\n=== SORT ===")
let compare = x => y => {
    if (x < y) {
        -1
    } else if (x > y) {
        1
    } else {
        0
    }
}
let unsorted = [5, 2, 8, 1, 9, 3]
let sorted = sort(compare)(unsorted)
print("Sort array:")
print("Original: [5, 2, 8, 1, 9, 3]")
print("Sorted:")
print(sorted)

// ZIPWITH - Zip with combining function
print("\n=== ZIPWITH ===")
let add = x => y => x + y
let array1 = [1, 2, 3]
let array2 = [10, 20, 30]
let zippedSum = zipWith(add)(array1)(array2)
print("Zip with addition:")
print("Array1: [1, 2, 3]")
print("Array2: [10, 20, 30]")
print("ZipWith add:")
print(zippedSum)

// SCAN - Like fold but returns intermediate results
print("\n=== SCAN ===")
let multiply = x => y => x * y
let scanned = scan(multiply)(1)(numbers)
print("Scan with multiplication (running product):")
print(scanned)

// COMPLEX EXAMPLE - Chaining multiple functions
print("\n=== COMPLEX CHAINING EXAMPLE ===")
print("Process: take 4, remove duplicates, reverse, then double each:")

let double = x => x * 2
// Since Carbon doesn't have pipe operator, let's do it step by step
let step1 = take(4)(duplicates)
let step2 = unique(step1)
let step3 = reverse(step2)
let step4 = map(double)(step3)

print("Original with duplicates: [1, 2, 2, 3, 3, 3, 4, 5]")
print("After take(4):")
print(step1)
print("After unique:")
print(step2)
print("After reverse:")
print(step3)
print("After map(double):")
print(step4)

print("\n=== Demo Complete ===")
