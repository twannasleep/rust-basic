use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use rand::Rng;

// Custom error type
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    InvalidInput(String),
    NotFound,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

// Vector examples
fn demonstrate_vectors() {
    println!("Vector Examples:");
    println!("--------------");

    // Creating and populating a vector
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Vector with initial values
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("Initial vector: {:?}", vec);

    // Accessing elements
    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Iterating over vector
    println!("Doubled values:");
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    // Filtering vector
    let even: Vec<i32> = vec.iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()
        .collect();
    println!("Even numbers: {:?}", even);
}

// String examples
fn demonstrate_strings() {
    println!("\nString Examples:");
    println!("--------------");

    // Creating strings
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // note s1 has been moved here
    println!("{}", s3);

    // String manipulation
    let text = String::from("  Hello World  ");
    println!("Trimmed: {}", text.trim());
    println!("Uppercase: {}", text.to_uppercase());
    
    // Splitting strings
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("Words: {:?}", words);
}

// HashMap examples
fn demonstrate_hashmaps() {
    println!("\nHashMap Examples:");
    println!("---------------");

    // Creating a new HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values
    match scores.get("Blue") {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team doesn't have a score"),
    }

    // Word frequency counter
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word frequencies: {:?}", word_count);
}

// Error handling examples
fn demonstrate_error_handling() -> Result<(), AppError> {
    println!("\nError Handling Examples:");
    println!("---------------------");

    // Working with Result
    let username_result = get_username(-1);
    match username_result {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error getting username: {:?}", e),
    }

    // Working with Option
    let user = find_user(1);
    if let Some(name) = user {
        println!("Found user: {}", name);
    }

    // File operations with error handling
    match write_and_read_file() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("File operation failed: {:?}", e),
    }

    Ok(())
}

// Helper functions for error handling demonstration
fn get_username(id: i32) -> Result<String, AppError> {
    if id < 1 {
        return Err(AppError::InvalidInput(String::from("ID must be positive")));
    }
    Ok(String::from("alice"))
}

fn find_user(id: i32) -> Option<String> {
    if id > 0 {
        Some(String::from("bob"))
    } else {
        None
    }
}

fn write_and_read_file() -> Result<String, AppError> {
    // Generate a random number to append to filename to avoid conflicts
    let random_num = rand::thread_rng().gen_range(1000..9999);
    let filename = format!("test_{}.txt", random_num);

    // Write to file
    let mut file = File::create(&filename)?;
    file.write_all(b"Hello, Rust!")?;

    // Read from file
    let mut file = File::open(&filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Clean up - delete the file
    std::fs::remove_file(&filename)?;

    Ok(contents)
}

fn main() {
    demonstrate_vectors();
    demonstrate_strings();
    demonstrate_hashmaps();
    if let Err(e) = demonstrate_error_handling() {
        println!("Error in demonstration: {:?}", e);
    }
} 