r# 🔄 Concurrency in Rust

## 📋 Overview

Rust's concurrency model is built on the concept of "fearless concurrency" - making concurrent programming safe and efficient through compile-time guarantees.

## 🧵 Threads Basics

### Creating and Managing Threads

```rust
use std::thread;
use std::time::Duration;

// Basic thread creation
fn basic_threads() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread work
    for i in 1..5 {
        println!("Number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for spawned thread to finish
    handle.join().unwrap();
}

// Threads with move closure
fn move_closure_example() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });
    
    handle.join().unwrap();
}

// Thread builder with configuration
fn configured_thread() {
    let builder = thread::Builder::new()
        .name("worker".into())
        .stack_size(32 * 1024);
        
    let handle = builder.spawn(|| {
        println!("Running in configured thread");
    }).unwrap();
    
    handle.join().unwrap();
}
```

## 📬 Message Passing

### Channels for Thread Communication

```rust
use std::sync::mpsc;
use std::thread;

// Basic channel usage
fn basic_channel() {
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

    // Producer 1
    thread::spawn(move || {
        let messages = vec!["Hi", "How", "Are", "You"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Producer 2
    thread::spawn(move || {
        let messages = vec!["More", "Messages", "For", "You"];
        for msg in messages {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Consumer
    for received in rx {
        println!("Got: {}", received);
    }
}

// Synchronous channels
fn sync_channel() {
    let (tx, rx) = mpsc::sync_channel(2);  // Buffer size of 2
    
    let sender = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
            println!("Sent: {}", i);
        }
    });
    
    thread::sleep(Duration::from_secs(2));
    
    for _ in 1..=5 {
        println!("Received: {}", rx.recv().unwrap());
    }
}
```

## 🔒 Shared State

### Mutex and Arc for Safe Sharing

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Basic Mutex usage
fn mutex_example() {
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

    println!("Result: {}", *counter.lock().unwrap());
}

// RwLock for multiple readers
use std::sync::RwLock;

fn rwlock_example() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // Spawn multiple readers
    for _ in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let values = data.read().unwrap();
            println!("Read values: {:?}", *values);
        }));
    }
    
    // Spawn a writer
    {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut values = data.write().unwrap();
            values.push(4);
            println!("Updated values: {:?}", *values);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## ⚡ Async/Await

### Asynchronous Programming

```rust
use tokio;

// Basic async function
async fn fetch_data(id: u32) -> Result<String, Box<dyn std::error::Error>> {
    // Simulate async operation
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(format!("Data for id {}", id))
}

// Async main function
#[tokio::main]
async fn main() {
    // Sequential async operations
    let result1 = fetch_data(1).await.unwrap();
    println!("Result 1: {}", result1);
    
    // Parallel async operations
    let handle1 = tokio::spawn(async {
        fetch_data(2).await.unwrap()
    });
    
    let handle2 = tokio::spawn(async {
        fetch_data(3).await.unwrap()
    });
    
    // Wait for both operations
    let (result2, result3) = tokio::join!(handle1, handle2);
    println!("Results: {:?}, {:?}", result2.unwrap(), result3.unwrap());
}

// Stream processing
use tokio_stream::{self as stream, StreamExt};

async fn process_stream() {
    let mut stream = stream::iter(1..=3);
    
    while let Some(num) = stream.next().await {
        println!("Processing: {}", num);
    }
}
```

## 🔄 Synchronization Primitives

### Advanced Synchronization Tools

```rust
use std::sync::{Barrier, Condvar, Once};
use std::sync::atomic::{AtomicBool, Ordering};

// Barrier for thread synchronization
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

    for handle in handles {
        handle.join().unwrap();
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

// Atomic types
fn atomic_example() {
    let flag = Arc::new(AtomicBool::new(false));
    let flag2 = Arc::clone(&flag);
    
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        flag2.store(true, Ordering::SeqCst);
    });
    
    while !flag.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
    println!("Flag was set!");
}
```

## 🏭 Thread Pools

### Managing Thread Pools

```rust
use rayon::prelude::*;

// Basic parallel iterator
fn parallel_processing() {
    let numbers: Vec<i32> = (0..100).collect();
    let sum: i32 = numbers.par_iter()
                         .map(|&n| n * n)
                         .sum();
    println!("Sum of squares: {}", sum);
}

// Custom thread pool
use threadpool::ThreadPool;

fn thread_pool_example() {
    let pool = ThreadPool::new(4);
    let (tx, rx) = mpsc::channel();
    
    for i in 0..8 {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(format!("Task {} completed", i)).unwrap();
        });
    }
    
    drop(tx);
    
    for msg in rx.iter() {
        println!("{}", msg);
    }
}
```

## 🎯 Best Practices

### 1. Error Handling in Threads

```rust
fn error_handling() {
    let handle = thread::spawn(|| {
        if let Err(e) = potentially_failing_operation() {
            eprintln!("Thread error: {}", e);
            return Err(e);
        }
        Ok(())
    });

    match handle.join() {
        Ok(Ok(())) => println!("Thread completed successfully"),
        Ok(Err(e)) => println!("Thread returned error: {}", e),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
}
```

### 2. Resource Cleanup

```rust
struct CleanupGuard<T>(Arc<Mutex<T>>);

impl<T> Drop for CleanupGuard<T> {
    fn drop(&mut self) {
        let mut data = self.0.lock().unwrap();
        // Cleanup code here
    }
}

fn with_cleanup() {
    let data = Arc::new(Mutex::new(Vec::new()));
    let _guard = CleanupGuard(Arc::clone(&data));
    // Data will be cleaned up when guard is dropped
}
```

### 3. Thread Safety Patterns

```rust
// Thread-safe singleton
use std::sync::Once;

static INIT: Once = Once::new();
static mut SINGLETON: Option<Arc<MyService>> = None;

fn get_singleton() -> Arc<MyService> {
    unsafe {
        INIT.call_once(|| {
            SINGLETON = Some(Arc::new(MyService::new()));
        });
        Arc::clone(SINGLETON.as_ref().unwrap())
    }
}
```

Remember: Rust's concurrency features help prevent data races and ensure thread safety! 🚀
