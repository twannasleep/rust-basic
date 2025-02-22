# Ownership and Borrowing Exercises

## Exercise 1: String Manipulation

Implement functions that demonstrate ownership concepts:

```rust
fn main() {
    // TODO: Implement string manipulation examples
    let s1 = String::from("hello");
    // Add your code here
}

fn take_ownership(s: String) {
    // TODO: Implement ownership transfer
}

fn borrow_string(s: &String) {
    // TODO: Implement borrowing
}
```

## Exercise 2: Vector Operations

Create a program that manages a vector of integers:

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    // TODO: Implement vector operations
}

fn process_vector(v: &mut Vec<i32>) {
    // TODO: Implement mutable borrowing
}
```

## Exercise 3: Custom Data Structure

Implement a custom data structure with ownership semantics:

```rust
struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    author: String,
}

impl Library {
    // TODO: Implement methods demonstrating ownership
}
```

## Exercise 4: Lifetime Practice

Work with lifetime annotations:

```rust
// TODO: Implement struct with lifetime parameters
struct Excerpt<'a> {
    text: &'a str,
}

// TODO: Implement functions with explicit lifetimes
fn longest_word<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Implementation
}
```
