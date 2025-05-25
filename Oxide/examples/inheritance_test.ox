// Test inheritance

class Animal {
    name
    age

    speak() {
        return this.name + " makes a sound"
    }
}

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
