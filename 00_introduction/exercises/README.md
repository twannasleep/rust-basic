# Introduction to Rust

This directory contains introductory exercises, examples, and solutions to help you get started with Rust programming.

## Directory Structure

- `/examples`: Contains basic examples demonstrating Rust fundamentals
  - `01_hello_world.rs`: Basic Hello World program
  - `02_variables.rs`: Variables and basic types demonstration

- `/solutions`: Contains solutions to common programming problems
  - `01_fibonacci.rs`: Implementation of Fibonacci sequence
  - `02_temperature_converter.rs`: Temperature conversion program

## Exercises

### Exercise 1: Hello World

Create a new Rust project that prints "Hello, World!" with the following requirements:

```rust
// src/main.rs
fn main() {
    // TODO: Implement your hello world program
}
```

Requirements:

- Use proper project structure
- Add comments
- Include a README.md
- Format code with rustfmt

### Exercise 2: Basic Calculator

Create a simple calculator that:

- Takes two numbers as command line arguments
- Performs basic arithmetic
- Handles errors appropriately

```rust
// src/main.rs
use std::env;

fn main() {
    // TODO: Implement calculator logic
}
```

### Exercise 3: Environment Setup

Complete the following tasks:

1. Install Rust and Cargo
2. Configure your IDE
3. Set up Git
4. Create a new project
5. Run tests and format code

### Exercise 4: Cargo Practice

Practice using Cargo:

1. Create a new library
2. Add dependencies
3. Run tests
4. Generate documentation
5. Build in release mode

## How to Run

To run any of the examples or solutions, use the Rust compiler (`rustc`) or Cargo:

Using `rustc`:

```bash
# Compile the file
rustc examples/01_hello_world.rs
# Run the compiled program
./01_hello_world
```

Each file is self-contained and can be compiled and run independently. The examples progress from basic concepts to more complex implementations.

## Learning Path

1. Complete the environment setup (Exercise 3)
2. Start with the basic examples in numerical order
3. Work through the exercises in order
4. Study the solutions to understand different approaches
5. Practice with Cargo (Exercise 4)

Feel free to modify and experiment with the code to better understand how Rust works!
