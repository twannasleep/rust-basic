# 📚 Collections in Rust

## 📋 Overview

Rust provides several collection types to store multiple values. Each type has its own characteristics and use cases.

## 📦 Vectors (Vec<T>)

A growable array type that stores elements in contiguous memory.

### Basic Vector Operations

```rust
// Creating vectors
let mut numbers: Vec<i32> = Vec::new();
let mut vec = vec![1, 2, 3];  // Using vec! macro

// Adding elements
vec.push(4);
vec.extend([5, 6, 7].iter());

// Removing elements
let last = vec.pop();  // Returns Option<T>
vec.remove(1);  // Removes and returns element at index

// Accessing elements
let third = &vec[2];  // Panics if out of bounds
let safe_access = vec.get(2);  // Returns Option<&T>

// Iterating
for number in &vec {
    println!("{}", number);
}

// With index
for (i, number) in vec.iter().enumerate() {
    println!("Index {}: {}", i, number);
}
```

### Advanced Vector Operations

```rust
// Vector with different types using enum
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(42),
    SpreadsheetCell::Float(3.14),
    SpreadsheetCell::Text(String::from("hello")),
];

// Capacity management
let mut vec = Vec::with_capacity(10);
println!("Capacity: {}", vec.capacity());
vec.reserve(20);
vec.shrink_to_fit();

// Sorting and deduplication
let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
numbers.sort();
numbers.dedup();

// Advanced iteration
let vec = vec![1, 2, 3, 4, 5];
let sum: i32 = vec.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();
```

## 🗺️ HashMaps

A hash map stores key-value pairs with O(1) average access time.

### Basic HashMap Operations

```rust
use std::collections::HashMap;

// Creating HashMaps
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// From pairs of items
let teams = vec![String::from("Blue"), String::from("Red")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// Accessing values
if let Some(score) = scores.get("Blue") {
    println!("Blue team score: {}", score);
}

// Entry API for insertion
scores.entry(String::from("Yellow")).or_insert(50);

// Update based on old value
let text = "hello world hello";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

### Advanced HashMap Features

```rust
// Custom types as keys
#[derive(Hash, Eq, PartialEq, Debug)]
struct CustomKey {
    id: u32,
    category: String,
}

let mut map = HashMap::new();
map.insert(
    CustomKey { id: 1, category: String::from("test") },
    "value"
);

// Compound keys
let mut matrix = HashMap::new();
matrix.insert((1, 1), "Top left");
matrix.insert((1, 2), "Top right");

// Merging HashMaps
let mut map1 = HashMap::new();
map1.insert("a", 1);
let mut map2 = HashMap::new();
map2.insert("b", 2);
map1.extend(map2);

// Custom hasher
use std::collections::hash_map::RandomState;
let map: HashMap<_, _, RandomState> = HashMap::with_hasher(RandomState::new());
```

## 📝 String Types

Rust strings are UTF-8 encoded and support Unicode.

### String Operations

```rust
// Creating strings
let mut s = String::new();
let s = String::from("initial contents");
let s = "initial contents".to_string();

// Updating strings
let mut s = String::from("foo");
s.push_str("bar");  // Append string slice
s.push('!');        // Append single character

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 is moved here

// Format macro
let s = format!("{}-{}-{}", "2024", "01", "01");

// String manipulation
let text = "Hello, world!";
assert!(text.contains("world"));
assert!(text.starts_with("Hello"));
assert!(text.ends_with("!"));

// String splitting and joining
let words: Vec<&str> = "rust is awesome".split_whitespace().collect();
let joined = words.join("-");

// Unicode support
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

## 🔄 Specialized Collections

### VecDeque (Double-ended queue)

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);
assert_eq!(deque.pop_front(), Some(1));
```

### BTreeMap (Ordered map)

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "three");
map.insert(1, "one");
// Keys are always in sorted order
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

### BinaryHeap (Priority queue)

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();  // Max heap
heap.push(3);
heap.push(1);
heap.push(4);
assert_eq!(heap.pop(), Some(4));
```

### HashSet (Unique items)

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
assert!(set.contains("apple"));
```

## 🛠️ Collection Traits

### Implementing Collection Traits

```rust
use std::iter::FromIterator;

struct CustomCollection<T> {
    data: Vec<T>,
}

impl<T> FromIterator<T> for CustomCollection<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        CustomCollection {
            data: Vec::from_iter(iter),
        }
    }
}

impl<T> CustomCollection<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}
```

## 🔒 Thread-Safe Collections

### Shared Collections

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

// Thread-safe shared map
let shared_map: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));

// Thread-safe vector
let shared_vec: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

// Usage example
let map_clone = Arc::clone(&shared_map);
thread::spawn(move || {
    let mut map = map_clone.lock().unwrap();
    map.insert("key".to_string(), 42);
});
```

## 🎯 Best Practices

1. **Choosing the Right Collection**
   - Use `Vec` for sequential data
   - Use `HashMap` for key-value lookups
   - Use `HashSet` for unique items
   - Use `VecDeque` for FIFO queues
   - Use `BTreeMap` for sorted data

2. **Performance Considerations**
   - Pre-allocate with known size
   - Use appropriate capacity hints
   - Consider using specialized collections
   - Profile your collection usage

3. **Memory Management**
   - Clear collections when no longer needed
   - Use `shrink_to_fit` after large removals
   - Consider using arena allocators for complex structures

4. **Thread Safety**
   - Use `Arc<Mutex<T>>` for shared mutable collections
   - Consider using lock-free collections
   - Be aware of deadlock possibilities

Remember: Choose the right collection for your use case, and consider the performance implications of your choices! 🚀
