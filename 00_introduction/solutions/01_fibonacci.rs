// Solution 1: Fibonacci Sequence
// This program calculates the nth Fibonacci number

fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut a = 0;
    let mut b = 1;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

fn main() {
    // Test the fibonacci function
    for n in 0..10 {
        println!("Fibonacci({}) = {}", n, fibonacci(n));
    }
    
    // Example output:
    // Fibonacci(0) = 0
    // Fibonacci(1) = 1
    // Fibonacci(2) = 1
    // Fibonacci(3) = 2
    // Fibonacci(4) = 3
    // Fibonacci(5) = 5
    // Fibonacci(6) = 8
    // Fibonacci(7) = 13
    // Fibonacci(8) = 21
    // Fibonacci(9) = 34
} 