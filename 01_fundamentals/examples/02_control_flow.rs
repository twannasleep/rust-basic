// Example: Control Flow Structures
// This example demonstrates different control flow patterns in Rust

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // If-else with pattern matching
    let number = 7;
    println!("Number analysis:");
    if number < 5 {
        println!("{} is less than 5", number);
    } else if number > 5 {
        println!("{} is greater than 5", number);
    } else {
        println!("{} is equal to 5", number);
    }
    
    // Match expression
    let day_number = 3;
    println!("\nMatch expression example:");
    let day = match day_number {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 | 7 => "Weekend",
        _ => "Invalid day",
    };
    println!("Day {}: {}", day_number, day);
    
    // Loop with break and continue
    println!("\nLoop with break and continue:");
    let mut counter = 0;
    loop {
        counter += 1;
        
        if counter == 3 {
            println!("Skipping 3...");
            continue;
        }
        
        println!("Counter: {}", counter);
        
        if counter >= 5 {
            println!("Breaking the loop!");
            break;
        }
    }
    
    // While loop with condition
    println!("\nWhile loop example:");
    let mut n = 1;
    while n <= 5 {
        if is_even(n) {
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
        n += 1;
    }
    
    // For loop with range
    println!("\nFor loop with range:");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!(); // New line
    
    // For loop with iterator
    println!("\nFor loop with iterator:");
    let animals = vec!["cat", "dog", "bird"];
    for (index, animal) in animals.iter().enumerate() {
        println!("Animal {}: {}", index + 1, animal);
    }
} 