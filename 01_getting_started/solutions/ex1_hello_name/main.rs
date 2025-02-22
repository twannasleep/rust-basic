use std::io::{self, Write};

fn main() -> io::Result<()> {
    print!("Please enter your name: ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    
    // Trim whitespace and newlines
    let name = name.trim();
    
    if name.is_empty() {
        println!("Error: Name cannot be empty!");
        return Ok(());
    }
    
    println!("Hello, {}! Welcome to Rust programming!", name);
    Ok(())
} 