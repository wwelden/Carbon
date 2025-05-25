// Oxide New Features Demo - Working Features

print("=== Bitwise Operations ===");

let a = 12;  // 1100 in binary
let b = 10;  // 1010 in binary

print("a = " + toString(a));
print("b = " + toString(b));

print("a & b = " + toString(a & b));   // Bitwise AND: 8
print("a | b = " + toString(a | b));   // Bitwise OR: 14
print("a ^ b = " + toString(a ^ b));   // Bitwise XOR: 6
print("~a = " + toString(~a));         // Bitwise NOT: -13
print("a << 1 = " + toString(a << 1)); // Left shift: 24
print("a >> 1 = " + toString(a >> 1)); // Right shift: 6

print("\n=== Number Base Conversions ===");

let num = 255;
print("Decimal: " + toString(num));
print("Binary: " + toBinary(num));
print("Hex: " + toHex(num));

let binaryStr = "1111";  // 4-bit binary
let hexStr = "FF";
print("Binary '" + binaryStr + "' to decimal: " + toString(toDecimal(binaryStr)));
print("Hex '" + hexStr + "' to decimal: " + toString(toDecimal(hexStr)));

print("\n=== Sum Function ===");

let numbers = [1, 2, 3, 4, 5];
print("Array sum: " + toString(sum(numbers)));

let arrayList = ArrayList[10, 20, 30];
print("ArrayList sum: " + toString(sum(arrayList)));

let stack = Stack[5, 10, 15];
print("Stack sum: " + toString(sum(stack)));

let queue = Queue[2, 4, 6];
print("Queue sum: " + toString(sum(queue)));

print("\n=== Basic Inheritance ===");

class Animal {
    name
}

class Dog extends Animal {
    breed
}

let dog = new Dog();
print("Dog created with inheritance from Animal");
print("Dog has name field from Animal: " + toString(dog.getName()));
print("Dog has breed field from Dog: " + toString(dog.getBreed()));

print("\n=== Advanced Bitwise Examples ===");

// Bit manipulation examples
let flags = 0;
flags = flags | 1;      // Set bit 0
flags = flags | 4;      // Set bit 2
print("Flags after setting bits 0 and 2: " + toString(flags) + " (binary: " + toBinary(flags) + ")");

let mask = 7;    // 0111 in binary
let result = flags & mask;
print("Flags & mask: " + toString(result) + " (binary: " + toBinary(result) + ")");

print("\n=== Complex Number Conversions ===");

let powers = [1, 2, 4, 8, 16, 32];
for power in powers {
    print("Decimal " + toString(power) + " = Binary " + toBinary(power) + " = Hex " + toHex(power));
}

print("\n=== Working Features Summary ===");
print("+ Bitwise operators: &, |, ^, ~, <<, >>");
print("+ Base conversions: toBinary(), toHex(), toDecimal()");
print("+ Sum function for all data structures");
print("+ Basic inheritance with extends keyword");
print("+ Automatic getter/setter generation");
print("+ All features integrate with existing Oxide syntax");

print("\nOxide now supports comprehensive bit manipulation,");
print("number base conversions, data structure summation,");
print("and object-oriented inheritance!");
