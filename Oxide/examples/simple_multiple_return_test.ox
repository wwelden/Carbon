// Simple multiple return test

fn test() -> (int, int) {
    return 1, 2;
}

let a, b := test();
print("a = " + toString(a));
print("b = " + toString(b));
