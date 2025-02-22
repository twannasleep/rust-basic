// Example: Data Types and Operations
// This example demonstrates various data types and common operations

fn main() {
    // Numeric operations
    let x = 5;
    let y = 10;
    
    println!("Basic arithmetic:");
    println!("Addition: {} + {} = {}", x, y, x + y);
    println!("Subtraction: {} - {} = {}", y, x, y - x);
    println!("Multiplication: {} * {} = {}", x, y, x * y);
    println!("Division: {} / {} = {}", y, x, y / x);
    
    // Floating point operations
    let pi: f64 = 3.14159;
    let radius = 5.0;
    let area = pi * radius * radius;
    println!("\nCircle calculations:");
    println!("Area of circle with radius {}: {:.2}", radius, area);
    
    // String and char operations
    let first_name = "Rust";
    let last_name = "Developer";
    let full_name = format!("{} {}", first_name, last_name);
    println!("\nString operations:");
    println!("Full name: {}", full_name);
    println!("Name length: {}", full_name.len());
    println!("Is empty? {}", full_name.is_empty());
    
    // Array operations
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;
    println!("\nArray operations:");
    print!("Numbers: ");
    for num in numbers.iter() {
        print!("{} ", num);
        sum += num;
    }
    println!("\nSum: {}", sum);
    println!("Array length: {}", numbers.len());
    
    // Tuple operations
    let tuple = (42, "hello", 3.14);
    println!("\nTuple operations:");
    println!("First value: {}", tuple.0);
    println!("Second value: {}", tuple.1);
    println!("Third value: {}", tuple.2);
    
    // Type conversion
    let integer = 65;
    let character = integer as char;
    let float = integer as f64;
    println!("\nType conversion:");
    println!("Integer {} as character: {}", integer, character);
    println!("Integer {} as float: {}", integer, float);
} 