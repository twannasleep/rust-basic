# Getting Started with Rust

## Introduction to Rust

Rust is a systems programming language that focuses on safety, concurrency, and performance. It was developed by Mozilla Research and has gained popularity for its unique features:

- Memory safety without garbage collection
- Zero-cost abstractions
- Guaranteed thread safety
- Package management with Cargo
- Modern tooling and excellent documentation

## Setting Up Your Development Environment

1. **Install Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   This installs:
   - rustc (the compiler)
   - cargo (the package manager)
   - rustup (the toolchain manager)

2. **Verify Installation**

   ```bash
   rustc --version
   cargo --version
   ```

3. **IDE Setup**
   Recommended IDEs/editors:
   - VS Code with rust-analyzer extension
   - IntelliJ IDEA with Rust plugin
   - Sublime Text with Rust Enhanced

## Hello World Program

Let's create your first Rust program. Create a file named `hello_world.rs`:

```rust
fn main() {
    println!("Hello, World!");
}
```

To compile and run:

```bash
rustc hello_world.rs
./hello_world
```

### Understanding the Code

- `fn main()` is the entry point of the program
- `println!` is a macro (note the `!`)
- Statements end with semicolons
- Code blocks are enclosed in curly braces

## Understanding Cargo

Cargo is Rust's package manager and build system. It handles:

- Dependencies
- Building your project
- Running tests
- Generating documentation

### Basic Cargo Commands

```bash
# Create a new project
cargo new project_name

# Build your project
cargo build

# Run your project
cargo run

# Check if your project compiles
cargo check

# Run tests
cargo test
```

### Project Structure

```
my_project/
├── Cargo.toml    # Project configuration
├── Cargo.lock    # Lock file for dependencies
└── src/
    └── main.rs   # Source code
```

### Cargo.toml

This is your project's manifest file:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
# Add external packages here
```

## Exercises

1. Create a new Cargo project called "greeting"
2. Modify the program to ask for the user's name and print a personalized greeting
3. Add a dependency to your project (try using the `colored` crate)
4. Build and run your project using Cargo

## Next Steps

After completing this section, you should:

- Have Rust installed and working
- Understand basic Rust program structure
- Know how to use Cargo for project management
- Be ready to move on to Basic Concepts

Remember to experiment with the code and try modifying the examples to better understand how things work!
