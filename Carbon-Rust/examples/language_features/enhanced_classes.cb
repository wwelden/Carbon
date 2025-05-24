// Enhanced Object-Oriented Programming Examples

class Calculator {
    value

    init(int initialValue) {
        return this.value = initialValue
    }

    add(int x) {
        return this.value = this.value + x
    }

    multiply(int x) {
        return this.value = this.value * x
    }

    getValue() {
        return this.value
    }

    reset() {
        return this.value = 0
    }
}

// Test calculator
let calc = new Calculator()
calc.init(10)
print("Initial value: " + toString(calc.getValue()))

calc.add(5)
print("After adding 5: " + toString(calc.getValue()))

calc.multiply(3)
print("After multiplying by 3: " + toString(calc.getValue()))

calc.reset()
print("After reset: " + toString(calc.getValue()))

class Point {
    x
    y

    init(int x, int y) {
        this.x = x
        this.y = y
        return this
    }

    getX() { return this.x; }
    getY() { return this.y; }

    distanceFromOrigin() {
        return Math.sqrt(this.x * this.x + this.y * this.y)
    }

    move(int dx, int dy) {
        this.x = this.x + dx
        this.y = this.y + dy
        return this
    }

    toString() {
        return "(" + toString(this.x) + ", " + toString(this.y) + ")"
    }
}

// Test Point class
let p1 = new Point()
p1.init(3, 4)
print("Point p1: " + p1.toString())
print("Distance from origin: " + toString(p1.distanceFromOrigin()))

p1.move(2, -1)
print("After moving (2, -1): " + p1.toString())

class BankAccount {
    balance
    accountNumber

    init(string accountNumber, int initialBalance) {
        this.accountNumber = accountNumber
        this.balance = initialBalance
        return this
    }

    deposit(int amount) {
        if (amount > 0) {
            this.balance = this.balance + amount
            return "Deposited " + toString(amount) + ". New balance: " + toString(this.balance)
        } else {
            return err("Invalid deposit amount")
        }
    }

    withdraw(int amount) {
        if (amount > 0 && amount <= this.balance) {
            this.balance = this.balance - amount
            return "Withdrew " + toString(amount) + ". New balance: " + toString(this.balance)
        } else {
            return err("Invalid withdrawal amount or insufficient funds")
        }
    }

    getBalance() {
        return this.balance
    }

    getAccountInfo() {
        return "Account " + this.accountNumber + " - Balance: $" + toString(this.balance)
    }
}

// Test BankAccount
let account = new BankAccount()
account.init("ACC-001", 1000)
print(account.getAccountInfo())

let depositResult = account.deposit(250)
print(depositResult)

let withdrawResult = account.withdraw(100)
print(withdrawResult)

let invalidWithdraw = account.withdraw(2000)
if (isErr(invalidWithdraw)) {
    print("Withdrawal failed: " + toString(invalidWithdraw))
} else {
    print(invalidWithdraw)
}

print("Final " + account.getAccountInfo())
