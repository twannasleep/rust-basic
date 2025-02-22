# Types and Structs Exercises

## Exercise 1: Custom Types

Create a basic shape system:

```rust
// TODO: Implement these structs and enums
struct Rectangle {
    width: u32,
    height: u32,
}

struct Circle {
    radius: f64,
}

enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

trait Area {
    fn area(&self) -> f64;
}

// TODO: Implement Area for Rectangle and Circle
```

## Exercise 2: Method Implementation

Implement a basic banking system:

```rust
struct Account {
    holder: String,
    balance: f64,
    account_type: AccountType,
}

enum AccountType {
    Savings,
    Checking,
}

impl Account {
    // TODO: Implement these methods
    fn new(holder: String, account_type: AccountType) -> Account {
        // Implementation
    }
    
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        // Implementation
    }
    
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        // Implementation
    }
}
```

## Exercise 3: Traits and Generics

Create a data processing system:

```rust
trait Processable {
    fn process(&self) -> String;
    fn validate(&self) -> bool;
}

struct DataPoint<T> {
    value: T,
    timestamp: u64,
}

// TODO: Implement Processable for different DataPoint types
impl<T: std::fmt::Display> Processable for DataPoint<T> {
    // Implementation
}

// TODO: Create a generic data processor
struct DataProcessor<T> {
    data: Vec<DataPoint<T>>,
}
```

## Exercise 4: Advanced Patterns

Work with advanced type patterns:

```rust
// TODO: Implement a builder pattern
struct ServerBuilder {
    host: Option<String>,
    port: Option<u16>,
    max_connections: Option<u32>,
}

// TODO: Implement type state pattern
struct Draft;
struct Published;

struct Post<State> {
    content: String,
    state: State,
}

impl Post<Draft> {
    // Implementation
}
```
