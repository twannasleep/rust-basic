// Example: Basic Error Handling
// This example demonstrates fundamental error handling patterns in Rust

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// Basic Result with custom error type
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(x / y)
    }
}

// Option example with pattern matching
fn find_character(c: char, text: &str) -> Option<usize> {
    text.find(c)
}

// Using the ? operator with built-in Result
fn read_file_contents(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Combining Option and Result
fn parse_first_number(text: &str) -> Option<Result<i32, std::num::ParseIntError>> {
    text.split_whitespace()
        .next()
        .map(|word| word.parse::<i32>())
}

// Multiple error handling with match
fn process_number(text: &str) -> Result<i32, String> {
    let parsed = text.parse::<i32>().map_err(|e| e.to_string())?;
    
    if parsed < 0 {
        Err("Number cannot be negative".to_string())
    } else if parsed > 100 {
        Err("Number cannot be greater than 100".to_string())
    } else {
        Ok(parsed * 2)
    }
}

fn main() {
    // Handling Result with match
    println!("Division examples:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Handling Option with if let
    println!("\nCharacter finding:");
    let text = "Hello, World!";
    if let Some(pos) = find_character('W', text) {
        println!("Found 'W' at position {}", pos);
    } else {
        println!("Character not found");
    }
    
    // Using unwrap_or and expect
    println!("\nDefault values:");
    let safe_divide = divide(10.0, 2.0).unwrap_or(0.0);
    println!("Safe division result: {}", safe_divide);
    
    // Handling file operations
    println!("\nFile operations:");
    let path = Path::new("nonexistent.txt");
    match read_file_contents(path) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
    
    // Combining Option and Result
    println!("\nParsing numbers:");
    let text = "42 other words";
    match parse_first_number(text) {
        Some(Ok(number)) => println!("Successfully parsed: {}", number),
        Some(Err(e)) => println!("Parse error: {}", e),
        None => println!("No number found"),
    }
    
    // Processing with validation
    println!("\nNumber processing:");
    let numbers = ["15", "-5", "150", "not a number"];
    for &num in numbers.iter() {
        match process_number(num) {
            Ok(result) => println!("Processed {} -> {}", num, result),
            Err(e) => println!("Error processing {}: {}", num, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(divide(10.0, 0.0).is_err());
    }
    
    #[test]
    fn test_find_character() {
        assert_eq!(find_character('H', "Hello"), Some(0));
        assert_eq!(find_character('z', "Hello"), None);
    }
    
    #[test]
    fn test_process_number() {
        assert_eq!(process_number("50"), Ok(100));
        assert!(process_number("-5").is_err());
        assert!(process_number("150").is_err());
        assert!(process_number("abc").is_err());
    }
} 