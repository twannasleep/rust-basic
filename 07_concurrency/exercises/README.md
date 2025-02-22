# Concurrency Exercises

## 🌟 Exercise 1: Thread Basics

Create a parallel number processor that:

```rust
// TODO: Implement a system that:
// - Spawns multiple threads
// - Processes numbers in parallel
// - Collects and combines results

fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // Split work among threads
    // Process numbers
    // Combine results
}

fn main() {
    let numbers: Vec<i32> = (1..1000).collect();
    let results = process_numbers(numbers);
}
```

**Skills practiced:**

- Basic thread creation
- Thread joining
- Work distribution
- Result collection

## 🌟🌟 Exercise 2: Message Passing

Implement a concurrent task scheduler:

```rust
use std::sync::mpsc;
use std::thread;

enum Task {
    Process(String),
    Calculate(i32),
    Terminate,
}

struct TaskScheduler {
    sender: mpsc::Sender<Task>,
    workers: Vec<thread::JoinHandle<()>>,
}

// TODO: Implement the task scheduler
impl TaskScheduler {
    fn new(num_workers: usize) -> Self {
        // Create channels
        // Spawn worker threads
        // Return scheduler
    }

    fn schedule(&self, task: Task) {
        // Send task to workers
    }

    fn shutdown(self) {
        // Gracefully shutdown workers
    }
}
```

**Skills practiced:**

- Channel usage
- Multiple producers/consumers
- Thread coordination
- Graceful shutdown

## 🌟🌟 Exercise 3: Shared State

Create a thread-safe cache system:

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

struct Cache<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
    capacity: usize,
}

// TODO: Implement thread-safe cache operations
impl<K: Eq + Hash, V: Clone> Cache<K, V> {
    fn new(capacity: usize) -> Self {
        // Initialize cache
    }

    fn get(&self, key: &K) -> Option<V> {
        // Retrieve value if exists
    }

    fn set(&self, key: K, value: V) {
        // Add or update value
        // Respect capacity limit
    }
}
```

**Skills practiced:**

- Mutex usage
- Arc sharing
- Thread-safe data structures
- Concurrent access patterns

## 🌟🌟🌟 Exercise 4: Async Programming

Build an async task executor:

```rust
use tokio;

// TODO: Implement an async task executor that:
// - Handles multiple async tasks
// - Manages task priorities
// - Provides task cancellation
// - Reports task progress

struct TaskExecutor {
    tasks: Vec<Task>,
    running: bool,
}

impl TaskExecutor {
    async fn run_task(&self, task: Task) {
        // Execute task asynchronously
    }

    async fn run_all(&mut self) {
        // Run all tasks with proper concurrency
    }

    fn cancel_task(&mut self, task_id: usize) {
        // Cancel specific task
    }
}
```

**Skills practiced:**

- Async/await syntax
- Task management
- Error handling
- Resource management

## 🌟🌟🌟 Exercise 5: Advanced Synchronization

Implement a parallel pipeline processor:

```rust
use std::sync::{Arc, Barrier, Condvar, Mutex};

struct Pipeline {
    stages: Vec<Box<dyn Fn(Data) -> Data + Send + Sync>>,
    barriers: Vec<Arc<Barrier>>,
    data: Arc<Mutex<Vec<Data>>>,
}

// TODO: Implement a pipeline that:
// - Processes data through multiple stages
// - Maintains order
// - Handles backpressure
// - Provides progress monitoring

impl Pipeline {
    fn new(stages: Vec<Box<dyn Fn(Data) -> Data + Send + Sync>>) -> Self {
        // Initialize pipeline
    }

    fn process(&self, input: Vec<Data>) -> Vec<Data> {
        // Process data through pipeline
    }
}
```

**Skills practiced:**

- Complex synchronization
- Pipeline patterns
- Barrier usage
- Performance optimization

## Tips

1. Always handle thread panics
2. Use appropriate synchronization primitives
3. Consider deadlock prevention
4. Test with different thread counts
5. Monitor resource usage

## Evaluation Criteria

- Thread safety
- Performance
- Resource management
- Error handling
- Code organization
