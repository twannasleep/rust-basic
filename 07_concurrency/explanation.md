# Concurrency in Rust

## Threads Basics

```rust
use std::thread;
use std::time::Duration;

// Basic thread creation
fn basic_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
}

// Threads with move closure
fn move_closure() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });
    handle.join().unwrap();
}
```

## Message Passing

```rust
use std::sync::mpsc;

// Basic channel usage
fn channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());
}

// Multiple producers
fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send("Message 1").unwrap();
    });

    thread::spawn(move || {
        tx2.send("Message 2").unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

## Shared State

```rust
use std::sync::{Arc, Mutex};

// Mutex for shared state
fn shared_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// RwLock for multiple readers
use std::sync::RwLock;

fn reader_writer() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    
    // Reader
    let data_reader = Arc::clone(&data);
    thread::spawn(move || {
        let values = data_reader.read().unwrap();
        println!("Values: {:?}", *values);
    });

    // Writer
    let data_writer = Arc::clone(&data);
    thread::spawn(move || {
        let mut values = data_writer.write().unwrap();
        values.push(4);
    });
}
```

## Async/Await

```rust
use tokio;

// Basic async function
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    // Simulated async operation
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok("Data fetched!".to_string())
}

// Async main function
#[tokio::main]
async fn main() {
    let result = fetch_data().await.unwrap();
    println!("{}", result);
}

// Parallel async operations
async fn parallel_tasks() {
    let task1 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        "Task 1 complete"
    });

    let task2 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        "Task 2 complete"
    });

    let (result1, result2) = tokio::join!(task1, task2);
    println!("{}, {}", result1.unwrap(), result2.unwrap());
}
```

## Synchronization Primitives

```rust
use std::sync::{Barrier, Condvar, Once};

// Barrier for synchronization
fn barrier_example() {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} waiting", i);
            barrier.wait();
            println!("Thread {} passed barrier", i);
        }));
    }
}

// Condition Variables
fn condvar_example() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
}
```

## Thread Pools

```rust
use rayon::prelude::*;

// Basic parallel iterator
fn parallel_processing() {
    let numbers: Vec<i32> = (0..100).collect();
    let sum: i32 = numbers.par_iter()
                         .map(|&n| n * n)
                         .sum();
}

// Custom thread pool
use threadpool::ThreadPool;

fn thread_pool_example() {
    let pool = ThreadPool::new(4);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("Task {} executing", i);
        });
    }
    
    pool.join();
}
```

## Best Practices

```rust
// Error handling in threads
fn error_handling() {
    let handle = thread::spawn(|| {
        if some_condition() {
            Err("Thread error")
        } else {
            Ok("Success")
        }
    });

    match handle.join() {
        Ok(Ok(result)) => println!("Success: {}", result),
        Ok(Err(err)) => println!("Thread returned error: {}", err),
        Err(err) => println!("Thread panicked: {:?}", err),
    }
}

// Resource cleanup
fn cleanup() {
    let data = Arc::new(Mutex::new(Vec::new()));
    
    // Use drop guard for cleanup
    struct CleanupGuard<T>(Arc<Mutex<T>>);
    impl<T> Drop for CleanupGuard<T> {
        fn drop(&mut self) {
            // Cleanup code here
        }
    }
}
```
