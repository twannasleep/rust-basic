# Collections Exercises

## Exercise 1: Task Management System

Create a task management system using vectors and custom types:

1. Implement a Task struct with:
   - ID
   - Title
   - Description
   - Status (enum)
   - Due date
2. Create a TaskManager that can:
   - Add tasks
   - Remove tasks
   - Update task status
   - List tasks by status
   - Sort tasks by due date

Example structure:
```rust
#[derive(Debug)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

struct Task {
    id: u32,
    title: String,
    description: String,
    status: TaskStatus,
    due_date: chrono::DateTime<chrono::Utc>,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    // TODO: Implement task management methods
    fn add_task(&mut self, task: Task) {
        // Implementation
    }
    
    fn list_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        // Implementation
    }
}
```

## Exercise 2: Word Frequency Analyzer

Create a text analysis tool using HashMaps:

1. Count word frequencies
2. Track word positions
3. Find most common words
4. Implement case-insensitive matching
5. Handle punctuation properly

Example structure:
```rust
struct WordStats {
    count: usize,
    positions: Vec<usize>,
    first_seen: usize,
}

struct TextAnalyzer {
    word_stats: HashMap<String, WordStats>,
    total_words: usize,
}

impl TextAnalyzer {
    fn analyze(&mut self, text: &str) {
        // TODO: Implement text analysis
    }
    
    fn most_common_words(&self, limit: usize) -> Vec<(&String, usize)> {
        // TODO: Return most frequent words
    }
}
```

## Exercise 3: Custom Collection

Implement a priority queue using vectors:

1. Create a generic PriorityQueue<T>
2. Implement basic operations:
   - Push with priority
   - Pop highest priority
   - Peek
   - Clear
3. Implement custom iterators
4. Add proper error handling

Example structure:
```rust
#[derive(Debug)]
struct PriorityQueue<T> {
    items: Vec<(T, u32)>, // (item, priority)
}

impl<T> PriorityQueue<T> {
    fn new() -> Self {
        // TODO: Initialize empty queue
    }
    
    fn push(&mut self, item: T, priority: u32) {
        // TODO: Add item with priority
    }
    
    fn pop(&mut self) -> Option<T> {
        // TODO: Remove and return highest priority item
    }
}

impl<T> Iterator for PriorityQueue<T> {
    // TODO: Implement iterator
}
```

## Exercise 4: Multi-Map Implementation

Create a multi-map data structure that allows multiple values per key:

1. Use a combination of HashMap and Vec
2. Implement methods for:
   - Adding values
   - Removing values
   - Getting all values for a key
   - Merging multi-maps
3. Make it generic over key and value types
4. Implement proper iteration

Example structure:
```rust
#[derive(Debug)]
struct MultiMap<K, V> {
    data: HashMap<K, Vec<V>>,
}

impl<K: Hash + Eq, V> MultiMap<K, V> {
    fn insert(&mut self, key: K, value: V) {
        // TODO: Insert value for key
    }
    
    fn get_all(&self, key: &K) -> Option<&Vec<V>> {
        // TODO: Get all values for key
    }
    
    fn remove(&mut self, key: &K, value: &V) -> bool {
        // TODO: Remove specific value for key
    }
}
```

## Bonus Challenge: Concurrent Collection

Implement a thread-safe collection:

1. Use Arc and Mutex for shared access
2. Implement methods for:
   - Adding items
   - Removing items
   - Updating items
   - Atomic operations
3. Handle deadlock prevention
4. Implement proper error handling

Example structure:
```rust
use std::sync::{Arc, Mutex};

struct ConcurrentCollection<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T: Clone> ConcurrentCollection<T> {
    fn new() -> Self {
        // TODO: Initialize thread-safe collection
    }
    
    fn add(&self, item: T) -> Result<(), String> {
        // TODO: Add item safely
    }
    
    fn update<F>(&self, index: usize, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut T)
    {
        // TODO: Update item safely
    }
}
```

## Evaluation Criteria

Your solutions will be evaluated based on:

1. Proper use of Rust collections
2. Implementation of required functionality
3. Error handling
4. Code organization and documentation
5. Performance considerations

## Testing

For each exercise:

1. Write unit tests for all functionality
2. Test edge cases
3. Test concurrent access (where applicable)
4. Benchmark performance for large datasets
