// Example: Working with External Crates
// This example demonstrates how to use external crates and organize code in a workspace

// External crate imports
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use log::{info, warn, error};

// Define a serializable struct using serde
#[derive(Debug, Serialize, Deserialize)]
struct LogEntry {
    timestamp: DateTime<Utc>,
    level: String,
    message: String,
}

// Module for logging functionality
mod logger {
    use super::*;
    use std::fs::OpenOptions;
    use std::io::Write;
    
    pub struct FileLogger {
        file_path: String,
    }
    
    impl FileLogger {
        pub fn new(file_path: &str) -> Self {
            FileLogger {
                file_path: file_path.to_string(),
            }
        }
        
        pub fn log(&self, level: &str, message: &str) -> std::io::Result<()> {
            let entry = LogEntry {
                timestamp: Utc::now(),
                level: level.to_string(),
                message: message.to_string(),
            };
            
            // Serialize the log entry
            let json = serde_json::to_string(&entry)?;
            
            // Append to file
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.file_path)?;
            
            writeln!(file, "{}", json)?;
            Ok(())
        }
    }
}

// Module for configuration
mod config {
    use std::path::PathBuf;
    
    pub struct Config {
        pub log_path: PathBuf,
        pub log_level: String,
    }
    
    impl Config {
        pub fn new() -> Self {
            Config {
                log_path: PathBuf::from("app.log"),
                log_level: "INFO".to_string(),
            }
        }
        
        pub fn with_log_path(mut self, path: &str) -> Self {
            self.log_path = PathBuf::from(path);
            self
        }
        
        pub fn with_log_level(mut self, level: &str) -> Self {
            self.log_level = level.to_string();
            self
        }
    }
}

// Example application using the modules
fn main() {
    // Initialize configuration
    let config = config::Config::new()
        .with_log_path("example.log")
        .with_log_level("INFO");
    
    // Create logger
    let logger = logger::FileLogger::new(
        config.log_path.to_str().unwrap()
    );
    
    // Log some messages
    if let Err(e) = logger.log("INFO", "Application started") {
        eprintln!("Failed to log message: {}", e);
    }
    
    // Using the log crate macros
    info!("Processing started");
    warn!("Resource usage high");
    error!("Operation failed");
    
    // Simulate some work
    for i in 1..=3 {
        if let Err(e) = logger.log(
            "INFO",
            &format!("Processing item {}", i)
        ) {
            eprintln!("Failed to log message: {}", e);
        }
    }
}

// Example of workspace organization in Cargo.toml:
/*
[workspace]
members = [
    "core",
    "api",
    "cli"
]

[package]
name = "example_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    
    #[test]
    fn test_config_builder() {
        let config = config::Config::new()
            .with_log_path("test.log")
            .with_log_level("DEBUG");
        
        assert_eq!(config.log_path, Path::new("test.log"));
        assert_eq!(config.log_level, "DEBUG");
    }
    
    #[test]
    fn test_logger() {
        let test_log = "test.log";
        let logger = logger::FileLogger::new(test_log);
        
        // Log a test message
        logger.log("TEST", "Test message").unwrap();
        
        // Verify file exists and contains content
        assert!(Path::new(test_log).exists());
        let content = fs::read_to_string(test_log).unwrap();
        assert!(content.contains("Test message"));
        
        // Cleanup
        fs::remove_file(test_log).unwrap();
    }
} 