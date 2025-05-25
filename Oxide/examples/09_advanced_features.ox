// Oxide Advanced Features
// Generics, error handling, advanced patterns, and language features

print("=== Advanced Control Flow ===");

// Do-while loops
print("Do-while countdown:");
var countdown = 3;
do {
    print("  " + toString(countdown));
    countdown -= 1;
} while (countdown > 0);
print("  Launch!");

// Complex if expressions
fn getGrade(int score) string {
    return if (score >= 90) {
        "A - Excellent"
    } else if (score >= 80) {
        "B - Good"
    } else if (score >= 70) {
        "C - Average"
    } else if (score >= 60) {
        "D - Below Average"
    } else {
        "F - Failing"
    };
}

let scores = [95, 87, 72, 65, 45];
for score in scores {
    print("Score " + toString(score) + ": " + getGrade(score));
}

print("\n=== Advanced Pattern Matching ===");

// Complex match expressions with multiple conditions
fn analyzeData(tuple data) string {
    return match data {
        (type, value) if type == "number" && value > 100 => "Large number: " + toString(value),
        (type, value) if type == "number" && value < 0 => "Negative number: " + toString(value),
        (type, value) if type == "string" && value.len > 10 => "Long string: " + value,
        (type, value) if type == "string" => "Short string: " + value,
        ("boolean", true) => "Boolean true value",
        ("boolean", false) => "Boolean false value",
        _ => "Unknown data type"
    };
}

let dataPoints = [
    ("number", 150),
    ("number", -5),
    ("string", "Hello"),
    ("string", "This is a very long string"),
    ("boolean", true),
    ("boolean", false),
    ("unknown", 42)
];

for data in dataPoints {
    print(analyzeData(data));
}

print("\n=== Error Handling Patterns ===");

// Result-like pattern with multiple returns
fn parseNumber(string str) (int, bool, string) {
    if (str == "0") {
        return 0, true, "success";
    } else if (str == "1") {
        return 1, true, "success";
    } else if (str == "2") {
        return 2, true, "success";
    } else if (str == "3") {
        return 3, true, "success";
    } else if (str == "4") {
        return 4, true, "success";
    } else if (str == "5") {
        return 5, true, "success";
    } else {
        return 0, false, "invalid number format";
    }
}

let testStrings = ["3", "5", "abc", "1", "xyz"];
for str in testStrings {
    let value, success, message := parseNumber(str);
    if (success) {
        print("Parsed '" + str + "' as " + toString(value));
    } else {
        print("Failed to parse '" + str + "': " + message);
    }
}

print("\n=== Advanced Data Processing ===");

// Complex data transformation pipeline
fn processStudentData(tuple student) tuple {
    let name, scores := student;
    let total = sum(scores);
    let average = total / scores.len;
    let grade = getGrade(average);
    let passed = average >= 70;

    return name, average, grade, passed;
}

let students = [
    ("Alice", [95, 87, 92, 88]),
    ("Bob", [78, 82, 75, 80]),
    ("Charlie", [65, 68, 62, 70]),
    ("Diana", [92, 95, 89, 94])
];

print("Student Report:");
for student in students {
    let name, avg, grade, passed := processStudentData(student);
    let status = if (passed) { "PASSED" } else { "FAILED" };
    print(name + ": " + toString(avg) + "% (" + grade + ") - " + status);
}

print("\n=== Advanced Function Patterns ===");

// Currying-like pattern
fn createMultiplier(int factor) fn {
    fn multiply(int x) int {
        return x * factor;
    }
    return multiply;
}

let double = createMultiplier(2);
let triple = createMultiplier(3);

print("Double 5: " + toString(double(5)));
print("Triple 7: " + toString(triple(7)));

// Function that returns different functions based on condition
fn getOperation(string op) fn {
    return match op {
        "add" => fn(int a, int b) int { return a + b; },
        "multiply" => fn(int a, int b) int { return a * b; },
        "subtract" => fn(int a, int b) int { return a - b; },
        _ => fn(int a, int b) int { return 0; }
    };
}

let addFunc = getOperation("add");
let mulFunc = getOperation("multiply");

print("Add 10 + 5: " + toString(addFunc(10, 5)));
print("Multiply 6 * 7: " + toString(mulFunc(6, 7)));

print("\n=== Complex Data Structures ===");

// Nested data structure processing
class Database {
    tables

    fn addTable(string name, tuple[] data) {
        if (tables == null) {
            tables = Map[];
        }
        tables[name] = data;
    }

    fn query(string tableName, fn predicate) tuple[] {
        if (tables == null || tables[tableName] == null) {
            return [];
        }

        return filter(tables[tableName], predicate);
    }
}

let db = new Database();

// Add user data
let users = [
    ("Alice", 25, "Engineer"),
    ("Bob", 30, "Designer"),
    ("Charlie", 28, "Manager"),
    ("Diana", 26, "Engineer")
];

db.addTable("users", users);

// Query functions
fn isEngineer(tuple user) bool {
    let name, age, role := user;
    return role == "Engineer";
}

fn isYoung(tuple user) bool {
    let name, age, role := user;
    return age < 27;
}

let engineers = db.query("users", isEngineer);
let youngUsers = db.query("users", isYoung);

print("Engineers:");
for engineer in engineers {
    let name, age, role := engineer;
    print("  " + name + " (" + toString(age) + ")");
}

print("Young users (< 27):");
for user in youngUsers {
    let name, age, role := user;
    print("  " + name + " (" + toString(age) + ", " + role + ")");
}

print("\n=== Mathematical Constants and Functions ===");

print("Mathematical constants:");
print("PI: " + toString(PI));
print("E: " + toString(E));
print("PHI (Golden Ratio): " + toString(PHI));
print("AVOGADRO: " + toString(AVOGADRO));

// Advanced mathematical operations
fn factorial(int n) int {
    return if (n <= 1) { 1 } else { n * factorial(n - 1) };
}

fn fibonacci(int n) int {
    return if (n <= 1) { n } else { fibonacci(n - 1) + fibonacci(n - 2) };
}

print("\nFactorials:");
for i in [1, 2, 3, 4, 5, 6] {
    print(toString(i) + "! = " + toString(factorial(i)));
}

print("\nFibonacci sequence:");
for i in [0, 1, 2, 3, 4, 5, 6, 7, 8] {
    print("fib(" + toString(i) + ") = " + toString(fibonacci(i)));
}

print("\n=== Advanced Features Summary ===");
print("+ Do-while loops for post-condition iteration");
print("+ Complex if expressions with multiple branches");
print("+ Advanced pattern matching with guards");
print("+ Error handling with multiple return values");
print("+ Higher-order functions and closures");
print("+ Complex data structure manipulation");
print("+ Mathematical constants and recursive functions");
print("+ Functional programming patterns");
print("+ Type-safe data processing pipelines");
