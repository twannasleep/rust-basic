# Error Handling in Rust

## Understanding Error Types

### Result<T, E>

```rust
// Basic Result usage
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(x / y)
    }
}

// Using Result
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

### Option<T>

```rust
// Basic Option usage
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

// Using Option
match find_user(1) {
    Some(name) => println!("User found: {}", name),
    None => println!("User not found"),
}
```

## Error Handling Techniques

### The ? Operator

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("config.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### Custom Error Types

```rust
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    Custom(String),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}
```

## Error Propagation Patterns

### Using map and and_then

```rust
fn process_data() -> Result<i32, String> {
    let raw = get_raw_data()
        .map_err(|e| format!("Data error: {}", e))?;
    
    process_raw_data(&raw)
        .and_then(|n| {
            if n > 0 {
                Ok(n * 2)
            } else {
                Err("Number must be positive".to_string())
            }
        })
}
```

### Multiple Error Types

```rust
use std::error::Error;

fn complex_operation() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.txt")?;
    let parsed: i32 = str::parse("123")?;
    // More operations...
    Ok(())
}
```

## Advanced Error Handling

### Error Context

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorContext<E> {
    error: E,
    context: String,
}

impl<E: Error> Error for ErrorContext<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<E: fmt::Display> fmt::Display for ErrorContext<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.error)
    }
}
```

### Result Combinators

```rust
fn process_number(input: &str) -> Result<i32, String> {
    input
        .parse::<i32>()
        .map(|n| n * 2)
        .map_err(|e| format!("Failed to parse: {}", e))
}

// Chaining operations
fn complex_process(input: &str) -> Result<String, String> {
    input
        .parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))
        .and_then(|n| {
            if n > 0 {
                Ok(n)
            } else {
                Err("Number must be positive".to_string())
            }
        })
        .map(|n| format!("Processed: {}", n))
}
```

### Option Combinators

```rust
fn process_optional(value: Option<i32>) -> Option<String> {
    value
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .and_then(|x| {
            if x < 100 {
                Some(format!("Value: {}", x))
            } else {
                None
            }
        })
}
```

## Best Practices

### Error Handling Guidelines

```rust
// 1. Use custom error types for domain-specific errors
#[derive(Debug)]
enum DatabaseError {
    ConnectionFailed,
    QueryFailed(String),
    DataCorrupted,
}

// 2. Implement std::error::Error for custom errors
impl std::error::Error for DatabaseError {}

// 3. Provide context with error messages
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed => 
                write!(f, "Failed to connect to database"),
            DatabaseError::QueryFailed(query) => 
                write!(f, "Query failed: {}", query),
            DatabaseError::DataCorrupted => 
                write!(f, "Database data is corrupted"),
        }
    }
}

// 4. Use type aliases for common Result types
type DbResult<T> = Result<T, DatabaseError>;
```
