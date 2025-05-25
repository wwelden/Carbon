// Simple multiple return test

fn divmod(int a, int b) (int, int) {
    let quotient = a / b;
    let remainder = a % b;
    return quotient, remainder;
}

let q, r := divmod(17, 5);
print("Quotient: " + toString(q));
print("Remainder: " + toString(r));
