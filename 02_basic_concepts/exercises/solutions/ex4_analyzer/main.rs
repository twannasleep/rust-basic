use std::io::{self, Write};
use std::collections::HashMap;

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn calculate_mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn calculate_median(numbers: &mut [f64]) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[mid]
    }
}

fn calculate_mode(numbers: &[f64]) -> Vec<f64> {
    let mut counts = HashMap::new();
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    let max_count = counts.values().max().unwrap();
    counts
        .into_iter()
        .filter(|&(_, count)| count == *max_count)
        .map(|(num, _)| num)
        .collect()
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    println!("Number Analyzer");
    println!("--------------");

    loop {
        println!("\nEnter numbers separated by spaces (or 'q' to quit):");
        let input = get_user_input("");

        if input.to_lowercase() == "q" {
            println!("Goodbye!");
            break;
        }

        let numbers: Vec<f64> = match input
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect()
        {
            Ok(nums) if !nums.is_empty() => nums,
            _ => {
                println!("Please enter valid numbers separated by spaces!");
                continue;
            }
        };

        let mut numbers_clone = numbers.clone();
        
        // Basic statistics
        println!("\nStatistics:");
        println!("----------");
        println!("Count: {}", numbers.len());
        println!("Mean: {:.2}", calculate_mean(&numbers));
        println!("Median: {:.2}", calculate_median(&mut numbers_clone));
        println!("Mode: {:?}", calculate_mode(&numbers));
        println!("Minimum: {:.2}", numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b)));
        println!("Maximum: {:.2}", numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)));

        // Prime number analysis
        println!("\nPrime Number Analysis:");
        println!("--------------------");
        for &num in &numbers {
            if num.fract() == 0.0 && num >= 0.0 && num <= i64::MAX as f64 {
                let n = num as i64;
                println!("{} is {}", n, if is_prime(n) { "prime" } else { "not prime" });
            } else {
                println!("{} is not a valid number for prime checking", num);
            }
        }
    }
} 