# 🚨 Error Handling in Rust

## 📋 Overview

Rust's error handling is designed to be:

- Explicit and clear
- Type-safe
- Zero-cost when unused
- Impossible to ignore unintentionally

## 🎯 Understanding Error Types

### Result<T, E>

The `Result` enum is Rust's primary error handling type:

```rust
enum Result<T, E> {
    Ok(T),    // Success case containing value
    Err(E),   // Error case containing error
}
```

#### Basic Usage

```rust
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

// Using if let
if let Ok(result) = divide(10.0, 2.0) {
    println!("Result: {}", result);
}
```

### Option<T>

`Option` represents optional values:

```rust
fn find_user(id: u32) -> Option<String> {
    let users = vec!["Alice", "Bob", "Charlie"];
    if id < users.len() as u32 {
        Some(users[id as usize].to_string())
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

## 🛠️ Error Handling Techniques

### The ? Operator

Simplifies error propagation:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("config.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Without ? operator (equivalent)
fn read_file_verbose() -> Result<String, io::Error> {
    let mut file = match File::open("config.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}
```

### Custom Error Types

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "I/O error: {}", err),
            AppError::ParseError(err) => write!(f, "Parse error: {}", err),
            AppError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::IoError(err) => Some(err),
            AppError::ParseError(err) => Some(err),
            AppError::Custom(_) => None,
        }
    }
}

// Implement From for automatic conversion
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}
```

## 🔄 Error Propagation Patterns

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

## 🎨 Advanced Error Handling

### Error Context

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorContext<E> {
    error: E,
    context: String,
    backtrace: std::backtrace::Backtrace,
}

impl<E: Error> Error for ErrorContext<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<E: fmt::Display> fmt::Display for ErrorContext<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}\n\nBacktrace:\n{:?}", 
            self.context, self.error, self.backtrace)
    }
}
```

### Result Combinators

```rust
// Option combinators
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

// Result combinators
fn process_result(input: Result<i32, String>) -> Result<String, String> {
    input
        .map(|x| x * 2)
        .and_then(|x| {
            if x > 100 {
                Ok(format!("Large value: {}", x))
            } else {
                Err("Value too small".to_string())
            }
        })
}
```

## 🎯 Best Practices

### 1. Error Type Design

```rust
#[derive(Debug, thiserror::Error)]
enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Query failed: {query}")]
    QueryFailed {
        query: String,
        #[source]
        source: sqlx::Error,
    },
    
    #[error("Data corrupted")]
    DataCorrupted,
}
```

### 2. Error Handling Patterns

```rust
// Pattern 1: Early return with context
fn process_config() -> Result<Config, AppError> {
    let file = File::open("config.json").map_err(|e| {
        AppError::Custom(format!("Failed to open config: {}", e))
    })?;
    
    let config = serde_json::from_reader(file).map_err(|e| {
        AppError::Custom(format!("Invalid config format: {}", e))
    })?;
    
    Ok(config)
}

// Pattern 2: Fallback values
fn get_setting() -> i32 {
    std::env::var("SETTING")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42)  // Default value
}
```

### 3. Testing Error Cases

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_handling() {
        // Test success case
        assert!(process_positive(42).is_ok());
        
        // Test error case
        let err = process_positive(-1).unwrap_err();
        assert_eq!(err.to_string(), "Number must be positive");
        
        // Test error conversion
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_error = AppError::from(io_error);
        assert!(matches!(app_error, AppError::IoError(_)));
    }
}
```

## 🔍 Complete Example

```rust
use thiserror::Error;
use std::fs::File;
use std::io::Read;

#[derive(Error, Debug)]
enum DataProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("Invalid data: {0}")]
    Validation(String),
}

struct DataProcessor {
    filename: String,
}

impl DataProcessor {
    fn new(filename: &str) -> Self {
        Self { filename: filename.to_string() }
    }
    
    fn process(&self) -> Result<Vec<i32>, DataProcessingError> {
        // Read file
        let mut file = File::open(&self.filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // Process each line
        contents
            .lines()
            .map(|line| {
                let number = line.parse::<i32>()?;
                if number < 0 {
                    Err(DataProcessingError::Validation(
                        format!("Negative number not allowed: {}", number)
                    ))
                } else {
                    Ok(number)
                }
            })
            .collect()
    }
}

fn main() {
    let processor = DataProcessor::new("data.txt");
    match processor.process() {
        Ok(numbers) => println!("Processed numbers: {:?}", numbers),
        Err(e) => {
            eprintln!("Error: {}", e);
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source);
            }
        }
    }
}
```

Remember: Good error handling makes your code more reliable and maintainable! 🚀
