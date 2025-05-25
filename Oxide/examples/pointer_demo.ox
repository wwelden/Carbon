// Oxide Pointer Operations Demo

print("=== Basic Pointer Operations ===");

// Create a value
let value = 42;
print("Original value: " + toString(value));

// Create a pointer to the value using address-of operator (&)
let ptr = &value;
print("Pointer created: " + toString(ptr));

// Dereference the pointer using dereference operator (*)
let dereferenced = *ptr;
print("Dereferenced value: " + toString(dereferenced));

print("\n=== Pointer Type Information ===");

print("Type of value: " + typeof(value));
print("Type of pointer: " + typeof(ptr));
print("Type of dereferenced: " + typeof(dereferenced));

print("\n=== Pointers with Different Data Types ===");

// String pointer
let message = "Hello Oxide";
let strPtr = &message;
print("String: " + message);
print("String pointer: " + toString(strPtr));
print("Dereferenced string: " + *strPtr);

// Boolean pointer
let flag = true;
let boolPtr = &flag;
print("Boolean: " + toString(flag));
print("Boolean pointer: " + toString(boolPtr));
print("Dereferenced boolean: " + toString(*boolPtr));

// Array pointer
let numbers = [1, 2, 3, 4, 5];
let arrayPtr = &numbers;
print("Array: " + toString(numbers));
print("Array pointer: " + toString(arrayPtr));
print("Dereferenced array: " + toString(*arrayPtr));

print("\n=== Pointer Chains ===");

// Pointer to a pointer
let value2 = 100;
let ptr1 = &value2;
let ptr2 = &ptr1;
print("Original value: " + toString(value2));
print("First pointer: " + toString(ptr1));
print("Second pointer (pointer to pointer): " + toString(ptr2));
print("Dereference first pointer: " + toString(*ptr1));
print("Dereference second pointer: " + toString(*ptr2));

print("\n=== Pointers with Objects ===");

class Person {
    name
    age
}

let person = new Person();
person.setName("Alice");
person.setAge(30);

let personPtr = &person;
print("Person object: " + toString(person));
print("Person pointer: " + toString(personPtr));
print("Dereferenced person: " + toString(*personPtr));

print("\n=== Pointer Operations Summary ===");
print("+ Address-of operator (&): Creates a pointer to a value");
print("+ Dereference operator (*): Gets the value from a pointer");
print("+ Pointers work with all Oxide data types");
print("+ Pointers can point to other pointers (pointer chains)");
print("+ Type system recognizes pointers as distinct types");
