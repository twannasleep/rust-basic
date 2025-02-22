// Solution: File Processing System
// This solution demonstrates comprehensive error handling for file operations

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

// Custom error type
#[derive(Debug)]
enum FileProcessError {
    IoError(io::Error),
    ParseError { line: usize, error: ParseIntError },
    ValidationError { line: usize, message: String },
    EmptyFileError,
}

// Implement Display for FileProcessError
impl fmt::Display for FileProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileProcessError::IoError(err) => write!(f, "I/O error: {}", err),
            FileProcessError::ParseError { line, error } => {
                write!(f, "Parse error at line {}: {}", line, error)
            }
            FileProcessError::ValidationError { line, message } => {
                write!(f, "Validation error at line {}: {}", line, message)
            }
            FileProcessError::EmptyFileError => write!(f, "File is empty"),
        }
    }
}

// Implement Error trait
impl Error for FileProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileProcessError::IoError(err) => Some(err),
            FileProcessError::ParseError { error, .. } => Some(error),
            _ => None,
        }
    }
}

// Implement From for automatic conversion
impl From<io::Error> for FileProcessError {
    fn from(err: io::Error) -> FileProcessError {
        FileProcessError::IoError(err)
    }
}

// File processor struct
struct FileProcessor {
    path: String,
    min_value: i32,
    max_value: i32,
}

impl FileProcessor {
    fn new(path: String, min_value: i32, max_value: i32) -> Self {
        FileProcessor {
            path,
            min_value,
            max_value,
        }
    }
    
    // Process the file and return a vector of valid numbers
    fn process_file(&self) -> Result<Vec<i32>, FileProcessError> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut numbers = Vec::new();
        let mut line_number = 0;
        
        for line in reader.lines() {
            line_number += 1;
            let line = line?;
            
            if line.trim().is_empty() {
                continue;
            }
            
            // Parse the number
            let number = line
                .trim()
                .parse()
                .map_err(|e| FileProcessError::ParseError {
                    line: line_number,
                    error: e,
                })?;
            
            // Validate the number
            self.validate_number(number, line_number)?;
            numbers.push(number);
        }
        
        if numbers.is_empty() {
            Err(FileProcessError::EmptyFileError)
        } else {
            Ok(numbers)
        }
    }
    
    // Validate a single number
    fn validate_number(&self, number: i32, line: usize) -> Result<(), FileProcessError> {
        if number < self.min_value {
            return Err(FileProcessError::ValidationError {
                line,
                message: format!("Number {} is below minimum {}", number, self.min_value),
            });
        }
        if number > self.max_value {
            return Err(FileProcessError::ValidationError {
                line,
                message: format!("Number {} is above maximum {}", number, self.max_value),
            });
        }
        Ok(())
    }
    
    // Process file and calculate statistics
    fn process_and_analyze(&self) -> Result<FileStats, FileProcessError> {
        let numbers = self.process_file()?;
        let sum: i32 = numbers.iter().sum();
        let avg = sum as f64 / numbers.len() as f64;
        
        Ok(FileStats {
            count: numbers.len(),
            sum,
            average: avg,
            min: *numbers.iter().min().unwrap(),
            max: *numbers.iter().max().unwrap(),
        })
    }
}

// Statistics structure
#[derive(Debug)]
struct FileStats {
    count: usize,
    sum: i32,
    average: f64,
    min: i32,
    max: i32,
}

fn main() {
    // Example usage
    let process_file = |path: &str| {
        println!("Processing file: {}", path);
        let processor = FileProcessor::new(path.to_string(), 0, 100);
        
        match processor.process_and_analyze() {
            Ok(stats) => {
                println!("File processed successfully!");
                println!("Statistics:");
                println!("  Count: {}", stats.count);
                println!("  Sum: {}", stats.sum);
                println!("  Average: {:.2}", stats.average);
                println!("  Min: {}", stats.min);
                println!("  Max: {}", stats.max);
            }
            Err(e) => {
                println!("Error processing file: {}", e);
                if let Some(source) = e.source() {
                    println!("Caused by: {}", source);
                }
            }
        }
        println!();
    };
    
    // Process different files
    process_file("valid_numbers.txt");
    process_file("invalid_numbers.txt");
    process_file("empty_file.txt");
    process_file("nonexistent_file.txt");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::NamedTempFile;
    
    fn create_temp_file(contents: &str) -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), contents).unwrap();
        file
    }
    
    #[test]
    fn test_valid_file() {
        let file = create_temp_file("10\n20\n30\n40\n50\n");
        let processor = FileProcessor::new(
            file.path().to_str().unwrap().to_string(),
            0,
            100,
        );
        
        let result = processor.process_file().unwrap();
        assert_eq!(result, vec![10, 20, 30, 40, 50]);
    }
    
    #[test]
    fn test_invalid_number() {
        let file = create_temp_file("10\n20\n300\n40\n50\n");
        let processor = FileProcessor::new(
            file.path().to_str().unwrap().to_string(),
            0,
            100,
        );
        
        match processor.process_file() {
            Err(FileProcessError::ValidationError { line, .. }) => {
                assert_eq!(line, 3);
            }
            _ => panic!("Expected validation error"),
        }
    }
    
    #[test]
    fn test_parse_error() {
        let file = create_temp_file("10\n20\nnotanumber\n40\n50\n");
        let processor = FileProcessor::new(
            file.path().to_str().unwrap().to_string(),
            0,
            100,
        );
        
        match processor.process_file() {
            Err(FileProcessError::ParseError { line, .. }) => {
                assert_eq!(line, 3);
            }
            _ => panic!("Expected parse error"),
        }
    }
    
    #[test]
    fn test_empty_file() {
        let file = create_temp_file("");
        let processor = FileProcessor::new(
            file.path().to_str().unwrap().to_string(),
            0,
            100,
        );
        
        match processor.process_file() {
            Err(FileProcessError::EmptyFileError) => (),
            _ => panic!("Expected empty file error"),
        }
    }
    
    #[test]
    fn test_statistics() {
        let file = create_temp_file("10\n20\n30\n40\n50\n");
        let processor = FileProcessor::new(
            file.path().to_str().unwrap().to_string(),
            0,
            100,
        );
        
        let stats = processor.process_and_analyze().unwrap();
        assert_eq!(stats.count, 5);
        assert_eq!(stats.sum, 150);
        assert_eq!(stats.average, 30.0);
        assert_eq!(stats.min, 10);
        assert_eq!(stats.max, 50);
    }
} 