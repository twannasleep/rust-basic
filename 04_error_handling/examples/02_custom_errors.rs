// Example: Custom Error Types and Advanced Error Handling
// This example demonstrates creating and using custom error types

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::num::ParseIntError;

// Custom error type
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(ParseIntError),
    ValidationError(String),
    DatabaseError { code: i32, message: String },
}

// Implement Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "I/O error: {}", err),
            AppError::ParseError(err) => write!(f, "Parse error: {}", err),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::DatabaseError { code, message } => {
                write!(f, "Database error {}: {}", code, message)
            }
        }
    }
}

// Implement Error trait for AppError
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::IoError(err) => Some(err),
            AppError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

// Implement From traits for automatic conversion
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::IoError(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> AppError {
        AppError::ParseError(err)
    }
}

// Type alias for Result with AppError
type AppResult<T> = Result<T, AppError>;

// Struct representing a user
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: u8,
}

// User-related functions that can fail
impl User {
    fn new(id: i32, name: String, age: u8) -> AppResult<User> {
        if name.is_empty() {
            return Err(AppError::ValidationError("Name cannot be empty".into()));
        }
        if age < 18 {
            return Err(AppError::ValidationError("User must be 18 or older".into()));
        }
        Ok(User { id, name, age })
    }
    
    fn save(&self) -> AppResult<()> {
        // Simulate database error
        Err(AppError::DatabaseError {
            code: 1001,
            message: "Connection failed".into(),
        })
    }
}

// Function that combines multiple error types
fn process_user_data(path: &str) -> AppResult<User> {
    // Read file contents (can fail with IoError)
    let contents = std::fs::read_to_string(path)?;
    
    // Split the contents
    let parts: Vec<&str> = contents.split(',').collect();
    if parts.len() != 3 {
        return Err(AppError::ValidationError("Invalid data format".into()));
    }
    
    // Parse the parts (can fail with ParseError)
    let id = parts[0].parse::<i32>()?;
    let name = parts[1].to_string();
    let age = parts[2].parse::<u8>()?;
    
    // Create and return user (can fail with ValidationError)
    User::new(id, name, age)
}

fn main() {
    // Example 1: Handling file operations
    println!("Attempting to read file...");
    match File::open("nonexistent.txt") {
        Ok(_) => println!("File opened successfully"),
        Err(e) => {
            let app_error: AppError = e.into();
            println!("Error: {}", app_error);
        }
    }
    
    // Example 2: Creating a user with validation
    println!("\nAttempting to create user...");
    match User::new(1, "Alice".to_string(), 15) {
        Ok(user) => println!("User created: {:?}", user),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 3: Complex error handling
    println!("\nAttempting to process user data...");
    match process_user_data("user.txt") {
        Ok(user) => {
            println!("User processed: {:?}", user);
            if let Err(e) = user.save() {
                println!("Failed to save user: {}", e);
            }
        }
        Err(e) => {
            println!("Error processing user: {}", e);
            if let Some(source) = e.source() {
                println!("Caused by: {}", source);
            }
        }
    }
    
    // Example 4: Error handling with context
    fn read_config() -> AppResult<String> {
        std::fs::read_to_string("config.txt").map_err(|e| {
            AppError::ValidationError(format!("Failed to read config: {}", e))
        })
    }
    
    println!("\nAttempting to read config...");
    match read_config() {
        Ok(config) => println!("Config: {}", config),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_validation() {
        // Test empty name
        assert!(matches!(
            User::new(1, "".into(), 20),
            Err(AppError::ValidationError(_))
        ));
        
        // Test underage
        assert!(matches!(
            User::new(1, "Test".into(), 15),
            Err(AppError::ValidationError(_))
        ));
        
        // Test valid user
        assert!(User::new(1, "Test".into(), 20).is_ok());
    }
    
    #[test]
    fn test_error_conversion() {
        // Test io::Error conversion
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();
        assert!(matches!(app_error, AppError::IoError(_)));
        
        // Test ParseIntError conversion
        let parse_error = "not a number".parse::<i32>().unwrap_err();
        let app_error: AppError = parse_error.into();
        assert!(matches!(app_error, AppError::ParseError(_)));
    }
} 