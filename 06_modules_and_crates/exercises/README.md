# Modules and Crates Exercises

## Exercise 1: Library Crate Design

Create a library crate for a math utilities package:

1. Create a library crate with the following structure:

   ```
   math_utils/
   ├── Cargo.toml
   ├── src/
   │   ├── lib.rs
   │   ├── arithmetic.rs
   │   ├── statistics.rs
   │   └── geometry/
   │       ├── mod.rs
   │       ├── shapes.rs
   │       └── transformations.rs
   ```

2. Implement the following modules:
   - `arithmetic`: Basic arithmetic operations
   - `statistics`: Mean, median, mode, standard deviation
   - `geometry::shapes`: Area and perimeter calculations
   - `geometry::transformations`: Rotation, scaling, translation

3. Requirements:
   - Use proper visibility rules
   - Implement comprehensive tests
   - Add documentation with examples
   - Create a useful public API
   - Handle errors appropriately

Example structure:

```rust
// lib.rs
pub mod arithmetic;
pub mod statistics;
pub mod geometry;

// arithmetic.rs
pub fn gcd(a: u64, b: u64) -> u64 {
    // Implementation
}

pub fn lcm(a: u64, b: u64) -> u64 {
    // Implementation
}

// statistics.rs
pub fn mean(numbers: &[f64]) -> Option<f64> {
    // Implementation
}

// geometry/shapes.rs
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        // Implementation
    }
}
```

## Exercise 2: Application Architecture

Create a command-line todo application with the following module structure:

1. Project structure:

   ```
   todo_app/
   ├── Cargo.toml
   ├── src/
   │   ├── main.rs
   │   ├── commands/
   │   │   ├── mod.rs
   │   │   ├── add.rs
   │   │   ├── list.rs
   │   │   └── remove.rs
   │   ├── storage/
   │   │   ├── mod.rs
   │   │   └── file.rs
   │   └── models/
   │       ├── mod.rs
   │       └── todo.rs
   ```

2. Requirements:
   - Implement command handling in separate modules
   - Use a storage module for persistence
   - Create a proper type system for todos
   - Handle errors with custom error types
   - Use external crates for:
     - Command line parsing
     - Serialization
     - Date/time handling

Example usage:

```rust
// main.rs
mod commands;
mod storage;
mod models;

fn main() {
    // Parse commands and execute appropriate handlers
}

// models/todo.rs
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// commands/add.rs
pub fn execute(title: String) -> Result<(), Error> {
    // Implementation
}
```

## Exercise 3: Plugin System

Create a plugin system for a text processing application:

1. Project structure:

   ```
   text_processor/
   ├── Cargo.toml
   ├── src/
   │   ├── lib.rs
   │   ├── plugin.rs
   │   └── processor.rs
   ├── plugins/
   │   ├── uppercase/
   │   │   └── src/lib.rs
   │   └── counter/
   │       └── src/lib.rs
   ```

2. Requirements:
   - Define a plugin trait
   - Create a plugin manager
   - Implement dynamic loading
   - Create example plugins
   - Use workspace organization

Example implementation:

```rust
// lib.rs
pub trait TextPlugin {
    fn name(&self) -> &str;
    fn process(&self, input: &str) -> String;
}

// plugin.rs
pub struct PluginManager {
    plugins: Vec<Box<dyn TextPlugin>>,
}

// plugins/uppercase/src/lib.rs
pub struct UppercasePlugin;

impl TextPlugin for UppercasePlugin {
    // Implementation
}
```

## Exercise 4: Web Service

Create a modular web service with the following features:

1. Project structure:

   ```
   web_service/
   ├── Cargo.toml
   ├── src/
   │   ├── main.rs
   │   ├── api/
   │   │   ├── mod.rs
   │   │   └── handlers.rs
   │   ├── db/
   │   │   ├── mod.rs
   │   │   └── models.rs
   │   └── config/
   │       ├── mod.rs
   │       └── settings.rs
   ```

2. Requirements:
   - Use actix-web or rocket for the web framework
   - Implement proper module organization
   - Create middleware
   - Handle configuration
   - Implement database integration
   - Add logging and error handling

Example structure:
```rust
// api/handlers.rs
pub async fn get_user(id: Path<i32>) -> Result<Json<User>> {
    // Implementation
}

// db/models.rs
pub struct User {
    id: i32,
    name: String,
}

// config/settings.rs
pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
}
```

## Evaluation Criteria

Your solutions will be evaluated based on:

1. Module Organization
   - Proper use of visibility rules
   - Clear module hierarchy
   - Logical grouping of functionality

2. Code Quality
   - Error handling
   - Documentation
   - Testing
   - Type safety

3. External Dependencies
   - Appropriate use of external crates
   - Version management
   - Feature selection

4. Project Structure
   - Workspace organization
   - File layout
   - Build configuration

## Testing

For each exercise:

1. Write unit tests for each module
2. Add integration tests
3. Include documentation tests
4. Test error conditions
5. Benchmark critical operations
