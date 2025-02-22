use std::io::{self, Write};
use colored::*;

fn main() {
    println!("Please enter your name");
    
    io::stdout().flush().unwrap();

    let mut name= String::new();

    io::stdin().read_line(&mut name).expect("Faild to read name!");
    
    let name = name.trim();
    
    if name.is_empty() {
        println!("{}", "Error: Name is empty".red());
        return;
    }

    println!("Hello, Welcome {} to the Rust Tutorial!", name.green().bold());
} 