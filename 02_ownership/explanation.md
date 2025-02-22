# Ownership and Borrowing

## Ownership Rules

1. Each value has a single owner
2. Only one owner at a time
3. Value is dropped when owner goes out of scope

### Basic Ownership

```rust
fn main() {
    // String type demonstrates ownership
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // This would not compile!
    
    // Clone for deep copy
    let s3 = String::from("hello");
    let s4 = s3.clone();  // Both s3 and s4 are valid
}
```

## References and Borrowing

### Borrowing Rules

1. Any number of immutable references OR
2. Exactly one mutable reference
3. References must be valid

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Immutable borrow
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // Mutable borrow
    let r3 = &mut s;
    r3.push_str(" world");
}

// Function borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## Slices

```rust
fn main() {
    let s = String::from("hello world");
    
    // String slices
    let hello = &s[0..5];
    let world = &s[6..11];
    
    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
}

// String slice parameter
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

## Lifetime Annotations

```rust
// Lifetime annotations in functions
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime in structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime elision rules
fn first_word(s: &str) -> &str {  // Lifetimes are elided here
    // Implementation
}
```
