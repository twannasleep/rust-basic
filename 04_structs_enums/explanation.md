# Structs and Enums in Rust

Structs and enums are the building blocks for creating custom data types in Rust.

## Structs

A struct is a custom data type that lets you package together and name multiple related values.

### Types of Structs

1. **Classic C-like Structs**

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

2. **Tuple Structs**

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

3. **Unit-Like Structs**

```rust
struct AlwaysEqual;
```

### Using Structs

```rust
// Creating instances
let user = User {
    email: String::from("user@example.com"),
    username: String::from("user123"),
    active: true,
    sign_in_count: 1,
};

// Updating instances
let mut user2 = user;
user2.email = String::from("another@example.com");

// Struct update syntax
let user3 = User {
    email: String::from("another@example.com"),
    ..user2
};
```

### Method Syntax

```rust
impl Rectangle {
    // Associated function (static method)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

## Enums

Enums allow you to define a type by enumerating its possible variants.

### Basic Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

### Enum with Data

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

### Complex Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Pattern Matching

Pattern matching is a powerful feature in Rust that works well with enums.

### Match Expression

```rust
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    },
}
```

### Option Enum

```rust
enum Option<T> {
    Some(T),
    None,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

### if let Syntax

```rust
if let Some(3) = some_value {
    println!("three");
}
```

## Best Practices

1. Use structs to group related data
2. Implement methods for behavior associated with the data
3. Use enums when you have a fixed set of possibilities
4. Use pattern matching to handle all cases
5. Consider using Option instead of null values

## Common Patterns

### Builder Pattern

```rust
impl User {
    fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    fn email(mut self, email: String) -> UserBuilder {
        self.email = Some(email);
        self
    }
    // ... other builder methods
}
```

### Type State Pattern

```rust
enum State {
    Draft,
    Review,
    Published,
}

struct Post {
    state: State,
    content: String,
}
```

## Exercises

1. Create a struct to represent a rectangle with width and height
2. Implement methods to calculate area and perimeter
3. Create an enum to represent different shapes
4. Implement pattern matching for the shapes enum
5. Use Option<T> to handle nullable values

## Next Steps

After completing this section, you should:

- Understand how to define and use structs
- Know how to implement methods for structs
- Be comfortable with enums and their variants
- Understand pattern matching
- Be able to use Option<T> effectively
