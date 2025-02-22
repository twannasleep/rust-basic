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

## Exercise 1: File Processing System

Create a file processing system that demonstrates comprehensive error handling:

1. Create custom error types for:
   - File operations
   - Data parsing
   - Validation errors
2. Implement proper error conversion and propagation
3. Handle multiple error cases elegantly
4. Provide meaningful error messages

Example structure:

```rust
#[derive(Debug)]
enum FileProcessError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    ValidationError(String),
}

impl std::error::Error for FileProcessError {}

fn process_file(path: &str) -> Result<Vec<i32>, FileProcessError> {
    // TODO: Implement file processing with error handling
}
```

## Exercise 2: Configuration Parser

Implement a configuration parser with robust error handling:

1. Parse different data types (strings, numbers, booleans)
2. Handle missing fields and invalid values
3. Provide detailed error context
4. Implement custom error types and conversions

Example structure:

```rust
#[derive(Debug)]
struct Config {
    server_host: String,
    server_port: u16,
    max_connections: usize,
    timeout_seconds: u64,
}

#[derive(Debug)]
enum ConfigError {
    MissingField(String),
    InvalidValue { field: String, message: String },
    ParseError { field: String, error: String },
}

impl Config {
    fn from_file(path: &str) -> Result<Config, ConfigError> {
        // TODO: Implement config parsing with error handling
    }
}
```

## Exercise 3: Result and Option Combinators

Create a data processing pipeline using Result and Option combinators:

1. Implement a chain of transformations
2. Use map, and_then, or_else, filter
3. Handle errors at each step
4. Provide meaningful error context

Example structure:

```rust
#[derive(Debug)]
struct DataProcessor {
    data: Vec<String>,
}

impl DataProcessor {
    fn process_item(item: &str) -> Option<Result<i32, String>> {
        // TODO: Implement processing with combinators
    }
    
    fn transform_data(&self) -> Result<Vec<i32>, String> {
        // TODO: Implement transformation pipeline
    }
}
```

## Exercise 4: API Error Handling

Create a REST API-like system with proper error handling:

1. Define different types of API errors
2. Implement status codes and error messages
3. Handle validation and authentication
4. Provide detailed error responses

Example structure:

```rust
#[derive(Debug)]
enum ApiError {
    NotFound(String),
    Unauthorized(String),
    ValidationError { field: String, message: String },
    InternalError(String),
}

struct ApiResponse<T> {
    data: Option<T>,
    error: Option<ApiError>,
    status_code: u16,
}

impl<T> ApiResponse<T> {
    // TODO: Implement API response handling
}
```

## Bonus Challenge: Database Operations

Implement a mock database system with comprehensive error handling:

1. Handle connection errors
2. Deal with transaction failures
3. Manage constraint violations
4. Provide detailed error context and recovery options

Example structure:

```rust
#[derive(Debug)]
enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    TransactionError(String),
    ConstraintViolation { table: String, message: String },
}

struct Database {
    connected: bool,
    transactions: Vec<String>,
}

impl Database {
    fn execute_transaction(&mut self, query: &str) -> Result<(), DatabaseError> {
        // TODO: Implement database operations with error handling
    }
}
```

## Evaluation Criteria

Your solutions will be evaluated based on:

1. Proper use of Rust's error handling mechanisms
2. Error type design and implementation
3. Error context and messages
4. Error recovery strategies
5. Code organization and documentation

## Testing

For each exercise:

1. Write tests for success cases
2. Write tests for various error cases
3. Test error conversion and propagation
4. Verify error messages and context
