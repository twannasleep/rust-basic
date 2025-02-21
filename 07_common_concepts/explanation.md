# Common Programming Concepts in Rust

## Generic Types

Generics allow you to write code that can work with multiple types while maintaining type safety.

### Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

// Multiple type parameters
struct KeyValue<K, V> {
    key: K,
    value: V,
}
```

## Traits

Traits define shared behavior across types, similar to interfaces in other languages.

### Defining Traits

```rust
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### Implementing Traits

```rust
struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}
```

### Trait Bounds

```rust
// Single trait bound
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify<T: Summary + Display>(item: &T) { }

// Where clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ }
```

## Lifetimes

Lifetimes ensure that references are valid for the duration they're being used.

### Lifetime Annotations

```rust
// Basic lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with references
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### Lifetime Elision Rules

1. Each parameter that is a reference gets its own lifetime parameter
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

## Testing

Rust has built-in support for testing your code.

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "panic message")]
    fn it_panics() {
        panic!("panic message");
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn it_adds_two() {
    assert_eq!(4, my_crate::add_two(2));
}
```

## Advanced Features

### Associated Types

```rust
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}
```

### Default Type Parameters

```rust
trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
```

### Operator Overloading

```rust
use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

### Smart Pointers

```rust
// Box<T> for heap allocation
let b = Box::new(5);

// Rc<T> for multiple ownership
use std::rc::Rc;
let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);
```

## Best Practices

1. **Generic Code**
   - Use generics to reduce code duplication
   - Implement traits for custom types
   - Use trait bounds to specify behavior

2. **Error Handling**
   - Use Result for recoverable errors
   - Use Option for optional values
   - Implement custom error types

3. **Testing**
   - Write unit tests alongside code
   - Create integration tests for public API
   - Use test organization techniques

## Exercises

1. Create a generic data structure with associated methods
2. Implement a trait for multiple types
3. Write code that uses lifetime annotations
4. Create a comprehensive test suite
5. Implement operator overloading for a custom type

## Next Steps

After completing this section, you should:

- Understand how to use generics and traits
- Be comfortable with lifetime annotations
- Know how to write and organize tests
- Be familiar with advanced Rust features
- Be ready to write complex Rust programs
