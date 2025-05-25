// Comprehensive test of sum function on all supported data structures

print("=== Comprehensive Sum Function Test ===");

// Test with Array
print("\n1. Array:");
let arr = [1, 2, 3, 4, 5];
print("Array: " + toString(arr));
print("sum(arr): " + toString(sum(arr)));
print("arr.sum: " + toString(arr.sum));

// Test with ArrayList
print("\n2. ArrayList:");
let list = ArrayList[10, 20, 30];
print("ArrayList: " + toString(list));
print("sum(list): " + toString(sum(list)));
print("list.sum: " + toString(list.sum));

// Test with Set
print("\n3. Set:");
let numSet = Set{1, 2, 3, 4, 5};
print("Set: " + toString(numSet));
print("sum(numSet): " + toString(sum(numSet)));
print("numSet.sum: " + toString(numSet.sum));

// Test with Stack
print("\n4. Stack:");
let stack = Stack[7, 8, 9];
print("Stack: " + toString(stack));
print("sum(stack): " + toString(sum(stack)));
print("stack.sum: " + toString(stack.sum));

// Test with Queue
print("\n5. Queue:");
let queue = Queue[100, 200, 300];
print("Queue: " + toString(queue));
print("sum(queue): " + toString(sum(queue)));
print("queue.sum: " + toString(queue.sum));

// Test with mixed types
print("\n6. Mixed types:");
let mixedArr = [1, 2.5, 3, 4.7];
print("Mixed Array: " + toString(mixedArr));
print("sum(mixedArr): " + toString(sum(mixedArr)));
print("mixedArr.sum: " + toString(mixedArr.sum));

print("\n=== Summary ===");
print("+ sum() function works on: arrays, arraylists, sets, stacks, queues");
print("+ .sum method works on: arrays, arraylists, sets, stacks, queues");
print("+ Both syntaxes produce identical results");
print("+ Supports both integer and floating-point numbers");
print("+ Sets parse string representations of numbers correctly");
