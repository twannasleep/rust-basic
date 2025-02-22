# Modules and Crates in Rust

## Module System Basics

```rust
// Basic module structure
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// Using modules
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

## Visibility and Privacy

```rust
mod back_of_house {
    // Public struct with some private fields
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,  // Private field
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    // Public enum - all variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```

## Module Organization

```rust
// lib.rs or main.rs
mod front_of_house;  // Loads from front_of_house.rs or front_of_house/mod.rs

// Using the use keyword
use crate::front_of_house::hosting;
use std::collections::HashMap;
use std::io::{self, Write};

// Renaming with 'as'
use std::io::Error as IoError;

// Re-exporting
pub use crate::front_of_house::hosting;
```

## Creating a Library Crate

```rust
// lib.rs
pub mod client;
pub mod network;

pub fn connect() {
    // Library code here
}

// Nested modules
pub mod client {
    pub fn connect() {
        super::connect();
    }
}
```

## External Dependencies

```toml
# Cargo.toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
```

## Workspaces

```toml
# Cargo.toml in workspace root
[workspace]
members = [
    "adder",
    "add-one",
    "add-two",
]
```

## Documentation

```rust
/// Adds two numbers together
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

## Best Practices

```rust
// Organizing large modules
pub mod models;     // Database models
pub mod handlers;   // Request handlers
pub mod services;   // Business logic
pub mod utils;      // Utility functions

// Re-exports for convenient access
pub use models::{User, Post};
pub use handlers::*;

// Internal implementation details
mod internal {
    pub(crate) fn helper() {
        // Visible only within the crate
    }
}
```

## Testing Modules

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_external_api() {
    assert!(my_crate::public_api());
}

// Unit tests within modules
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_api() {
        assert!(internal_function());
    }
}
```
