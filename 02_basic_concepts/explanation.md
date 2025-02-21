# Basic Concepts in Rust

## Variables and Mutability

In Rust, variables are immutable by default. This is one of Rust's many design decisions that help ensure memory safety and make concurrent programming easier.

### Variable Declaration

```rust
// Immutable variable
let x = 5;

// Mutable variable
let mut y = 5;
y = 6; // This is allowed

// Constants
const MAX_POINTS: u32 = 100_000;

// Shadowing
let x = 5;
let x = x + 1; // Creates a new variable, shadows the previous one
```

## Data Types

Rust is a statically typed language, which means it must know the types of all variables at compile time.

### Scalar Types

1. **Integers**
   - Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
   - Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

2. **Floating-Point**
   - `f32`: single precision
   - `f64`: double precision (default)

3. **Boolean**
   - `bool`: true or false

4. **Character**
   - `char`: Unicode scalar value

### Compound Types

1. **Tuples**
   - Fixed-length collection of values of different types

   ```rust
   let tup: (i32, f64, u8) = (500, 6.4, 1);
   ```

2. **Arrays**
   - Fixed-length collection of same-type values

   ```rust
   let arr: [i32; 5] = [1, 2, 3, 4, 5];
   ```

## Functions

Functions are the building blocks of readable, maintainable code.

### Function Declaration

```rust
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // function body
}
```

### Return Values

- Implicit return: last expression in a block
- Explicit return: using `return` keyword

## Control Flow

### If Expressions

```rust
if condition {
    // code
} else if another_condition {
    // code
} else {
    // code
}
```

### Loops

1. **loop**
   - Infinite loop until break

   ```rust
   loop {
       // code
       if condition {
           break;
       }
   }
   ```

2. **while**

   ```rust
   while condition {
       // code
   }
   ```

3. **for**

   ```rust
   for element in collection {
       // code
   }
   ```

## Comments and Documentation

### Regular Comments

```rust
// Line comments
/* Block comments */
```

### Documentation Comments

```rust
/// Generate documentation for the following item
//! Generate documentation for the enclosing item
```

## Exercises

1. Create variables of different types and experiment with mutability
2. Write a function that uses different control flow structures
3. Create and manipulate compound data types
4. Write documented code with both regular and documentation comments

## Next Steps

After completing this section, you should:

- Understand Rust's type system
- Be comfortable with variables and mutability
- Know how to write functions and control flow
- Be able to write well-documented code
