# Carbon Data Structures

Carbon provides a comprehensive set of built-in data structures that support modern programming patterns. All data structures are strongly typed and provide efficient operations for common use cases.

## Overview

Carbon includes the following data structures:
- **ArrayList** - Dynamic array with automatic resizing
- **Set** - Unordered collection of unique elements
- **Map** - Key-value dictionary/hash table
- **Stack** - LIFO (Last In, First Out) data structure
- **Queue** - FIFO (First In, First Out) data structure
- **LinkedList** - Doubly-linked list for efficient insertion/deletion

## ArrayList

Dynamic array that automatically resizes as elements are added or removed.

### Syntax
```carbon
let list = ArrayList[element1, element2, element3];
```

### Methods
- `size()` - Returns the number of elements
- `isEmpty()` - Returns true if the list is empty
- `push(element)` - Adds element to the end (mutating)
- `pop()` - Removes and returns the last element (mutating)
- `get(index)` - Returns element at index
- `set(index, element)` - Sets element at index (mutating)

### Example
```carbon
let numbers = ArrayList[1, 2, 3];
print(numbers.size()); // 3
print(numbers.isEmpty()); // false
```

## Set

Unordered collection that automatically removes duplicates.

### Syntax
```carbon
let set = Set{element1, element2, element3};
```

### Methods
- `size()` - Returns the number of unique elements
- `isEmpty()` - Returns true if the set is empty
- `contains(element)` - Returns true if element exists in set
- `add(element)` - Adds element to set (mutating)
- `remove(element)` - Removes element from set (mutating)

### Example
```carbon
let fruits = Set{"apple", "banana", "apple"}; // Duplicate "apple" removed
print(fruits.size()); // 2
print(fruits.contains("apple")); // true
```

## Map

Key-value dictionary for associative data storage.

### Syntax
```carbon
let map = Map{key1: value1, key2: value2};
```

### Methods
- `size()` - Returns the number of key-value pairs
- `isEmpty()` - Returns true if the map is empty
- `containsKey(key)` - Returns true if key exists
- `get(key)` - Returns value for key
- `put(key, value)` - Sets key-value pair (mutating)

### Example
```carbon
let person = Map{"name": "Alice", "age": 30};
print(person.size()); // 2
print(person.containsKey("name")); // true
```

## Stack

LIFO (Last In, First Out) data structure for managing ordered data.

### Syntax
```carbon
let stack = Stack[element1, element2, element3];
```

### Methods
- `size()` - Returns the number of elements
- `isEmpty()` - Returns true if the stack is empty
- `push(element)` - Adds element to top (mutating)
- `pop()` - Removes and returns top element (mutating)
- `peek()` - Returns top element without removing

### Example
```carbon
let stack = Stack[1, 2, 3];
print(stack.size()); // 3
print(stack.isEmpty()); // false
```

## Queue

FIFO (First In, First Out) data structure for sequential processing.

### Syntax
```carbon
let queue = Queue[element1, element2, element3];
```

### Methods
- `size()` - Returns the number of elements
- `isEmpty()` - Returns true if the queue is empty
- `enqueue(element)` - Adds element to back (mutating)
- `dequeue()` - Removes and returns front element (mutating)
- `front()` - Returns front element without removing

### Example
```carbon
let queue = Queue["first", "second", "third"];
print(queue.size()); // 3
print(queue.isEmpty()); // false
```

## LinkedList

Doubly-linked list for efficient insertion and deletion operations.

### Syntax
```carbon
let list = LinkedList[element1, element2, element3];
```

### Methods
- `size()` - Returns the number of elements
- `isEmpty()` - Returns true if the list is empty
- `prepend(element)` - Adds element to beginning (mutating)
- `head()` - Returns first element

### Example
```carbon
let list = LinkedList[100, 200, 300];
print(list.size()); // 3
print(list.isEmpty()); // false
```

## Type Safety

All data structures in Carbon are type-safe and can hold mixed types:

```carbon
// Mixed type ArrayList
let mixed = ArrayList["string", 42, true];

// Mixed type Set
let mixedSet = Set{1, "hello", false};

// Mixed type Map
let mixedMap = Map{"name": "Alice", "age": 30, "active": true};
```

## Performance Characteristics

| Data Structure | Access | Search | Insertion | Deletion |
|---------------|--------|--------|-----------|----------|
| ArrayList     | O(1)   | O(n)   | O(1)*     | O(n)     |
| Set           | N/A    | O(1)   | O(1)      | O(1)     |
| Map           | O(1)   | O(1)   | O(1)      | O(1)     |
| Stack         | O(1)   | O(n)   | O(1)      | O(1)     |
| Queue         | O(1)   | O(n)   | O(1)      | O(1)     |
| LinkedList    | O(n)   | O(n)   | O(1)      | O(1)     |

*Amortized time complexity

## Best Practices

1. **Choose the right data structure** for your use case:
   - Use `ArrayList` for indexed access and dynamic sizing
   - Use `Set` for uniqueness constraints
   - Use `Map` for key-value associations
   - Use `Stack` for LIFO operations (undo functionality, parsing)
   - Use `Queue` for FIFO operations (task processing, BFS)
   - Use `LinkedList` for frequent insertions/deletions

2. **Memory efficiency**: Consider the memory overhead of each structure
3. **Immutability**: Most operations return new values; use mutating methods sparingly
4. **Type consistency**: While mixed types are supported, consistent typing improves performance

## Common Patterns

### Task Queue Processing
```carbon
let tasks = Queue["compile", "test", "deploy"];
for tasks.size() > 0 {
    let task = tasks.dequeue();
    print("Processing: " + task);
}
```

### Undo Stack
```carbon
let undoStack = Stack[];
undoStack.push("create file");
undoStack.push("edit text");
let lastAction = undoStack.pop(); // "edit text"
```

### Caching with Map
```carbon
let cache = Map{};
if cache.containsKey("user:123") {
    // Cache hit
} else {
    cache.put("user:123", "user_data");
}
```

### Unique Item Collection
```carbon
let uniqueItems = Set{};
let items = [1, 2, 2, 3, 1, 4];
for item in items {
    uniqueItems.add(item);
}
// uniqueItems now contains {1, 2, 3, 4}
```
