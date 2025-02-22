# Ownership and Borrowing Exercises

## Exercise 1: String Manipulation with Ownership

Create a program that demonstrates string manipulation while respecting ownership rules:

1. Create a function that takes ownership of a string and returns its uppercase version
2. Create a function that borrows a string and counts its vowels
3. Implement a function that takes two strings and returns a new concatenated string
4. Demonstrate proper cleanup of resources

Example structure:
```rust
fn to_uppercase(s: String) -> String {
    // TODO: Convert string to uppercase and return it
}

fn count_vowels(s: &str) -> usize {
    // TODO: Count vowels without taking ownership
}

fn concatenate(s1: String, s2: String) -> String {
    // TODO: Concatenate strings and handle ownership
}
```

## Exercise 2: Vector Operations with References

Create a program that processes vectors using references:

1. Implement a function that finds the average of a vector using an immutable reference
2. Create a function that modifies a vector in-place using a mutable reference
3. Write a function that borrows two vectors and returns a new vector containing common elements
4. Ensure proper handling of empty vectors

Example structure:
```rust
fn calculate_average(numbers: &Vec<f64>) -> Option<f64> {
    // TODO: Calculate average without taking ownership
}

fn double_in_place(numbers: &mut Vec<i32>) {
    // TODO: Double each number in the vector
}

fn find_common_elements(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    // TODO: Find and return common elements
}
```

## Exercise 3: Custom Data Structure with Ownership

Implement a custom data structure that demonstrates ownership concepts:

1. Create a struct `Library` that owns a collection of books
2. Implement methods for adding and removing books
3. Create a method that loans out books (using references)
4. Implement the Drop trait for proper cleanup

```rust
struct Book {
    title: String,
    author: String,
    available: bool,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        // TODO: Initialize empty library
    }
    
    fn add_book(&mut self, title: String, author: String) {
        // TODO: Add new book
    }
    
    fn loan_book(&mut self, title: &str) -> Option<&Book> {
        // TODO: Implement book loaning system
    }
}
```

## Exercise 4: Lifetime Annotations

Create examples that demonstrate understanding of lifetime annotations:

1. Write a function that returns the longest of two string slices
2. Create a struct that holds references with explicit lifetimes
3. Implement methods for the struct that work with references
4. Demonstrate cases where lifetime elision works

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO: Return the longest string slice
}

struct TextAnalyzer<'a> {
    content: &'a str,
    current_word: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    // TODO: Implement methods that work with references
}
```

## Bonus Challenge: Memory-Safe Data Structure

Implement a memory-safe doubly-linked list:

1. Use `Rc<RefCell<T>>` for shared ownership
2. Implement basic operations (push, pop, insert)
3. Ensure no memory leaks
4. Handle circular references properly

Example structure:
```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    // TODO: Implement doubly-linked list operations
}
```

## Evaluation Criteria

Your solutions will be evaluated based on:

1. Correct implementation of ownership rules
2. Proper use of references and borrowing
3. Memory safety and resource management
4. Error handling and edge cases
5. Code organization and documentation

## Testing

For each exercise:

1. Write unit tests to verify behavior
2. Test edge cases and error conditions
3. Verify memory safety with tools like `valgrind`
4. Ensure no resource leaks
