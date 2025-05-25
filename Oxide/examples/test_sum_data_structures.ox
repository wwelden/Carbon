// Test sum function on different data structures

print("=== Testing sum() function on different data structures ===");

// Test with Array
print("\n1. Array:");
let arr = [1, 2, 3, 4, 5];
print("Array: " + toString(arr));
print("Sum: " + toString(sum(arr)));

// Test with ArrayList
print("\n2. ArrayList:");
let list = ArrayList[10, 20, 30];
print("ArrayList: " + toString(list));
print("Sum: " + toString(sum(list)));

// Test with Set (containing numeric strings)
print("\n3. Set:");
let numSet = Set{1, 2, 3, 4, 5};
print("Set: " + toString(numSet));
print("Sum: " + toString(sum(numSet)));

// Test with Stack
print("\n4. Stack:");
let stack = Stack[7, 8, 9];
print("Stack: " + toString(stack));
print("Sum: " + toString(sum(stack)));

// Test with Queue
print("\n5. Queue:");
let queue = Queue[100, 200, 300];
print("Queue: " + toString(queue));
print("Sum: " + toString(sum(queue)));

// Test with mixed integer and float values
print("\n6. Mixed types:");
let mixedArr = [1, 2.5, 3, 4.7];
print("Mixed Array: " + toString(mixedArr));
print("Sum: " + toString(sum(mixedArr)));

print("\n=== All data structures support sum() function! ===");
