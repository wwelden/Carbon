// Test pointer modification capabilities

print("=== Testing Pointer Modification ===");

let value = 42;
let ptr = &value;

print("Original value: " + toString(value));
print("Pointer: " + toString(ptr));
print("Dereferenced: " + toString(*ptr));

// Try to modify through pointer
print("\nAttempting to modify through pointer...");
*ptr = 100;

print("After modification attempt:");
print("Value: " + toString(value));
print("Dereferenced: " + toString(*ptr));
