# 🏗️ Types and Structs in Rust

## 📦 Custom Types Overview

Rust provides powerful tools for creating custom types that are both safe and expressive. This guide covers the main ways to define and use custom types in Rust.

## 🔧 Structs

### Types of Structs

1. **Named-Field Structs**
2. **Tuple Structs**
3. **Unit Structs**

```rust
// Named-field struct
#[derive(Debug)]
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
```

### Working with Structs

```rust
// Creating instances
let mut user = User {
    username: String::from("johndoe"),
    email: String::from("john@example.com"),
    sign_in_count: 1,
    active: true,
};

// Accessing fields
println!("Username: {}", user.username);

// Updating fields
user.sign_in_count += 1;

// Struct update syntax
let user2 = User {
    email: String::from("jane@example.com"),
    username: String::from("janedoe"),
    ..user  // Copy remaining fields from user
};
```

### Methods and Associated Functions

```rust
impl User {
    // Associated function (constructor)
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }
    
    // Method
    fn login(&mut self) {
        self.sign_in_count += 1;
    }
    
    // Getter method
    fn username(&self) -> &str {
        &self.username
    }
}

// Usage
let mut user = User::new("alice".to_string(), "alice@example.com".to_string());
user.login();
```

## 🔄 Enums

### Basic Enums

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // Unit variant
    Move { x: i32, y: i32 },   // Struct variant
    Write(String),             // Tuple variant
    ChangeColor(Color),        // Custom type variant
}
```

### Pattern Matching with Enums

```rust
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(color) => println!("Changing color: {:?}", color),
        }
    }
}
```

### Option and Result

```rust
// Option enum for nullable values
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

// Result enum for error handling
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(x / y)
    }
}
```

## 🎨 Traits

### Defining and Implementing Traits

```rust
// Define a trait
trait Summary {
    // Required method
    fn summarize(&self) -> String;
    
    // Default implementation
    fn default_behavior(&self) -> String {
        String::from("Default implementation")
    }
}

// Implement trait for User
impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
}
```

### Trait Bounds

```rust
// Single trait bound
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
use std::fmt::Display;
fn complex<T: Summary + Display>(item: &T) {
    println!("{}", item);
    println!("{}", item.summarize());
}

// Where clause syntax
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    // Implementation
    42
}
```

## 🧬 Generics

### Generic Structs

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Different type parameters
struct KeyValue<K, V> {
    key: K,
    value: V,
}
```

### Generic Implementation

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Specific type implementation
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### Generic Functions

```rust
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

## 🛡️ Type Safety Features

### Type Aliases

```rust
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, CustomError>;
```

### Newtype Pattern

```rust
struct Email(String);

impl Email {
    fn new(address: &str) -> Result<Email, String> {
        if address.contains('@') {
            Ok(Email(address.to_string()))
        } else {
            Err("Invalid email address".to_string())
        }
    }
}
```

### Zero-Sized Types

```rust
struct Unit;
struct PhantomData<T>;
```

## 🎯 Best Practices

### 1. Type Design

- Keep types focused and single-purpose
- Use meaningful names
- Document public interfaces
- Implement common traits where appropriate

```rust
#[derive(Debug, Clone, PartialEq)]
struct Temperature {
    degrees: f64,
    scale: TemperatureScale,
}
```

### 2. Error Handling

- Use Result for fallible operations
- Implement Error trait for custom errors
- Provide helpful error messages

```rust
#[derive(Debug)]
struct ParseTemperatureError(String);

impl Temperature {
    fn try_from_celsius(degrees: f64) -> Result<Temperature, ParseTemperatureError> {
        if degrees < -273.15 {
            Err(ParseTemperatureError("Temperature below absolute zero".to_string()))
        } else {
            Ok(Temperature {
                degrees,
                scale: TemperatureScale::Celsius,
            })
        }
    }
}
```

### 3. API Design

- Make invalid states unrepresentable
- Use builder pattern for complex construction
- Provide convenience methods

```rust
impl Temperature {
    fn to_fahrenheit(&self) -> f64 {
        match self.scale {
            TemperatureScale::Celsius => self.degrees * 9.0/5.0 + 32.0,
            TemperatureScale::Fahrenheit => self.degrees,
        }
    }
}
```

## 🔍 Examples in Practice

### Complete Type System Example

```rust
#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: ISBN,
}

#[derive(Debug)]
struct ISBN(String);

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }
    
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    
    fn find_by_isbn(&self, isbn: &ISBN) -> Option<&Book> {
        self.books.iter().find(|book| book.isbn == *isbn)
    }
}

fn main() {
    let mut library = Library::new();
    
    let book = Book {
        title: String::from("Rust Programming"),
        author: String::from("John Doe"),
        isbn: ISBN(String::from("978-0-00-000000-0")),
    };
    
    library.add_book(book);
    
    if let Some(found_book) = library.find_by_isbn(&ISBN(String::from("978-0-00-000000-0"))) {
        println!("Found book: {:?}", found_book);
    }
}
```

Remember: Strong type systems help catch errors at compile time rather than runtime! 🚀
