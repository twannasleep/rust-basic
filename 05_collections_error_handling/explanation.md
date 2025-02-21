# Collections and Error Handling in Rust

## Collections

Rust's standard library includes several very useful data structures called collections. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap.

### Vectors (Vec<T>)

A vector allows you to store a variable number of values in a single data structure that puts all the values next to each other in memory.

```rust
// Creating a vector
let mut vec = Vec::new();
vec.push(1);

// Creating with initial values
let vec = vec![1, 2, 3];

// Accessing elements
let third: &i32 = &vec[2];
match vec.get(2) {
    Some(third) => println!("Third element is {}", third),
    None => println!("There is no third element."),
}
```

### Strings

Rust has two main string types: `String` and `&str`.

```rust
// Creating a String
let mut s = String::new();
let s = String::from("initial contents");
let s = "initial contents".to_string();

// Updating a String
let mut s = String::from("foo");
s.push_str("bar");

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note: s1 has been moved here
```

### HashMaps

The type `HashMap<K, V>` stores a mapping of keys of type K to values of type V.

```rust
use std::collections::HashMap;

// Creating a new HashMap
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Accessing values
match scores.get("Blue") {
    Some(score) => println!("Blue team score: {}", score),
    None => println!("Blue team doesn't have a score"),
}

// Updating values
scores.entry(String::from("Yellow")).or_insert(50);
```

## Error Handling

Rust groups errors into two major categories: recoverable and unrecoverable errors.

### Recoverable Errors with Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Using Result
fn get_username(id: i32) -> Result<String, String> {
    if id < 1 {
        return Err(String::from("Invalid ID"));
    }
    Ok(String::from("alice"))
}

// Handling Result
match get_username(0) {
    Ok(username) => println!("Username: {}", username),
    Err(error) => println!("Error: {}", error),
}

// Using ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

### Unrecoverable Errors with panic

```rust
// Causes a panic
panic!("crash and burn");

// Accessing an array element beyond its bounds
let v = vec![1, 2, 3];
v[99]; // This will panic!
```

### The Option Type

```rust
enum Option<T> {
    Some(T),
    None,
}

// Using Option
fn find_user(id: i32) -> Option<String> {
    if id > 0 {
        Some(String::from("alice"))
    } else {
        None
    }
}

// Handling Option
match find_user(1) {
    Some(user) => println!("User found: {}", user),
    None => println!("User not found"),
}
```

## Common Collection Patterns

### Vector Operations

```rust
// Filtering
let filtered: Vec<i32> = vec.iter()
    .filter(|&x| x > &0)
    .cloned()
    .collect();

// Mapping
let doubled: Vec<i32> = vec.iter()
    .map(|x| x * 2)
    .collect();
```

### String Operations

```rust
// String manipulation
let trimmed = s.trim();
let uppercase = s.to_uppercase();
let words: Vec<&str> = s.split_whitespace().collect();
```

### HashMap Operations

```rust
// Counting occurrences
let text = "hello world hello";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

## Error Handling Best Practices

1. Use `Result` for recoverable errors
2. Use `panic!` for unrecoverable errors
3. Use the `?` operator to propagate errors
4. Create custom error types for your libraries
5. Provide meaningful error messages

## Exercises

1. Create a program that manages a list of items using a vector
2. Build a simple text processing tool using string operations
3. Create a word frequency counter using a HashMap
4. Implement error handling for a file processing function
5. Practice using Option and Result in various scenarios

## Next Steps

After completing this section, you should:

- Understand how to use Rust's common collections
- Know when to use each collection type
- Be comfortable with error handling in Rust
- Understand the difference between Result and Option
- Be able to implement proper error handling in your code
