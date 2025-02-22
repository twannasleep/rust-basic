use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use thiserror::Error;

// Custom error type using thiserror
#[derive(Error, Debug)]
pub enum FileProcessError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Invalid file content: {0}")]
    InvalidContent(String),
    
    #[error("Empty file")]
    EmptyFile,
    
    #[error("Number parsing error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    
    #[error("Value out of range: {0}")]
    ValueOutOfRange(i32),
}

// Type alias for Result with our custom error type
type Result<T> = std::result::Result<T, FileProcessError>;

// Struct to hold processed data
#[derive(Debug)]
struct ProcessedData {
    numbers: Vec<i32>,
    sum: i32,
    average: f64,
}

// Function to read file contents
fn read_file(path: &Path) -> Result<String> {
    // Using ? operator for error propagation
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.trim().is_empty() {
        return Err(FileProcessError::EmptyFile);
    }

    Ok(content)
}

// Function to parse numbers from content
fn parse_numbers(content: &str) -> Result<Vec<i32>> {
    let numbers: Result<Vec<i32>> = content
        .lines()
        .map(|line| {
            // Parse the number
            let num = line.trim().parse()?;
            
            // Validate range
            if num < -1000 || num > 1000 {
                return Err(FileProcessError::ValueOutOfRange(num));
            }
            
            Ok(num)
        })
        .collect();

    let numbers = numbers?;
    
    if numbers.is_empty() {
        return Err(FileProcessError::InvalidContent("No valid numbers found".to_string()));
    }

    Ok(numbers)
}

// Function to process numbers
fn process_numbers(numbers: &[i32]) -> ProcessedData {
    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / numbers.len() as f64;

    ProcessedData {
        numbers: numbers.to_vec(),
        sum,
        average,
    }
}

// Function to save results
fn save_results(path: &Path, data: &ProcessedData) -> Result<()> {
    let mut file = File::create(path)?;
    
    writeln!(file, "Numbers: {:?}", data.numbers)?;
    writeln!(file, "Sum: {}", data.sum)?;
    writeln!(file, "Average: {:.2}", data.average)?;
    
    Ok(())
}

// Function that demonstrates different ways to handle errors
fn process_file(input_path: &Path, output_path: &Path) -> Result<()> {
    println!("Processing file: {:?}", input_path);

    // Read and process the file
    let content = read_file(input_path)
        .map_err(|e| {
            eprintln!("Error reading file: {}", e);
            e
        })?;

    // Parse numbers with custom error handling
    let numbers = match parse_numbers(&content) {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("Error parsing numbers: {}", e);
            return Err(e);
        }
    };

    // Process the numbers (this operation cannot fail)
    let processed_data = process_numbers(&numbers);

    // Save results, using ? for error propagation
    save_results(output_path, &processed_data)?;

    println!("Processing completed successfully!");
    println!("Results saved to: {:?}", output_path);
    
    Ok(())
}

fn main() {
    println!("Error Handling Demo");
    println!("-----------------\n");

    // Create a temporary directory for our files
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    
    // Test with various scenarios
    let scenarios = vec![
        ("valid.txt", "1\n2\n3\n4\n5"),
        ("empty.txt", ""),
        ("invalid.txt", "1\n2\nnotanumber\n4\n5"),
        ("out_of_range.txt", "1\n2000\n3\n4\n5"),
    ];

    for (filename, content) in scenarios {
        println!("\nTesting with {}", filename);
        println!("Content: {}", content);
        
        // Create input file
        let input_path = temp_dir.path().join(filename);
        fs::write(&input_path, content).expect("Failed to write test file");
        
        // Process file
        let output_path = temp_dir.path().join(format!("output_{}", filename));
        match process_file(&input_path, &output_path) {
            Ok(()) => {
                // Try to read and display the results
                if let Ok(results) = fs::read_to_string(&output_path) {
                    println!("Results:\n{}", results);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        
        println!("----------------------------------------");
    }
}
