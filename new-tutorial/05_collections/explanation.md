# Collections in Rust

## Vectors (Vec<T>)

```rust
// Creating vectors
let mut numbers: Vec<i32> = Vec::new();
let mut vec = vec![1, 2, 3];

// Common vector operations
vec.push(4);
vec.pop();
let third = &vec[2];
let maybe_value = vec.get(2);

// Iterating over vectors
for number in &vec {
    println!("{}", number);
}

// Vector with different types using enums
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
```

## HashMaps

```rust
use std::collections::HashMap;

// Creating HashMaps
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// Accessing and updating values
scores.entry(String::from("Yellow")).or_insert(50);
let score = scores.get("Blue");

// Updating based on old value
let text = "hello world hello";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

## String Types

```rust
// Creating strings
let mut s = String::new();
let s = String::from("initial contents");
let s = "initial contents".to_string();

// Updating strings
let mut s = String::from("foo");
s.push_str("bar");
s.push('!');

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```

## Custom Collections

```rust
// Implementing custom collections
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item)
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

## Advanced Vector Operations

```rust
// Slicing vectors
let vec = vec![1, 2, 3, 4, 5];
let slice = &vec[1..4];  // [2, 3, 4]

// Sorting and deduplication
let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
numbers.sort();
numbers.dedup();

// Vector capacity management
let mut vec = Vec::with_capacity(10);
println!("Capacity: {}", vec.capacity());
vec.reserve(20);
vec.shrink_to_fit();

// Advanced iteration
let vec = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = vec.iter()
    .map(|x| x * 2)
    .filter(|x| x > &5)
    .collect();
```

## Advanced HashMap Features

```rust
use std::collections::HashMap;

// Custom types as keys
#[derive(Hash, Eq, PartialEq)]
struct CustomKey {
    id: u32,
    category: String,
}

// Entry API
let mut map = HashMap::new();
map.entry("key")
   .or_insert_with(|| expensive_computation());

// Compound keys
let mut matrix = HashMap::new();
matrix.insert((1, 1), "Top left");
matrix.insert((1, 2), "Top right");

// Merging HashMaps
let mut map1 = HashMap::new();
let map2 = HashMap::new();
map1.extend(map2);
```

## Advanced String Operations

```rust
// String manipulation
let mut s = String::with_capacity(25);
s.extend(['a', 'b', 'c'].iter());
s.extend_from_slice("def");

// String searching
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

## Specialized Collections

```rust
use std::collections::{VecDeque, BTreeMap, BinaryHeap, HashSet};

// Double-ended queue
let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);

// Ordered map
let mut btree = BTreeMap::new();
btree.insert(3, "three");
btree.insert(1, "one");

// Priority queue (max heap)
let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(4);

// HashSet for unique items
let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
```

## Collection Traits

```rust
use std::iter::FromIterator;
use std::collections::LinkedList;

// Implementing collection traits
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

// Iterator implementation
impl<T> CustomCollection<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}
```

## Performance Considerations

```rust
// Capacity pre-allocation
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);  // No reallocations needed
}

// Efficient string building
let mut string = String::with_capacity(100);
for _ in 0..100 {
    string.push_str("a");  // No reallocations needed
}

// HashMap with custom hasher
use std::collections::hash_map::RandomState;
let map: HashMap<_, _, RandomState> = HashMap::with_hasher(RandomState::new());
```

## Thread-Safe Collections

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Thread-safe shared map
let shared_map: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));

// Thread-safe vector
let shared_vec: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

// Usage example
let map_clone = Arc::clone(&shared_map);
std::thread::spawn(move || {
    let mut map = map_clone.lock().unwrap();
    map.insert("key".to_string(), 42);
});
```
