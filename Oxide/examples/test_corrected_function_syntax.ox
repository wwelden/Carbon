// Test corrected function syntax

// Single return type
fn add(int a, int b) int {
    return a + b;
}

// Multiple return types
fn divmod(int a, int b) (int, int) {
    let quotient = a / b;
    let remainder = a % b;
    return quotient, remainder;
}

// No return type
fn greet(string name) {
    print("Hello, " + name);
}

// Test the functions
let sum = add(3, 4);
print("3 + 4 = " + toString(sum));

let q, r := divmod(17, 5);
print("17 divided by 5: quotient = " + toString(q) + ", remainder = " + toString(r));

greet("World");
