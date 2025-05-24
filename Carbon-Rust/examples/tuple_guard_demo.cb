// =======================================================
// Carbon Language: Tuples and Guard Patterns Demo
// =======================================================

print("=== Tuple and Guard Pattern Demonstration ===");

// Basic tuple creation and access
let coord = (10, 20);
print("Basic tuple: " + toString(coord));

// Tuple pattern matching
let result = match coord {
    (x, y) => "Coordinate at (" + toString(x) + ", " + toString(y) + ")"
};
print("Pattern matched: " + result);

// Mixed type tuples
let person = ("Alice", 25, true);
print("Person tuple: " + toString(person));

let description = match person {
    (name, age, isActive) => name + " is " + toString(age) + " years old and " +
                           (isActive ? "active" : "inactive")
};
print("Person description: " + description);

// Guard patterns for more complex matching
print("=== Guard Pattern Examples ===");

// Number classification with guards
let classify = n => match n {
    x if x > 100 => "Large number: " + toString(x)
    x if x > 10 => "Medium number: " + toString(x)
    x if x > 0 => "Small positive: " + toString(x)
    0 => "Zero"
    x if x < 0 => "Negative: " + toString(x)
    _ => "Unknown"
};

print("classify(150): " + classify(150));
print("classify(25): " + classify(25));
print("classify(5): " + classify(5));
print("classify(0): " + classify(0));
print("classify(-10): " + classify(-10));

// Guard patterns with tuples
print("=== Tuple + Guard Patterns ===");

let analyzePoint = point => match point {
    (0, 0) => "Origin point"
    (x, y) if x == y => "Diagonal point (" + toString(x) + ", " + toString(y) + ")"
    (x, y) if x > 0 && y > 0 => "First quadrant"
    (x, y) if x < 0 && y > 0 => "Second quadrant"
    (x, y) if x < 0 && y < 0 => "Third quadrant"
    (x, y) if x > 0 && y < 0 => "Fourth quadrant"
    (x, 0) => "On X-axis at " + toString(x)
    (0, y) => "On Y-axis at " + toString(y)
    _ => "Unknown point"
};

print("analyzePoint((0, 0)): " + analyzePoint((0, 0)));
print("analyzePoint((5, 5)): " + analyzePoint((5, 5)));
print("analyzePoint((3, 4)): " + analyzePoint((3, 4)));
print("analyzePoint((-2, 3)): " + analyzePoint((-2, 3)));
print("analyzePoint((-1, -1)): " + analyzePoint((-1, -1)));
print("analyzePoint((7, 0)): " + analyzePoint((7, 0)));

// Complex guard patterns with array destructuring
print("=== Complex Guard Patterns ===");

let analyzeArray = arr => match arr {
    [] => "Empty array"
    [x] if x > 10 => "Single large element: " + toString(x)
    [x] => "Single element: " + toString(x)
    [x, y] if x == y => "Two equal elements: " + toString(x)
    [x, y] if x + y == 10 => "Two elements that sum to 10"
    [x, y] => "Two elements: " + toString(x) + " and " + toString(y)
    _ => "Array with " + toString(length(arr)) + " elements"
};

print("analyzeArray([]): " + analyzeArray([]));
print("analyzeArray([15]): " + analyzeArray([15]));
print("analyzeArray([3]): " + analyzeArray([3]));
print("analyzeArray([4, 4]): " + analyzeArray([4, 4]));
print("analyzeArray([3, 7]): " + analyzeArray([3, 7]));
print("analyzeArray([1, 2]): " + analyzeArray([1, 2]));
print("analyzeArray([1, 2, 3, 4]): " + analyzeArray([1, 2, 3, 4]));

// Nested pattern matching with guards
print("=== Nested Pattern Matching ===");

let processData = data => match data {
    ("user", (name, age)) if age >= 18 => "Adult user: " + name
    ("user", (name, age)) if age < 18 => "Minor user: " + name
    ("admin", (name, _)) => "Administrator: " + name
    ("guest", _) => "Guest user"
    _ => "Unknown data format"
};

print("processData((\"user\", (\"Bob\", 25))): " + processData(("user", ("Bob", 25))));
print("processData((\"user\", (\"Charlie\", 16))): " + processData(("user", ("Charlie", 16))));
print("processData((\"admin\", (\"Alice\", 30))): " + processData(("admin", ("Alice", 30))));

// Functional programming with tuples and guards
print("=== Functional Programming Examples ===");

let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Create coordinate pairs
let coordinates = map(x => (x, x * x))(numbers);
print("Coordinates (x, x^2): " + toString(coordinates));

// Filter coordinates using guards
let filterCoords = coords => filter(coord => match coord {
    (x, y) if y > 25 => true
    _ => false
})(coords);

let largeCoords = filterCoords(coordinates);
print("Coordinates where y > 25: " + toString(largeCoords));

// Complex tuple transformation
let transformCoord = coord => match coord {
    (x, y) if x == y => ("equal", x)
    (x, y) if x > y => ("x_greater", x - y)
    (x, y) if x < y => ("y_greater", y - x)
    _ => ("unknown", 0)
};

let transformed = map(transformCoord)([(1, 1), (5, 3), (2, 8), (0, 0)]);
print("Transformed coordinates: " + toString(transformed));

print("=== Tuple and Guard Features Complete! ===");
