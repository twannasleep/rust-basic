# 🎯 Rust Fundamentals

## 📦 Variables and Data Types

### Variables in Rust

Rust's variable system is designed for safety and clarity. Here's what you need to know:

| Feature | Description | Example |
|---------|-------------|---------|
| Immutability | Variables are immutable by default | `let x = 5;` |
| Mutability | Use `mut` for mutable variables | `let mut y = 10;` |
| Constants | Uppercase, type required | `const MAX_POINTS: u32 = 100_000;` |
| Shadowing | Reuse variable names | `let spaces = "   "; let spaces = spaces.len();` |

```rust
// Examples of variable declarations
let immutable_var = 42;
let mut mutable_var = 100;
const MAX_VALUE: i32 = 1_000_000;

// Shadowing example
let value = "hello";
let value = value.len();  // value is now 5
```

### 🔢 Basic Data Types

#### Integer Types

| Type | Size | Range |
|------|------|-------|
| i8 | 1 byte | -128 to 127 |
| u8 | 1 byte | 0 to 255 |
| i32 | 4 bytes | -2^31 to 2^31-1 |
| u64 | 8 bytes | 0 to 2^64-1 |

```rust
// Integer examples
let signed: i32 = -42;
let unsigned: u64 = 42;
let byte: u8 = 255;
```

#### Floating-Point Types

| Type | Precision | Size |
|------|-----------|------|
| f32 | Single | 4 bytes |
| f64 | Double | 8 bytes |

```rust
let float32: f32 = 3.14;
let float64: f64 = 3.14159265359;
```

#### Other Basic Types

```rust
// Boolean
let is_active: bool = true;

// Character (Unicode)
let letter: char = 'A';
let emoji: char = '😀';

// Tuple
let coordinates: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = coordinates;  // Destructuring

// Array
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let first = numbers[0];
```

## 🔄 Control Flow

### Conditional Statements

```rust
// If expressions
let number = 7;
let message = if number < 5 {
    "less than five"
} else if number > 5 {
    "greater than five"
} else {
    "equal to five"
};

// Match expressions
let code = 404;
let status = match code {
    200 => "OK",
    404 => "Not Found",
    other => "Unknown Status",
};
```

### 🔁 Loops

#### Three Types of Loops

1. **`loop`** - Infinite loop with break

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

2. **`while`** - Conditional loop

```rust
let mut lives = 3;
while lives > 0 {
    println!("Lives remaining: {}", lives);
    lives -= 1;
}
```

3. **`for`** - Iterator-based loop

```rust
// Range-based
for number in 1..=5 {
    println!("{}", number);
}

// Iterator-based
let animals = ["🐱", "🐶", "🐰"];
for animal in animals.iter() {
    println!("{}", animal);
}
```

## 🛠️ Functions

### Function Basics

```rust
// Basic function
fn add(x: i32, y: i32) -> i32 {
    x + y  // Implicit return
}

// Multiple parameters
fn print_coordinates(x: i32, y: i32) {
    println!("({}, {})", x, y);
}

// Generic functions
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### Advanced Function Features

```rust
// Function with early return
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        return None;
    }
    Some(x / y)
}

// Closures
let add_one = |x| x + 1;
let result = add_one(5);  // 6

// Higher-order functions
let numbers = vec![1, 2, 3, 4, 5];
let squares: Vec<i32> = numbers
    .iter()
    .map(|x| x * x)
    .collect();
```

## 📝 Best Practices

1. **Variable Naming**
   - Use snake_case for variables and functions
   - Use SCREAMING_SNAKE_CASE for constants
   - Choose descriptive names

2. **Type Annotations**
   - Let type inference work when obvious
   - Add annotations for clarity when needed
   - Always annotate function parameters and returns

3. **Control Flow**
   - Prefer `match` over multiple `if-else` when possible
   - Use `loop` with `break` for complex conditions
   - Leverage iterator methods over manual indexing

4. **Error Handling**
   - Use `Option` for nullable values
   - Use `Result` for operations that can fail
   - Avoid unwrap() in production code

## 🔍 Examples in Practice

### Complete Program Example

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Rectangle: {:?}", rect1);
    println!("Area: {}", rect1.area());
    println!("Can hold rect2? {}", rect1.can_hold(&rect2));
}
```

## 🎯 Practice Exercises

1. Create a function that calculates the fibonacci sequence
2. Implement a temperature converter (Celsius to Fahrenheit)
3. Write a program that finds the largest number in a vector
4. Create a simple calculator using match expressions

Remember: Practice is key to mastering Rust fundamentals! 🚀
