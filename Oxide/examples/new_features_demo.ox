// Oxide New Features Demo - Bitwise, Conversions, Sum, Inheritance, Generics, Pointers

print("=== Bitwise Operations ===");

// Bitwise operators
let a = 12;  // 1100 in binary
let b = 10;  // 1010 in binary

print("a = " + toString(a) + " (binary: " + toBinary(a) + ")");
print("b = " + toString(b) + " (binary: " + toBinary(b) + ")");

print("a & b = " + toString(a & b));   // Bitwise AND
print("a | b = " + toString(a | b));   // Bitwise OR
print("a ^ b = " + toString(a ^ b));   // Bitwise XOR
print("~a = " + toString(~a));         // Bitwise NOT
print("a << 1 = " + toString(a << 1)); // Left shift
print("a >> 1 = " + toString(a >> 1)); // Right shift

print("\n=== Number Base Conversions ===");

let num = 255;
print("Decimal: " + toString(num));
print("Binary: " + toBinary(num));
print("Hex: " + toHex(num));

let binaryStr = "11111111";
let hexStr = "FF";
print("Binary '" + binaryStr + "' to decimal: " + toString(toDecimal(binaryStr)));
print("Hex '" + hexStr + "' to decimal: " + toString(toDecimal(hexStr)));

print("\n=== Sum Function ===");

let numbers = [1, 2, 3, 4, 5];
print("Array: " + toString(numbers));
print("Sum: " + toString(sum(numbers)));

let arrayList = ArrayList[10, 20, 30];
print("ArrayList: " + toString(arrayList));
print("Sum: " + toString(sum(arrayList)));

print("\n=== Inheritance ===");

// Base class
class Animal {
    name
    age

    speak() {
        return this.name + " makes a sound"
    }
}

// Derived class (inheritance)
class Dog extends Animal {
    breed

    speak() {
        return this.name + " barks!"
    }

    wagTail() {
        return this.name + " wags tail"
    }
}

let dog = new Dog();
dog.setName("Buddy");
dog.setAge(3);
dog.setBreed("Golden Retriever");

print("Dog name: " + dog.getName());
print("Dog age: " + toString(dog.getAge()));
print("Dog breed: " + dog.getBreed());
print("Dog speaks: " + dog.speak());
print("Dog action: " + dog.wagTail());

print("\n=== Generic Functions ===");

// Generic function example (simplified)
fn identity<T>(T value) -> T {
    return value;
}

print("Identity of 42: " + toString(identity(42)));
print("Identity of 'hello': " + identity("hello"));
print("Identity of true: " + toString(identity(true)));

print("\n=== Pointers (Basic) ===");

let value = 100;
let ptr = &value;  // Address-of operator
print("Value: " + toString(value));
print("Pointer: " + toString(ptr));
print("Dereferenced: " + toString(*ptr));  // Dereference operator

// Modify through pointer
*ptr = 200;
print("After modification through pointer:");
print("Value: " + toString(value));
print("Dereferenced: " + toString(*ptr));

print("\n=== Advanced Bitwise Examples ===");

// Bit manipulation examples
let flags = 0;
flags |= 1;      // Set bit 0
flags |= 4;      // Set bit 2
print("Flags after setting bits 0 and 2: " + toString(flags) + " (binary: " + toBinary(flags) + ")");

flags &= ~1;     // Clear bit 0
print("Flags after clearing bit 0: " + toString(flags) + " (binary: " + toBinary(flags) + ")");

let mask = 7;    // 0111 in binary
let result = flags & mask;
print("Flags & mask: " + toString(result) + " (binary: " + toBinary(result) + ")");

print("\n=== Complex Sum Examples ===");

// Sum with different data structures
let stack = Stack[5, 10, 15, 20];
print("Stack sum: " + toString(sum(stack)));

let queue = Queue[2, 4, 6, 8];
print("Queue sum: " + toString(sum(queue)));

// Nested arrays
let nested = [[1, 2], [3, 4], [5, 6]];
print("Nested array: " + toString(nested));
// Note: sum would need to be enhanced to handle nested structures

print("\n=== New Features Demo Complete ===");
print("Oxide now supports bitwise operations, base conversions, sum function,");
print("inheritance, basic generics, and pointer operations!");
