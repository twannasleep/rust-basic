# Ownership and Borrowing in Rust

Ownership is Rust's most unique feature and has deep implications for how the language works. It enables Rust to make memory safety guarantees without needing a garbage collector.

## Ownership Rules

1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

## Understanding Ownership

### Move Semantics

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// println!("{}", s1); // This would cause an error!
```

### Clone for Deep Copy

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Creates a deep copy
println!("s1 = {}, s2 = {}", s1, s2); // This works!
```

### Copy for Stack-Only Data

Types that implement the Copy trait:

- All integer types
- Boolean type
- Floating point types
- Character type
- Tuples if they only contain types that are also Copy

## References and Borrowing

References allow you to refer to some value without taking ownership of it.

### Borrowing Rules

1. At any given time, you can have either:
   - One mutable reference
   - Any number of immutable references
2. References must always be valid

### Examples

```rust
fn calculate_length(s: &String) -> usize { // s is a reference
    s.len()
} // s goes out of scope, but because it's a reference, nothing happens

fn change_string(s: &mut String) {
    s.push_str(" world");
}
```

## The Slice Type

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

### String Slices

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

### Other Slices

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

## Memory Management

Rust's ownership system ensures memory safety at compile time:

- No null pointer dereferencing
- No dangling references
- No double free
- No memory leaks (in safe code)

### Stack vs Heap

- Stack: Fixed size, fast access, LIFO
- Heap: Dynamic size, slower access, managed through ownership

## Common Patterns

### Taking Ownership

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
} // s is dropped here
```

### Borrowing with References

```rust
fn borrows(s: &String) {
    println!("{}", s);
} // s reference is dropped, but the String is still valid
```

### Mutable References

```rust
fn mutate(s: &mut String) {
    s.push_str(" world");
}
```

## Exercises

1. Create a function that takes ownership of a String and returns ownership
2. Write a function that borrows a String and calculates its length
3. Experiment with mutable and immutable references
4. Create and use string slices
5. Practice with different types of ownership transfers

## Next Steps

After completing this section, you should:

- Understand Rust's ownership system
- Know how to use references and borrowing
- Be able to work with slices
- Understand memory management in Rust
- Be comfortable with ownership transfers
