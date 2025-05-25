// Oxide Multiple Return Values Demo - Go Style

print("=== Multiple Return Values Demo ===" );

// Function that returns multiple values
fn divmod(int a, int b) (int, int) {
    let quotient = a / b;
    let remainder = a % b;
    return quotient, remainder;
}

print("\n1. Basic Multiple Return Values");
let q, r := divmod(17, 5);
print("17 divided by 5: quotient = " + toString(q) + ", remainder = " + toString(r));

// Function that returns name and age
fn getPersonInfo(string name) (string, int, bool) {
    if (name == "Alice") {
        return "Alice Johnson", 25, true;
    } else if (name == "Bob") {
        return "Bob Smith", 30, false;
    } else {
        return "Unknown", 0, false;
    }
}

print("\n2. Multiple Return with Different Types");
let fullName, age, isActive := getPersonInfo("Alice");
print("Person: " + fullName + ", Age: " + toString(age) + ", Active: " + toString(isActive));

let name2, age2, active2 := getPersonInfo("Bob");
print("Person: " + name2 + ", Age: " + toString(age2) + ", Active: " + toString(active2));

// Function that returns coordinates
fn getCoordinates() (int, int, int) {
    return 10, 20, 30;
}

print("\n3. Three Return Values");
let x, y, z := getCoordinates();
print("Coordinates: x=" + toString(x) + ", y=" + toString(y) + ", z=" + toString(z));

// Function that returns array statistics
fn arrayStats(int[] arr) (int, int, int) {
    let sum = 0;
    let min = arr[0];
    let max = arr[0];

    for i in arr {
        sum = sum + i;
        if (i < min) {
            min = i;
        }
        if (i > max) {
            max = i;
        }
    }

    return sum, min, max;
}

print("\n4. Array Statistics with Multiple Returns");
let numbers = [5, 2, 8, 1, 9, 3];
let total, minimum, maximum := arrayStats(numbers);
print("Array: [5, 2, 8, 1, 9, 3]");
print("Sum: " + toString(total) + ", Min: " + toString(minimum) + ", Max: " + toString(maximum));

// Function that returns error status and result
fn safeDivide(int a, int b) (int, bool) {
    if (b == 0) {
        return 0, false;  // error case
    } else {
        return a / b, true;  // success case
    }
}

print("\n5. Error Handling with Multiple Returns");
let result1, success1 := safeDivide(10, 2);
if (success1) {
    print("10 / 2 = " + toString(result1));
} else {
    print("Division failed");
}

let result2, success2 := safeDivide(10, 0);
if (success2) {
    print("10 / 0 = " + toString(result2));
} else {
    print("Division by zero error");
}

// Function that returns string processing results
fn processString(string str) (string, int, bool) {
    let upper = str;  // In a real implementation, this would convert to uppercase
    let length = str.len;
    let isEmpty = length == 0;
    return upper, length, isEmpty;
}

print("\n6. String Processing with Multiple Returns");
let processed, len, empty := processString("Hello");
print("String: 'Hello' -> Processed: '" + processed + "', Length: " + toString(len) + ", Empty: " + toString(empty));

let proc2, len2, empty2 := processString("");
print("String: '' -> Processed: '" + proc2 + "', Length: " + toString(len2) + ", Empty: " + toString(empty2));

print("\n=== Multiple Return Values Features Summary ===");
print("+ Function can return multiple values: fn name() (type1, type2, type3)");
print("+ Multiple assignment syntax: a, b, c := function_call()");
print("+ Works with different types: int, string, bool, arrays, etc.");
print("+ Useful for error handling patterns (value, success)");
print("+ Great for returning related data (coordinates, statistics, etc.)");
print("+ Go-style syntax and semantics");
