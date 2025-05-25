// Oxide Pointers and Memory Management
// Pointer creation, dereferencing, null handling, and memory patterns

print("=== Basic Pointer Operations ===");

// Create a variable and get its pointer
let value = 42;
let ptr = &value;

print("Original value: " + toString(value));
print("Pointer address: " + toString(ptr));
print("Dereferenced value: " + toString(*ptr));

// Modify through pointer
*ptr = 100;
print("After modifying through pointer:");
print("Original variable: " + toString(value));
print("Dereferenced pointer: " + toString(*ptr));

print("\n=== Pointer Assignment ===");

let a = 10;
let b = 20;
let ptrA = &a;
let ptrB = &b;

print("a = " + toString(a) + ", b = " + toString(b));
print("*ptrA = " + toString(*ptrA) + ", *ptrB = " + toString(*ptrB));

// Swap values through pointers
let temp = *ptrA;
*ptrA = *ptrB;
*ptrB = temp;

print("After swapping through pointers:");
print("a = " + toString(a) + ", b = " + toString(b));

print("\n=== Null Pointer Handling ===");

let nullPtr = null;
print("Null pointer: " + toString(nullPtr));

// Safe null checking
if (nullPtr != null) {
    print("Pointer is valid: " + toString(*nullPtr));
} else {
    print("Pointer is null - cannot dereference");
}

// Assign valid pointer
nullPtr = &value;
if (nullPtr != null) {
    print("Now pointer is valid: " + toString(*nullPtr));
}

print("\n=== Pointers with Arrays ===");

let numbers = [1, 2, 3, 4, 5];
let arrayPtr = &numbers;

print("Original array: " + toString(numbers));
print("Array through pointer: " + toString(*arrayPtr));

// Modify array through pointer
(*arrayPtr)[0] = 99;
print("After modifying first element through pointer:");
print("Original array: " + toString(numbers));
print("Array through pointer: " + toString(*arrayPtr));

print("\n=== Pointers with Objects ===");

class Point {
    x
    y

    fn display() {
        print("Point(" + toString(x) + ", " + toString(y) + ")");
    }
}

let point = new Point();
point.setX(10);
point.setY(20);

let pointPtr = &point;
print("Original point:");
point.display();

print("Point through pointer:");
(*pointPtr).display();

// Modify object through pointer
(*pointPtr).setX(50);
(*pointPtr).setY(60);

print("After modifying through pointer:");
point.display();

print("\n=== Pointer Chains ===");

let original = 123;
let ptr1 = &original;
let ptr2 = &ptr1;

print("Original value: " + toString(original));
print("Single dereference: " + toString(*ptr1));
print("Double dereference: " + toString(**ptr2));

// Modify through double pointer
**ptr2 = 456;
print("After modifying through double pointer:");
print("Original value: " + toString(original));

print("\n=== Function Parameters with Pointers ===");

// Function that modifies value through pointer
fn increment(int* ptr) {
    if (ptr != null) {
        *ptr = *ptr + 1;
    }
}

let counter = 5;
print("Counter before increment: " + toString(counter));
increment(&counter);
print("Counter after increment: " + toString(counter));

// Function that swaps two values
fn swap(int* a, int* b) {
    if (a != null && b != null) {
        let temp = *a;
        *a = *b;
        *b = temp;
    }
}

let x = 100;
let y = 200;
print("Before swap: x = " + toString(x) + ", y = " + toString(y));
swap(&x, &y);
print("After swap: x = " + toString(x) + ", y = " + toString(y));

print("\n=== Pointer Return Values ===");

// Function returning pointer to larger value
fn getLargerPtr(int* a, int* b) int* {
    if (a == null) return b;
    if (b == null) return a;
    return if (*a > *b) { a } else { b };
}

let val1 = 30;
let val2 = 45;
let largerPtr = getLargerPtr(&val1, &val2);

if (largerPtr != null) {
    print("Larger value: " + toString(*largerPtr));
    *largerPtr = 999;  // Modify the larger value
}

print("After modifying larger value:");
print("val1 = " + toString(val1) + ", val2 = " + toString(val2));

print("\n=== Pointer Arithmetic Patterns ===");

// Array processing with pointers
fn sumArray(int[] arr) int {
    let sum = 0;
    let arrPtr = &arr;

    for i in *arrPtr {
        sum = sum + i;
    }

    return sum;
}

let testArray = [10, 20, 30, 40, 50];
print("Array: " + toString(testArray));
print("Sum using pointer: " + toString(sumArray(testArray)));

print("\n=== Memory Safety Patterns ===");

// Safe pointer operations
fn safeIncrement(int* ptr) bool {
    if (ptr == null) {
        print("Error: Cannot increment null pointer");
        return false;
    }

    *ptr = *ptr + 1;
    return true;
}

let safeValue = 10;
let success = safeIncrement(&safeValue);
print("Safe increment result: " + toString(success) + ", value: " + toString(safeValue));

let failResult = safeIncrement(null);
print("Null increment result: " + toString(failResult));

print("\n=== Pointer Comparison ===");

let var1 = 100;
let var2 = 200;
let ptr1 = &var1;
let ptr2 = &var2;
let ptr3 = &var1;  // Points to same variable as ptr1

print("ptr1 == ptr2: " + toString(ptr1 == ptr2));  // Different variables
print("ptr1 == ptr3: " + toString(ptr1 == ptr3));  // Same variable
print("ptr1 != null: " + toString(ptr1 != null));

print("\n=== Pointers Summary ===");
print("+ & operator: Get address of variable");
print("+ * operator: Dereference pointer");
print("+ null: Special null pointer value");
print("+ Pointer assignment and comparison");
print("+ Safe null checking before dereferencing");
print("+ Pointers work with all data types");
print("+ Functions can take and return pointers");
print("+ Memory safety through null checks");
