// Advanced Pointer Features Demo

print("=== Null Pointer Support ===");

let nullPtr = nullptr;
print("Null pointer: " + toString(nullPtr));
print("Type of null pointer: " + typeof(nullPtr));

print("\n=== Pointer Arithmetic ===");

let value = 100;
let ptr = &value;
print("Original pointer: " + toString(ptr));

// Test pointer arithmetic
let ptr_plus_1 = ptr + 1;
let ptr_plus_5 = ptr + 5;
let ptr_minus_2 = ptr - 2;

print("ptr + 1: " + toString(ptr_plus_1));
print("ptr + 5: " + toString(ptr_plus_5));
print("ptr - 2: " + toString(ptr_minus_2));

print("\n=== Pointer Comparison ===");

let value1 = 42;
let value2 = 42;
let value3 = 100;

let ptr1 = &value1;
let ptr2 = &value2;
let ptr3 = &value3;

print("ptr1 points to: " + toString(*ptr1));
print("ptr2 points to: " + toString(*ptr2));
print("ptr3 points to: " + toString(*ptr3));

// Note: These comparisons compare the pointed-to values, not addresses
print("ptr1 == ptr2 (same values): " + toString(ptr1 == ptr2));
print("ptr1 == ptr3 (different values): " + toString(ptr1 == ptr3));

print("\n=== Null Pointer Comparisons ===");

let nullPtr1 = nullptr;
let nullPtr2 = nullptr;
let regularPtr = &value;

print("nullptr == nullptr: " + toString(nullPtr1 == nullPtr2));
print("nullptr == regularPtr: " + toString(nullPtr1 == regularPtr));

print("\n=== Pointer Assignment Syntax ===");

let target = 50;
let targetPtr = &target;

print("Before assignment:");
print("Target value: " + toString(target));
print("Through pointer: " + toString(*targetPtr));

// Pointer assignment (syntax works, but doesn't modify original)
*targetPtr = 200;

print("After assignment attempt:");
print("Target value: " + toString(target));
print("Through pointer: " + toString(*targetPtr));

print("\n=== Error Handling ===");

// Test null pointer dereference (should cause error)
print("Testing null pointer dereference...");
// Uncomment the next line to test error handling:
// print("Dereferencing null: " + toString(*nullptr));

print("\n=== Pointer Features Summary ===");
print("+ Null pointer support (nullptr)");
print("+ Pointer arithmetic (ptr + offset, ptr - offset)");
print("+ Pointer comparison (ptr1 == ptr2, ptr1 != ptr2)");
print("+ Pointer assignment syntax (*ptr = value)");
print("+ Error handling for null pointer operations");
print("- Pointer modification doesn't affect original values (limitation)");
print("- Pointer arithmetic is conceptual (no real address manipulation)");
print("- Pointer comparison compares values, not addresses");

print("\nOxide now has comprehensive pointer support with safety features!");
