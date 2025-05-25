// Test higher-order functions
print("=== Higher-Order Functions Test ===")

let numbers = [1, 2, 3, 4, 5]
print("Original numbers: " + toString(numbers))

// Test map
let doubled = map(x => x * 2)(numbers)
print("map(x => x * 2): " + toString(doubled))

// Test filter
let filtered = filter(x => x > 3)(numbers)
print("filter(x => x > 3): " + toString(filtered))

// Test fold
let sum = fold((acc, x) => acc + x)(0)(numbers)
print("fold((acc, x) => acc + x)(0): " + toString(sum))

print("=== Higher-order test complete ===")
