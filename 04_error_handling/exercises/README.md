# Error Handling Exercises

## 🌟 Exercise 1: Custom Error Types

Create a user authentication system that:

```rust
// Define custom error types for authentication
#[derive(Debug)]
enum AuthError {
    InvalidUsername,
    WeakPassword,
    UserNotFound,
    PasswordMismatch,
    DatabaseError(String),
}

struct User {
    username: String,
    password_hash: String,
}

// TODO: Implement these functions with proper error handling
impl User {
    fn new(username: &str, password: &str) -> Result<User, AuthError> {
        // Validate username and password
        // Hash password
        // Create user
    }

    fn authenticate(&self, password: &str) -> Result<(), AuthError> {
        // Verify password
        // Return success or appropriate error
    }
}
```

**Skills practiced:**

- Custom error types
- Error conversion
- Result handling
- Password validation

## 🌟🌟 Exercise 2: Error Propagation

Implement a file processing system that:

```rust
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug)]
enum FileError {
    NotFound(String),
    AccessDenied(String),
    InvalidFormat(String),
    ProcessingError(String),
}

// TODO: Implement these functions with error propagation
fn read_config_file(path: &Path) -> Result<Config, FileError> {
    // Read file
    // Parse contents
    // Validate configuration
}

fn process_data_file(path: &Path) -> Result<Vec<Record>, FileError> {
    // Read file
    // Parse records
    // Validate data
}

fn save_results(records: &[Record], output: &Path) -> Result<(), FileError> {
    // Format data
    // Write to file
    // Verify write
}
```

**Skills practiced:**

- Error propagation
- The ? operator
- Error context
- File operations

## 🌟🌟 Exercise 3: Option Handling

Create a data processing pipeline that:

```rust
struct DataPoint {
    value: Option<f64>,
    timestamp: u64,
    metadata: Option<String>,
}

struct ProcessedData {
    average: f64,
    valid_points: usize,
    description: String,
}

// TODO: Implement these functions with Option handling
impl DataPoint {
    fn validate(&self) -> Option<f64> {
        // Check if value is valid
        // Apply validation rules
    }

    fn with_metadata(&self) -> Option<(f64, String)> {
        // Combine value and metadata if both exist
    }
}

fn process_data_points(points: &[DataPoint]) -> Option<ProcessedData> {
    // Calculate average of valid points
    // Generate statistics
    // Create summary
}
```

**Skills practiced:**

- Option combinations
- Iterator methods
- Data validation
- Optional chaining

## 🌟🌟🌟 Exercise 4: Advanced Error Handling

Implement a database interaction layer with:

```rust
#[derive(Debug)]
enum DbError {
    ConnectionError(String),
    QueryError(String),
    ValidationError(String),
    TransactionError(String),
}

struct Database {
    connection: Connection,
    transaction_active: bool,
}

// TODO: Implement database operations with comprehensive error handling
impl Database {
    fn execute_transaction<F>(&mut self, operations: F) -> Result<(), DbError>
    where
        F: FnOnce(&mut Database) -> Result<(), DbError>
    {
        // Start transaction
        // Execute operations
        // Handle rollback on error
        // Commit if successful
    }

    fn query_with_retry<T>(&self, query: &str) -> Result<T, DbError> {
        // Implement retry logic
        // Handle temporary failures
        // Apply backoff strategy
    }
}
```

**Skills practiced:**

- Complex error handling
- Error conversion
- Transaction management
- Retry logic

## 🌟🌟🌟 Exercise 5: Error Context and Logging

Create a service layer with contextual error handling:

```rust
use std::error::Error;
use log::{error, warn, info};

#[derive(Debug)]
struct ErrorContext<E> {
    error: E,
    context: String,
    severity: ErrorSeverity,
}

enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// TODO: Implement service with contextual error handling
struct Service {
    // Add necessary fields
}

impl Service {
    fn process_request(&self, request: Request) -> Result<Response, ServiceError> {
        // Add context to errors
        // Log appropriate information
        // Handle different severity levels
    }

    fn with_context<T, E, F>(&self, context: &str, op: F) -> Result<T, ErrorContext<E>>
    where
        F: FnOnce() -> Result<T, E>,
        E: Error,
    {
        // Implement context wrapper
        // Add logging
        // Handle severity
    }
}
```

**Skills practiced:**

- Error context
- Logging integration
- Error severity
- Service architecture

## Tips

1. Always provide context with errors
2. Use appropriate error types for different scenarios
3. Consider using error handling crates like `anyhow` or `thiserror`
4. Implement proper error conversion
5. Add logging at appropriate levels

## Evaluation Criteria

- Error type design
- Error propagation patterns
- Context handling
- Recovery strategies
- Logging implementation
