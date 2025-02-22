use std::io::{self, Write};
use std::time::Instant;

fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn fibonacci_iterative(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Fibonacci Calculator");
    println!("-------------------");

    loop {
        let input = get_user_input("Enter a number (0-40) or 'q' to quit: ");
        
        if input.to_lowercase() == "q" {
            println!("Goodbye!");
            break;
        }

        let n: u32 = match input.parse() {
            Ok(num) if num <= 40 => num,
            Ok(_) => {
                println!("Please enter a number between 0 and 40!");
                continue;
            }
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Measure recursive implementation
        let start = Instant::now();
        let recursive_result = fibonacci_recursive(n);
        let recursive_duration = start.elapsed();

        // Measure iterative implementation
        let start = Instant::now();
        let iterative_result = fibonacci_iterative(n);
        let iterative_duration = start.elapsed();

        println!("\nResults for n = {}:", n);
        println!("Recursive: {} (took {:?})", recursive_result, recursive_duration);
        println!("Iterative: {} (took {:?})", iterative_result, iterative_duration);
        
        if recursive_duration > iterative_duration {
            println!("Iterative solution was {:?} faster!", recursive_duration - iterative_duration);
        } else {
            println!("Recursive solution was {:?} faster!", iterative_duration - recursive_duration);
        }
        println!();
    }
} 