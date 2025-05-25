// Simple inheritance test

class Animal {
    name
}

class Dog extends Animal {
    breed
}

let dog = new Dog();
dog.setName("Buddy");
dog.setBreed("Golden Retriever");

print(dog.getName());
print(dog.getBreed());
