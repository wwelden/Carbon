// Comprehensive Demo: Automatic Getters, Setters, and Init Methods
// Carbon automatically generates these methods for every class!

print("=== Carbon Automatic Method Generation Demo ===")
print("")

// Define a simple Person class with two fields
class Person {
    name
    age
}

print("1. Class Definition:")
print("   class Person { name, age }")
print("")

// Create a person object
let person = new Person()
print("2. Object Creation:")
print("   let person = new Person()")
print("   Created: " + typeof(person))
print("")

// Test automatic getters (return field values)
print("3. Automatic Getters:")
print("   getName(): " + toString(person.getName()))
print("   getAge(): " + toString(person.getAge()))
print("")

// Test automatic setters (set field values and return the value)
print("4. Automatic Setters:")
print("   setName('Alice'): " + toString(person.setName("Alice")))
print("   setAge(30): " + toString(person.setAge(30)))
print("")

// Now getters should return the set values
print("5. Getters After Setting Values:")
print("   getName(): " + toString(person.getName()))
print("   getAge(): " + toString(person.getAge()))
print("")

// Test automatic init method (returns the object for chaining)
print("6. Automatic Init Method:")
let person2 = new Person()
let initialized = person2.init("Bob")
print("   person2.init('Bob'): " + typeof(initialized))
print("")

// More complex example with a Car class
class Car {
    make
    model
    year
    color
}

print("7. Complex Example - Car Class:")
print("   class Car { make, model, year, color }")
print("")

let car = new Car()

// Chain setter calls
print("8. Method Chaining with Setters:")
car.setMake("Toyota")
car.setModel("Camry")
car.setYear(2023)
car.setColor("Blue")

print("   After setting all values:")
print("   getMake(): " + toString(car.getMake()))
print("   getModel(): " + toString(car.getModel()))
print("   getYear(): " + toString(car.getYear()))
print("   getColor(): " + toString(car.getColor()))
print("")

// Show that manual methods still work alongside automatic ones
class Calculator {
    value

    // Manual method - won't be overridden by automatic generation
    getValue() {
        return "Manual getValue: " + toString(this.value)
    }

    // Manual init - won't be overridden
    init(initialValue) {
        this.value = initialValue
        return "Initialized with: " + toString(initialValue)
    }
}

print("9. Manual Methods Override Automatic Ones:")
let calc = new Calculator()
print("   calc.init(42): " + toString(calc.init(42)))
print("   calc.getValue(): " + toString(calc.getValue()))
print("   But automatic setValue() still exists: " + toString(calc.setValue(100)))
print("")

print("=== Demo Complete! ===")
print("Carbon automatically generates:")
print("  - getFieldName() - returns field value")
print("  - setFieldName(value) - sets field and returns value")
print("  - init(value) - initialization method")
print("  - Manual methods take precedence over automatic ones")
print("  - Zero-argument methods now supported!")
