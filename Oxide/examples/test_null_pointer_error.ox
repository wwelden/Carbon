// Test null pointer error handling

print("Testing null pointer dereference error...");

let nullPtr = nullptr;
print("Null pointer: " + toString(nullPtr));

// This should cause an error
print("Attempting to dereference null pointer:");
print("Result: " + toString(*nullPtr));
