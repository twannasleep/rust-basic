# Fundamentals Exercises

## Exercise 1: Temperature Converter

Create a program that converts temperatures between Fahrenheit and Celsius. Your program should:

1. Define functions for both conversion directions
2. Handle both integer and floating-point temperatures
3. Use proper type annotations
4. Include error handling for invalid inputs

```rust
fn fahrenheit_to_celsius(f: f64) -> f64 {
    // TODO: Implement conversion
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    // TODO: Implement conversion
}
```

## Exercise 2: Array Statistics

Create a program that calculates statistics for an array of numbers:

1. Find the minimum and maximum values
2. Calculate the average (mean)
3. Calculate the median
4. Count occurrences of each number

Requirements:

- Use appropriate data types
- Implement error handling for empty arrays
- Use functions to organize your code
- Use loops and control flow appropriately

## Exercise 3: String Manipulation

Create a program that processes strings with the following features:

1. Count words in a sentence
2. Reverse each word while maintaining word order
3. Convert to title case (capitalize first letter of each word)
4. Remove duplicate words

Example:

```rust
fn word_count(text: &str) -> usize {
    // TODO: Implement word counting
}

fn reverse_words(text: &str) -> String {
    // TODO: Implement word reversal
}
```

## Exercise 4: Pattern Matching

Create a program that demonstrates pattern matching with:

1. Match expressions for different types
2. Guards in match arms
3. Multiple patterns in a single arm
4. Default cases

Example structure:

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    // TODO: Implement area calculation using pattern matching
}
```

## Bonus Challenge: Mini Calculator

Create a command-line calculator that:

1. Accepts two numbers and an operator as arguments
2. Supports addition, subtraction, multiplication, and division
3. Handles decimal numbers
4. Provides proper error messages for:
   - Invalid numbers
   - Division by zero
   - Invalid operators
   - Missing arguments

Example usage:

```bash
$ cargo run -- 5.2 + 3.8
9.0
$ cargo run -- 10 / 0
Error: Division by zero
```

## How to Submit

For each exercise:

1. Create a new file with your implementation
2. Include comments explaining your code
3. Add error handling where appropriate
4. Format your code using `rustfmt`
5. Test your code with different inputs

## Evaluation Criteria

Your solutions will be evaluated based on:

1. Correctness of implementation
2. Proper use of Rust features
3. Code organization and clarity
4. Error handling
5. Code style and formatting
