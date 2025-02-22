# Types and Structs

## Custom Types

### Structs

```rust
// Basic struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple struct
struct Color(i32, i32, i32);

// Unit struct
struct AlwaysEqual;

// Struct implementation
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }
    
    fn login(&mut self) {
        self.sign_in_count += 1;
    }
}
```

### Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

impl Message {
    fn call(&self) {
        // Method implementation
    }
}

// Option enum
enum Option<T> {
    Some(T),
    None,
}

// Result enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Traits

```rust
// Define a trait
trait Summary {
    fn summarize(&self) -> String;
    
    fn default_behavior(&self) -> String {
        String::from("Default implementation")
    }
}

// Implement trait
impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
}

// Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn complex<T: Summary + Display>(item: &T) {}
```

## Generics

```rust
// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic implementation
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generic function
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
