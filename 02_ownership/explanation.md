# 🔐 Ownership and Borrowing in Rust

## 📋 Overview

Rust's ownership system is its most unique and powerful feature, enabling memory safety without garbage collection. This guide will help you master these core concepts.

## 🎯 Ownership Rules

Three fundamental rules govern ownership in Rust:

1. Each value has exactly one owner
2. Only one owner at a time
3. Owner goes out of scope → value is dropped

```rust
fn ownership_example() {
    let s1 = String::from("hello");    // s1 owns the string
    let s2 = s1;                       // ownership moves to s2
    // println!("{}", s1);             // ❌ Error: s1 no longer valid
    println!("{}", s2);                // ✅ Works fine
}
```

## 🔄 Move vs. Copy

### Types that Implement Copy

| Type | Description | Example |
|------|-------------|---------|
| Integers | All integer types | `i32`, `u64` |
| Booleans | `true` or `false` | `bool` |
| Floating point | All float types | `f32`, `f64` |
| Characters | Unicode scalar values | `char` |
| Tuples | If all elements implement Copy | `(i32, i32)` |

```rust
fn copy_types_example() {
    // Copy types are duplicated
    let x = 5;
    let y = x;    // x is copied to y
    println!("x = {}, y = {}", x, y);  // Both valid

    // Move types transfer ownership
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // ❌ Error
}
```

## 📚 References and Borrowing

### Borrowing Rules

1. Any number of immutable references (`&T`)
2. Exactly one mutable reference (`&mut T`)
3. Cannot have mutable and immutable references simultaneously

```rust
fn borrowing_example() {
    let mut string = String::from("hello");
    
    // Immutable borrowing
    let r1 = &string;
    let r2 = &string;
    println!("{} and {}", r1, r2);
    
    // Mutable borrowing
    let r3 = &mut string;
    r3.push_str(" world");
    println!("{}", r3);
}
```

### Reference Lifetimes

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 🔪 Slices

Slices are references to a contiguous sequence of elements in a collection.

### String Slices

```rust
fn string_slices_example() {
    let s = String::from("hello world");
    
    let hello = &s[0..5];    // "hello"
    let world = &s[6..11];   // "world"
    let entire = &s[..];     // "hello world"
    
    // String literals are slices
    let literal: &str = "hello world";
}
```

### Array Slices

```rust
fn array_slices_example() {
    let numbers = [1, 2, 3, 4, 5];
    
    let slice = &numbers[1..4];  // [2, 3, 4]
    assert_eq!(slice, &[2, 3, 4]);
}
```

## 🛠️ Practical Examples

### String Manipulation

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s);
    println!("First word: {}", word);  // "Hello"
}
```

### Safe Data Sharing

```rust
#[derive(Debug)]
struct SharedData {
    name: String,
    value: i32,
}

fn process_data(data: &mut SharedData) {
    data.value += 1;
    println!("Processing: {:?}", data);
}

fn main() {
    let mut data = SharedData {
        name: String::from("example"),
        value: 42,
    };
    
    process_data(&mut data);
    println!("After processing: {:?}", data);
}
```

## 🎓 Advanced Patterns

### Multiple Borrows

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }
    
    fn value(&self) -> i32 {
        self.count
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    
    // Mutable borrow for increment
    counter.increment();
    
    // Multiple immutable borrows
    let value1 = counter.value();
    let value2 = counter.value();
    println!("Values: {} {}", value1, value2);
}
```

### Temporary Ownership Transfer

```rust
struct Resource {
    data: String,
}

impl Resource {
    fn process(&mut self) {
        let temp = std::mem::take(&mut self.data);
        // Process temp...
        self.data = temp.to_uppercase();
    }
}
```

## ⚠️ Common Pitfalls and Solutions

1. **Borrowing After Move**

   ```rust
   let s = String::from("hello");
   let s2 = s;  // s is moved
   // println!("{}", s);  // ❌ Error
   ```

   Solution: Clone if you need both copies

   ```rust
   let s = String::from("hello");
   let s2 = s.clone();  // s is cloned
   println!("{}", s);  // ✅ Works
   ```

2. **Multiple Mutable Borrows**

   ```rust
   let mut s = String::from("hello");
   let r1 = &mut s;
   let r2 = &mut s;  // ❌ Error
   ```

   Solution: Use scopes

   ```rust
   let mut s = String::from("hello");
   {
       let r1 = &mut s;
       // use r1
   }  // r1 goes out of scope
   let r2 = &mut s;  // ✅ Works
   ```

## 🎯 Best Practices

1. **Use References When Possible**
   - Prefer `&str` over `String` for parameters
   - Use `&[T]` instead of `Vec<T>` when possible
   - Return owned types only when necessary

2. **Lifetime Annotations**
   - Keep them simple and clear
   - Use lifetime elision when possible
   - Document complex lifetime relationships

3. **Error Prevention**
   - Use clippy to catch common mistakes
   - Write tests for ownership patterns
   - Document ownership requirements

Remember: Rust's ownership system might feel restrictive at first, but it prevents many common programming errors! 🚀
