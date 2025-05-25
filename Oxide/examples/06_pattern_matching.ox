// Oxide Pattern Matching and Tuples
// Pattern matching, tuple destructuring, guards, and advanced patterns

print("=== Basic Pattern Matching ===");

fn describeNumber(int n) string {
    return match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other number"
    };
}

for i in [0, 1, 2, 3, 5, 10] {
    print(toString(i) + " is " + describeNumber(i));
}

print("\n=== Pattern Matching with Guards ===");

fn categorizeNumber(int n) string {
    return match n {
        x if x < 0 => "negative",
        0 => "zero",
        x if x > 0 && x <= 10 => "small positive",
        x if x > 10 && x <= 100 => "medium positive",
        _ => "large positive"
    };
}

let testNumbers = [-5, 0, 3, 15, 150];
for num in testNumbers {
    print(toString(num) + " is " + categorizeNumber(num));
}

print("\n=== Tuple Creation and Access ===");

let point2D = (10, 20);
let point3D = (5, 15, 25);
let personInfo = ("Alice", 25, true);

print("2D Point: " + toString(point2D.0) + ", " + toString(point2D.1));
print("3D Point: " + toString(point3D.0) + ", " + toString(point3D.1) + ", " + toString(point3D.2));
print("Person: " + personInfo.0 + ", age " + toString(personInfo.1) + ", active: " + toString(personInfo.2));

print("\n=== Tuple Destructuring ===");

// Destructuring assignment
let x, y := point2D;
print("Destructured 2D point: x = " + toString(x) + ", y = " + toString(y));

let name, age, active := personInfo;
print("Destructured person: " + name + ", " + toString(age) + " years old, active: " + toString(active));

print("\n=== Pattern Matching with Tuples ===");

fn describeTuple(tuple t) string {
    return match t {
        (0, 0) => "origin point",
        (x, 0) => "point on x-axis at " + toString(x),
        (0, y) => "point on y-axis at " + toString(y),
        (x, y) if x == y => "diagonal point at " + toString(x),
        (x, y) if x > 0 && y > 0 => "first quadrant",
        (x, y) if x < 0 && y > 0 => "second quadrant",
        (x, y) if x < 0 && y < 0 => "third quadrant",
        (x, y) if x > 0 && y < 0 => "fourth quadrant",
        _ => "unknown pattern"
    };
}

let points = [(0, 0), (5, 0), (0, -3), (4, 4), (3, 7), (-2, 5), (-4, -6), (8, -2)];
for point in points {
    print("Point " + toString(point.0) + ", " + toString(point.1) + ": " + describeTuple(point));
}

print("\n=== Complex Tuple Patterns ===");

fn analyzeTriple(tuple t) string {
    return match t {
        (a, b, c) if a + b + c == 0 => "sum is zero",
        (a, b, c) if a == b && b == c => "all equal: " + toString(a),
        (a, b, c) if a > b && b > c => "descending order",
        (a, b, c) if a < b && b < c => "ascending order",
        (a, b, c) if a == b || b == c || a == c => "has duplicates",
        _ => "no special pattern"
    };
}

let triples = [(1, -2, 1), (5, 5, 5), (10, 7, 3), (2, 4, 6), (3, 3, 8), (9, 2, 7)];
for triple in triples {
    print("Triple (" + toString(triple.0) + ", " + toString(triple.1) + ", " + toString(triple.2) + "): " + analyzeTriple(triple));
}

print("\n=== Nested Tuple Patterns ===");

let nestedTuple = ((1, 2), (3, 4));
let outerTuple = (nestedTuple, 5);

print("Nested tuple access:");
print("First inner tuple: (" + toString(nestedTuple.0.0) + ", " + toString(nestedTuple.0.1) + ")");
print("Second inner tuple: (" + toString(nestedTuple.1.0) + ", " + toString(nestedTuple.1.1) + ")");
print("Outer tuple value: " + toString(outerTuple.1));

print("\n=== Tuple Functions ===");

// Function returning tuple
fn getMinMax(int[] arr) (int, int) {
    let min = arr[0];
    let max = arr[0];

    for val in arr {
        if (val < min) {
            min = val;
        }
        if (val > max) {
            max = val;
        }
    }

    return min, max;
}

let numbers = [3, 1, 4, 1, 5, 9, 2, 6];
let minimum, maximum := getMinMax(numbers);
print("Array [3, 1, 4, 1, 5, 9, 2, 6]:");
print("Min: " + toString(minimum) + ", Max: " + toString(maximum));

// Function taking tuple parameter
fn distance(tuple point1, tuple point2) int {
    let dx = point1.0 - point2.0;
    let dy = point1.1 - point2.1;
    return dx * dx + dy * dy;  // Squared distance for simplicity
}

let p1 = (0, 0);
let p2 = (3, 4);
print("Squared distance between (0,0) and (3,4): " + toString(distance(p1, p2)));

print("\n=== Pattern Matching Summary ===");
print("+ match expressions with multiple patterns");
print("+ Guards with 'if' conditions");
print("+ Tuple creation: (a, b, c)");
print("+ Tuple access: tuple.0, tuple.1, tuple.2");
print("+ Tuple destructuring: let a, b := tuple");
print("+ Pattern matching on tuple structure");
print("+ Functions can return and accept tuples");
print("+ Nested tuples and complex patterns supported");
