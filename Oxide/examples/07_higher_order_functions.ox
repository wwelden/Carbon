// Oxide Higher-Order Functions
// Map, filter, fold, function composition, and functional programming

print("=== Map Function ===");

// Basic map usage
let numbers = [1, 2, 3, 4, 5];
print("Original array: [1, 2, 3, 4, 5]");

// Square each number
fn square(int x) int {
    return x * x;
}

let squared = map(numbers, square);
print("Squared: " + toString(squared));

// Double each number
fn double(int x) int {
    return x * 2;
}

let doubled = map(numbers, double);
print("Doubled: " + toString(doubled));

print("\n=== Filter Function ===");

// Filter even numbers
fn isEven(int x) bool {
    return x % 2 == 0;
}

let evens = filter(numbers, isEven);
print("Even numbers: " + toString(evens));

// Filter numbers greater than 3
fn greaterThanThree(int x) bool {
    return x > 3;
}

let large = filter(numbers, greaterThanThree);
print("Numbers > 3: " + toString(large));

print("\n=== Fold Function ===");

// Sum all numbers
fn add(int acc, int x) int {
    return acc + x;
}

let sum = fold(numbers, 0, add);
print("Sum of [1, 2, 3, 4, 5]: " + toString(sum));

// Product of all numbers
fn multiply(int acc, int x) int {
    return acc * x;
}

let product = fold(numbers, 1, multiply);
print("Product of [1, 2, 3, 4, 5]: " + toString(product));

// Find maximum
fn max(int acc, int x) int {
    return if (x > acc) { x } else { acc };
}

let maximum = fold(numbers, numbers[0], max);
print("Maximum of [1, 2, 3, 4, 5]: " + toString(maximum));

print("\n=== Function Composition ===");

// Compose square and double
fn squareThenDouble(int x) int {
    return double(square(x));
}

let composed = map(numbers, squareThenDouble);
print("Square then double: " + toString(composed));

// Chain multiple operations
let result = map(filter(numbers, isEven), square);
print("Filter evens then square: " + toString(result));

print("\n=== Working with Strings ===");

let words = ["hello", "world", "oxide", "language"];
print("Original words: " + toString(words));

// Get string lengths
fn getLength(string s) int {
    return s.len;
}

let lengths = map(words, getLength);
print("Word lengths: " + toString(lengths));

// Filter long words
fn isLongWord(string s) bool {
    return s.len > 5;
}

let longWords = filter(words, isLongWord);
print("Long words (>5 chars): " + toString(longWords));

print("\n=== Advanced Functional Patterns ===");

// Count elements using fold
fn count(int acc, int x) int {
    return acc + 1;
}

let arrayLength = fold(numbers, 0, count);
print("Array length using fold: " + toString(arrayLength));

// Build string from array
fn concatenate(string acc, int x) string {
    return acc + toString(x) + " ";
}

let numberString = fold(numbers, "", concatenate);
print("Numbers as string: '" + numberString + "'");

print("\n=== Nested Higher-Order Functions ===");

let matrix = [[1, 2], [3, 4], [5, 6]];
print("Matrix: [[1, 2], [3, 4], [5, 6]]");

// Sum each row
fn sumArray(int[] arr) int {
    return fold(arr, 0, add);
}

let rowSums = map(matrix, sumArray);
print("Row sums: " + toString(rowSums));

print("\n=== Custom Higher-Order Functions ===");

// Apply function twice
fn applyTwice(fn f, int x) int {
    return f(f(x));
}

fn increment(int x) int {
    return x + 1;
}

let incremented = applyTwice(increment, 5);
print("Apply increment twice to 5: " + toString(incremented));

let quadrupled = applyTwice(double, 3);
print("Apply double twice to 3: " + toString(quadrupled));

print("\n=== Practical Examples ===");

// Grade processing
let scores = [85, 92, 78, 96, 88, 73, 91];
print("Test scores: " + toString(scores));

// Convert to letter grades
fn toLetterGrade(int score) string {
    return match score {
        s if s >= 90 => "A",
        s if s >= 80 => "B",
        s if s >= 70 => "C",
        s if s >= 60 => "D",
        _ => "F"
    };
}

let grades = map(scores, toLetterGrade);
print("Letter grades: " + toString(grades));

// Filter passing grades
fn isPassing(int score) bool {
    return score >= 70;
}

let passingScores = filter(scores, isPassing);
print("Passing scores: " + toString(passingScores));

// Calculate average
fn average(int[] arr) int {
    let total = fold(arr, 0, add);
    return total / arr.len;
}

print("Average score: " + toString(average(scores)));
print("Average passing score: " + toString(average(passingScores)));

print("\n=== Functional Programming Summary ===");
print("+ map(array, function): Transform each element");
print("+ filter(array, predicate): Select elements that match");
print("+ fold(array, initial, function): Reduce to single value");
print("+ Functions are first-class values");
print("+ Function composition creates powerful pipelines");
print("+ Immutable data transformation patterns");
print("+ Higher-order functions enable code reuse");
