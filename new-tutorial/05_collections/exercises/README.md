# Collections Exercises

## Exercise 1: Vector Operations

Create a program that:

- Implements a dynamic task list
- Supports adding and removing tasks
- Allows marking tasks as complete
- Provides task filtering capabilities

```rust
struct Task {
    description: String,
    completed: bool,
}

struct TaskList {
    tasks: Vec<Task>,
}

// TODO: Implement TaskList methods
```

## Exercise 2: HashMap Usage

Build a word frequency counter that:

- Counts word occurrences in text
- Ignores case sensitivity
- Handles punctuation
- Provides frequency statistics

```rust
use std::collections::HashMap;

// TODO: Implement word frequency counter
fn count_words(text: &str) -> HashMap<String, usize> {
    // Implementation here
}
```

## Exercise 3: Custom Collection

Implement a priority queue that:

- Maintains items in priority order
- Supports adding and removing items
- Allows updating priorities
- Provides efficient access to highest priority item

```rust
struct PriorityQueue<T> {
    // TODO: Define structure
}

impl<T: Ord> PriorityQueue<T> {
    // TODO: Implement priority queue methods
}
```

## Exercise 4: String Processing

Create a text processing system that:

- Handles multiple string formats
- Performs text transformations
- Manages string concatenation efficiently
- Implements custom string splitting

```rust
struct TextProcessor {
    // TODO: Define text processor structure
}

// TODO: Implement text processing methods
```

## Challenge Exercise: Advanced Collections

Implement a cache system with:

- Limited size (LRU cache)
- Automatic cleanup of old entries
- Statistics tracking
- Thread-safe operations

```rust
use std::collections::HashMap;
use std::time::Instant;

struct CacheEntry<T> {
    value: T,
    timestamp: Instant,
}

struct Cache<K, V> {
    // TODO: Define cache structure
}

// TODO: Implement cache methods
```
