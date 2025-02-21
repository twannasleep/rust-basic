# Modules and Packages in Rust

Rust has a module system that allows you to organize code into logical units and manage visibility between them.

## Module System Basics

### Creating Modules

```rust
// In lib.rs or main.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

### Module Tree Structure

```
crate
 └── front_of_house
     └── hosting
         └── add_to_waitlist
```

## Visibility and Privacy

### Public vs Private

- Items in modules are private by default
- Use the `pub` keyword to make items public
- Parent modules can't use private items in child modules
- Child modules can use items in their ancestor modules

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} // Public function
        fn seat_at_table() {}       // Private function
    }
}
```

### The `use` Keyword

```rust
use crate::front_of_house::hosting;
// Now we can use hosting::add_to_waitlist()

// Bringing function directly into scope
use crate::front_of_house::hosting::add_to_waitlist;
// Now we can just call add_to_waitlist()
```

## File Organization

### Multiple Files

```rust
// src/lib.rs or main.rs
mod front_of_house;  // Declares the module

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

### Module Files

```
src/
├── main.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

## Packages and Crates

### Crate Types

- Binary crates: Programs you can compile to an executable
- Library crates: Code intended to be used by other programs

### Package Structure

```
my_package/
├── Cargo.toml
├── src/
│   ├── main.rs     // Binary crate root
│   ├── lib.rs      // Library crate root
│   └── bin/        // Additional binaries
│       └── more.rs
```

## Common Module Patterns

### Re-exporting

```rust
pub use crate::front_of_house::hosting;
// Makes hosting available to external code
```

### Nested Paths

```rust
use std::{io, cmp::Ordering};
// Instead of:
// use std::io;
// use std::cmp::Ordering;
```

### The Glob Operator

```rust
use std::collections::*;
// Brings all public items into scope
```

## Best Practices

1. **Module Organization**
   - Keep related functionality together
   - Use clear, descriptive module names
   - Follow the principle of least privilege

2. **File Structure**
   - One module per file
   - Match file names to module names
   - Use mod.rs for module hierarchies

3. **Visibility**
   - Make items public only when necessary
   - Use re-exports to provide a clean public API
   - Document public interfaces

## Working with External Crates

### Adding Dependencies

```toml
[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
```

### Using External Crates

```rust
use rand::Rng;
use serde::{Serialize, Deserialize};
```

## Common Module Structures

### API Design

```rust
// lib.rs
pub mod api {
    pub mod v1 {
        pub fn endpoint() {}
    }
    pub mod v2 {
        pub fn endpoint() {}
    }
}
```

### Feature Organization

```rust
// lib.rs
pub mod features {
    pub mod authentication {
        pub mod oauth {
            pub fn authenticate() {}
        }
    }
}
```

## Exercises

1. Create a library crate with multiple modules
2. Implement a module hierarchy with public and private items
3. Practice using different visibility levels
4. Create and use external crate dependencies
5. Organize code using common module patterns

## Next Steps

After completing this section, you should:

- Understand Rust's module system
- Know how to organize code into modules
- Be comfortable with visibility rules
- Know how to work with external crates
- Be able to create well-organized Rust projects
