use std::io::{self, Write};
use colored::*;

fn main() {
    // Print a welcome message
    println!("{}", "Welcome to the Rust Tutorial!".green().bold());
    
    // Get user input
    print!("Please enter your name: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input
    
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    // Remove trailing newline
    let name = name.trim();
    
    // Print personalized greeting
    println!("\nHello, {}! Let's learn Rust together!", name.blue().bold());
    
    // Show some basic Rust features
    println!("\n{}", "Some Rust Features:".yellow().bold());
    println!("1. {}", "Type Safety".cyan());
    println!("2. {}", "Memory Safety".cyan());
    println!("3. {}", "Zero-cost Abstractions".cyan());
    println!("4. {}", "Pattern Matching".cyan());
    println!("5. {}", "Modern Package Management".cyan());
    
    // Demonstrate string formatting
    println!("\n{}", "Example of string formatting:".yellow().bold());
    let language = "Rust";
    let year = 2010;
    println!("{} was first released in {}!", language, year);
} 