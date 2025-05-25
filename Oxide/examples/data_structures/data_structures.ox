// Data Structures Demo - Comprehensive showcase of Carbon's built-in data structures
// This demonstrates ArrayList, Set, Map, Stack, Queue, and LinkedList

print("=== Carbon Data Structures Demo ===")

// ============ ArrayList Demo ============
print("--- ArrayList Demo ---")
let list = ArrayList[1, 2, 3]
print("Initial ArrayList:")
print(list)

print("Size:")
print(list.size())

print("Is empty:")
print(list.isEmpty())

// ============ Set Demo ============
print("--- Set Demo ---")
let fruits = Set{"apple", "banana", "cherry"}
print("Initial Set:")
print(fruits)

print("Size:")
print(fruits.size())

print("Contains apple:")
print(fruits.contains("apple"))

print("Contains grape:")
print(fruits.contains("grape"))

print("Is empty:")
print(fruits.isEmpty())

// ============ Map Demo ============
print("--- Map Demo ---")
let person = Map{"name": "Alice", "age": 30, "city": "New York"}
print("Initial Map:")
print(person)

print("Size:")
print(person.size())

print("Contains key 'name':")
print(person.containsKey("name"))

print("Contains key 'email':")
print(person.containsKey("email"))

print("Is empty:")
print(person.isEmpty())

// ============ Stack Demo ============
print("--- Stack Demo ---")
let stack = Stack[10, 20, 30]
print("Initial Stack:")
print(stack)

print("Size:")
print(stack.size())

print("Is empty:")
print(stack.isEmpty())

// ============ Queue Demo ============
print("--- Queue Demo ---")
let queue = Queue["first", "second", "third"]
print("Initial Queue:")
print(queue)

print("Size:")
print(queue.size())

print("Is empty:")
print(queue.isEmpty())

// ============ LinkedList Demo ============
print("--- LinkedList Demo ---")
let linkedList = LinkedList[100, 200, 300]
print("Initial LinkedList:")
print(linkedList)

print("Size:")
print(linkedList.size())

print("Is empty:")
print(linkedList.isEmpty())

// ============ Mixed Types Demo ============
print("--- Mixed Types Demo ---")
let mixedList = ArrayList["hello", 42, true]
print("Mixed ArrayList:")
print(mixedList)

let mixedSet = Set{1, 2, 3, 1, 2}; // Duplicates will be removed
print("Mixed Set (duplicates removed):")
print(mixedSet)

let mixedMap = Map{"string": "value", "number": 123, "boolean": true}
print("Mixed Map:")
print(mixedMap)

print("=== Demo Complete ===")
