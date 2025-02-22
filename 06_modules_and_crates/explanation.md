# 📦 Modules and Crates in Rust

## 📋 Overview

Rust's module system helps you organize code, control privacy, and create clear paths for using items.

## 🏗️ Module System Basics

### Module Structure

```rust
// Basic module structure
mod front_of_house {
    // Public module
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        // Private function
        fn seat_at_table() {}
    }
    
    // Private module
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

### File Organization

```plaintext
restaurant/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── front_of_house/
    │   ├── mod.rs
    │   ├── hosting.rs
    │   └── serving.rs
    └── back_of_house/
        ├── mod.rs
        ├── kitchen.rs
        └── storage.rs
```

## 🔒 Visibility and Privacy

### Access Control

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
    
    // Restricted visibility
    pub(crate) fn cook_order() {
        // Visible only within the current crate
    }
    
    pub(super) fn clean_table() {
        // Visible only in the parent module
    }
}
```

## 📝 Module Organization

### Using the `use` Keyword

```rust
// Basic imports
use std::collections::HashMap;
use std::io::{self, Write};
use std::fmt::Result;
use std::io::Result as IoResult;

// Re-exporting
pub use crate::front_of_house::hosting;

// Nested paths
use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

// Glob imports (use sparingly)
use std::collections::*;
```

### Module Hierarchy

```rust
// lib.rs
pub mod front_of_house;
pub mod back_of_house;

// front_of_house/mod.rs
pub mod hosting;
pub mod serving;

// front_of_house/hosting.rs
pub fn add_to_waitlist() {}
pub fn seat_at_table() {}

// back_of_house/mod.rs
pub mod kitchen;
pub mod storage;
```

## 📦 Creating a Library Crate

### Library Structure

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
    
    pub mod types {
        #[derive(Debug)]
        pub struct Connection {
            pub id: u32,
            pub active: bool,
        }
    }
}
```

### Documentation

```rust
//! # My Library
//! 
//! `my_library` provides functionality for doing amazing things.

/// Adds two numbers together.
/// 
/// # Examples
/// 
/// ```
/// let result = my_library::add(2, 2);
/// assert_eq!(result, 4);
/// ```
/// 
/// # Panics
/// 
/// This function never panics.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 🔧 External Dependencies

### Cargo.toml Configuration

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A fantastic Rust project"
license = "MIT"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
env_logger = "0.9"

[dev-dependencies]
criterion = "0.3"
mockall = "0.11"

[build-dependencies]
cc = "1.0"
```

### Feature Flags

```toml
[features]
default = ["std"]
std = []
async = ["tokio"]
full = ["async", "extra"]

[dependencies.serde]
version = "1.0"
optional = true
```

## 🏭 Workspaces

### Workspace Structure

```toml
# Cargo.toml in workspace root
[workspace]
members = [
    "adder",
    "add-one",
    "add-two",
]

[workspace.dependencies]
serde = "1.0"
log = "0.4"
```

### Project Organization

```plaintext
workspace/
├── Cargo.toml
├── Cargo.lock
├── adder/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── add-one/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── add-two/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## 🎯 Best Practices

### 1. Module Organization

```rust
// Organize large modules by functionality
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

### 2. API Design

```rust
// Public API module
pub mod api {
    // Public interface
    pub use crate::internal::types::*;
    pub use crate::internal::functions::*;
    
    // Hide implementation details
    #[doc(hidden)]
    pub mod __private {
        pub use crate::internal::helpers::*;
    }
}

// Private implementation
mod internal {
    pub(crate) mod types {}
    pub(crate) mod functions {}
    pub(crate) mod helpers {}
}
```

### 3. Testing Organization

```rust
// Unit tests within modules
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_api() {
        assert!(internal_function());
    }
}

// Integration tests
// tests/integration_test.rs
use my_crate;

#[test]
fn test_external_api() {
    assert!(my_crate::public_api());
}
```

## 🔍 Complete Example

```rust
// lib.rs
pub mod auth {
    use std::collections::HashMap;
    
    #[derive(Debug)]
    pub struct User {
        pub username: String,
        password_hash: String,
        pub role: Role,
    }
    
    #[derive(Debug)]
    pub enum Role {
        Admin,
        Regular,
        Guest,
    }
    
    impl User {
        pub fn new(username: &str, password: &str) -> Self {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
                role: Role::Regular,
            }
        }
        
        pub fn verify_password(&self, password: &str) -> bool {
            self.password_hash == hash_password(password)
        }
    }
    
    fn hash_password(password: &str) -> String {
        // Simple hash for demonstration
        format!("hashed_{}", password)
    }
    
    pub mod session {
        use super::User;
        use std::time::{SystemTime, UNIX_EPOCH};
        
        pub struct Session {
            pub user: User,
            pub token: String,
            pub expires: u64,
        }
        
        impl Session {
            pub fn new(user: User) -> Self {
                Session {
                    user,
                    token: generate_token(),
                    expires: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() + 3600,
                }
            }
        }
        
        fn generate_token() -> String {
            // Simple token generation for demonstration
            format!("token_{}", rand::random::<u64>())
        }
    }
}

// main.rs
use my_lib::auth::{User, Role, session::Session};

fn main() {
    let user = User::new("alice", "password123");
    let session = Session::new(user);
    
    println!("Created session for user: {}", session.user.username);
    println!("Session token: {}", session.token);
    println!("Expires at: {}", session.expires);
}
```

Remember: Good module organization makes your code more maintainable and easier to understand! 🚀
