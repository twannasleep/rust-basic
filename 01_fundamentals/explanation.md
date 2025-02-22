# Rust Fundamentals

## Variables and Data Types

### Variables

```rust
// Immutable by default
let x = 5;

// Mutable variables
let mut y = 10;

// Constants
const MAX_POINTS: u32 = 100_000;

// Shadowing
let spaces = "   ";
let spaces = spaces.len();
```

### Basic Data Types

```rust
// Integer types
let i: i32 = 42;
let u: u64 = 42;

// Floating-point
let f: f64 = 3.14;

// Boolean
let b: bool = true;

// Character
let c: char = 'z';

// Tuple
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Array
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

## Control Flow

### Conditionals

```rust
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}

// If in a let statement
let condition = true;
let number = if condition { 5 } else { 6 };
```

### Loops

```rust
// Loop
loop {
    println!("again!");
    break;
}

// While
while number != 0 {
    println!("{number}!");
    number -= 1;
}

// For
for element in arr.iter() {
    println!("value is: {element}");
}
```

## Functions

```rust
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
