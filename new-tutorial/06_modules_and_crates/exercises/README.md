# Modules and Crates Exercises

## 🌟 Exercise 1: Library Organization

Create a library crate for a task management system:

```rust
// TODO: Implement the following module structure
// src/
//   lib.rs
//   tasks/
//     mod.rs
//     task.rs
//     status.rs
//   users/
//     mod.rs
//     user.rs
//     permissions.rs
//   utils/
//     mod.rs
//     date.rs
//     id_generator.rs

// Example module implementation
pub mod tasks {
    mod task;
    mod status;
    
    pub use task::Task;
    pub use status::Status;
    
    // Add necessary types and functions
}
```

**Skills practiced:**

- Module organization
- Visibility rules
- Re-exports
- Documentation

## 🌟🌟 Exercise 2: Public API Design

Design a public API for a math utilities library:

```rust
// TODO: Create a well-organized math library
pub mod arithmetic {
    // Basic operations
}

pub mod statistics {
    // Statistical functions
}

pub mod geometry {
    // Geometric calculations
}

// Include:
// - Public interfaces
// - Private implementation details
// - Proper documentation
// - Unit tests
```

**Skills practiced:**

- API design
- Documentation
- Testing
- Module privacy

## 🌟🌟 Exercise 3: Workspace Management

Create a workspace with multiple related crates:

```toml
# TODO: Create a workspace with:
# - Core library crate
# - CLI application
# - Web API
# - Shared utilities

[workspace]
members = [
    "core",
    "cli",
    "web",
    "utils"
]
```

**Skills practiced:**

- Workspace configuration
- Inter-crate dependencies
- Shared code
- Package management

## 🌟🌟🌟 Exercise 4: Plugin System

Implement a plugin system using modules:

```rust
// TODO: Create a plugin system where:
// - Plugins are separate modules
// - Plugins implement a common trait
// - Main application loads plugins dynamically

pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}

// Implement plugin loading and management
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}
```

**Skills practiced:**

- Dynamic loading
- Trait objects
- Error handling
- Module interfaces

## 🌟🌟🌟 Exercise 5: Feature Flags

Implement conditional compilation with features:

```toml
# Cargo.toml
[features]
default = ["std"]
std = []
async = ["tokio"]
logging = ["log"]
```

```rust
// TODO: Implement features that:
// - Provide different implementations based on features
// - Handle optional dependencies
// - Include conditional tests

#[cfg(feature = "async")]
pub async fn process() {
    // Async implementation
}

#[cfg(not(feature = "async"))]
pub fn process() {
    // Synchronous implementation
}
```

**Skills practiced:**

- Feature flags
- Conditional compilation
- Optional dependencies
- Cross-platform code

## Tips

1. Use meaningful module names
2. Keep public APIs minimal
3. Document all public items
4. Follow the Rust API guidelines
5. Write integration tests

## Evaluation Criteria

- Code organization
- Documentation quality
- API usability
- Test coverage
- Feature implementation
