// Example 2: Variables and Basic Types
// This example shows how to declare variables and use basic types in Rust

fn main() {
    // Variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Use 'mut' to make a variable mutable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);
    
    // Basic types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';
    
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
} 