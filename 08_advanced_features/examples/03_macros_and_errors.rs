// Example: Macros and Advanced Error Handling
// This example demonstrates macro creation and advanced error handling patterns

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::backtrace::Backtrace;

// =============== Declarative Macros ===============

// Simple macro for creating a vector of strings
#[macro_export]
macro_rules! vec_strs {
    // Match zero or more expressions separated by commas
    ($($element:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push(String::from($element));
        )*
        v
    }};
}

// Macro for simplified debug printing
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    }};
}

// Macro for creating test cases
#[macro_export]
macro_rules! create_test {
    ($name:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($value, $expected);
        }
    };
}

// =============== Advanced Error Handling ===============

// Custom error type with source and backtrace
#[derive(Debug)]
struct AppError {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
    backtrace: Backtrace,
}

#[derive(Debug)]
enum ErrorKind {
    Io,
    Parse,
    Validation,
}

impl AppError {
    fn new(
        kind: ErrorKind,
        message: impl Into<String>,
        source: Option<Box<dyn Error + Send + Sync>>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source,
            backtrace: Backtrace::capture(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)?;
        if let Some(source) = &self.source {
            write!(f, "\nCaused by:\n\t{}", source)?;
        }
        Ok(())
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

// Error conversion implementations
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::new(
            ErrorKind::Io,
            "IO operation failed",
            Some(Box::new(error)),
        )
    }
}

// Result type alias
type Result<T> = std::result::Result<T, AppError>;

// Function that might fail
fn read_file(path: &str) -> Result<String> {
    let file = File::open(path).map_err(|e| {
        AppError::new(
            ErrorKind::Io,
            format!("Failed to open file: {}", path),
            Some(Box::new(e)),
        )
    })?;
    
    // Use ? operator for error propagation
    let contents = std::io::read_to_string(file)?;
    
    if contents.is_empty() {
        return Err(AppError::new(
            ErrorKind::Validation,
            "File is empty",
            None,
        ));
    }
    
    Ok(contents)
}

// =============== Main Function ===============

fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Demonstrate macro usage
    let strings = vec_strs!["hello", "world", "macro"];
    println!("Created strings: {:?}", strings);
    
    debug_print!("Debug mode is active");
    
    // Demonstrate error handling
    match read_file("nonexistent.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("Caused by: {}", source);
            }
            println!("Backtrace:\n{:?}", e.backtrace);
        }
    }
    
    Ok(())
}

// =============== Tests ===============

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_strs_macro() {
        let v = vec_strs!["a", "b", "c"];
        assert_eq!(v, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }
    
    // Using the create_test macro
    create_test!(test_addition, 2 + 2, 4);
    
    #[test]
    fn test_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_error = AppError::from(io_error);
        assert!(matches!(app_error.kind, ErrorKind::Io));
    }
    
    #[test]
    fn test_read_file_error() {
        let result = read_file("nonexistent.txt");
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(matches!(e.kind, ErrorKind::Io));
 