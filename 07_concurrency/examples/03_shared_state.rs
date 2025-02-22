// Example: Shared State Concurrency
// This example demonstrates various ways to share state between threads

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock, Barrier, Condvar};
use std::collections::HashMap;

// Basic mutex usage for shared state
fn mutex_counter() {
    println!("Mutex counter example:");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter to {}", i, *num);
            // Lock is automatically released here
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
}

// RwLock for multiple readers
fn rwlock_example() {
    println!("\nRwLock example:");
    
    let data = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];
    
    // Spawn writer threads
    for i in 0..2 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut map = data.write().unwrap();
            map.insert(i, i * i);
            println!("Writer {} added value", i);
            thread::sleep(Duration::from_millis(100));
        }));
    }
    
    // Spawn reader threads
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let map = data.read().unwrap();
            println!("Reader {} sees: {:?}", i, *map);
            thread::sleep(Duration::from_millis(50));
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// Using Barrier for thread synchronization
fn barrier_example() {
    println!("\nBarrier example:");
    
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} doing some work", i);
            thread::sleep(Duration::from_millis(100 * i as u64));
            
            println!("Thread {} waiting at barrier", i);
            barrier.wait();
            println!("Thread {} passed barrier", i);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// Using Condvar for thread coordination
struct SharedState {
    completed: bool,
    data: Vec<i32>,
}

fn condvar_example() {
    println!("\nCondvar example:");
    
    let state = Arc::new((Mutex::new(SharedState {
        completed: false,
        data: Vec::new(),
    }), Condvar::new()));
    
    let state_producer = Arc::clone(&state);
    let producer = thread::spawn(move || {
        let (lock, cvar) = &*state_producer;
        let mut state = lock.lock().unwrap();
        
        // Simulate some work
        println!("Producer: working...");
        thread::sleep(Duration::from_millis(500));
        
        // Update state and notify consumer
        state.data.extend_from_slice(&[1, 2, 3, 4, 5]);
        state.completed = true;
        println!("Producer: data ready");
        cvar.notify_one();
    });
    
    let state_consumer = Arc::clone(&state);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*state_consumer;
        let mut state = lock.lock().unwrap();
        
        println!("Consumer: waiting for data...");
        while !state.completed {
            state = cvar.wait(state).unwrap();
        }
        
        println!("Consumer: received data {:?}", state.data);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

// Deadlock prevention example
struct BankAccount {
    balance: i32,
    id: i32,
}

impl BankAccount {
    fn new(id: i32, initial_balance: i32) -> Self {
        BankAccount {
            id,
            balance: initial_balance,
        }
    }
    
    fn transfer(&mut self, other: &mut BankAccount, amount: i32) {
        self.balance -= amount;
        other.balance += amount;
    }
}

fn deadlock_prevention() {
    println!("\nDeadlock prevention example:");
    
    let account1 = Arc::new(Mutex::new(BankAccount::new(1, 100)));
    let account2 = Arc::new(Mutex::new(BankAccount::new(2, 100)));
    
    let mut handles = vec![];
    
    // Safe transfer function that prevents deadlock
    let transfer = |from: &Arc<Mutex<BankAccount>>, to: &Arc<Mutex<BankAccount>>, amount: i32| {
        // Always lock accounts in a consistent order (by ID)
        let (first, second) = if from.lock().unwrap().id < to.lock().unwrap().id {
            (from, to)
        } else {
            (to, from)
        };
        
        let mut first_acc = first.lock().unwrap();
        let mut second_acc = second.lock().unwrap();
        
        if from.lock().unwrap().id == first_acc.id {
            first_acc.transfer(&mut second_acc, amount);
        } else {
            second_acc.transfer(&mut first_acc, amount);
        }
    };
    
    // Spawn threads that perform transfers
    for i in 0..4 {
        let acc1 = Arc::clone(&account1);
        let acc2 = Arc::clone(&account2);
        handles.push(thread::spawn(move || {
            println!("Thread {} starting transfer", i);
            transfer(&acc1, &acc2, 10);
            println!("Thread {} completed transfer", i);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final balances:");
    println!("Account 1: {}", account1.lock().unwrap().balance);
    println!("Account 2: {}", account2.lock().unwrap().balance);
}

fn main() {
    mutex_counter();
    rwlock_example();
    barrier_example();
    condvar_example();
    deadlock_prevention();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mutex_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*counter.lock().unwrap(), 5);
    }
    
    #[test]
    fn test_rwlock() {
        let data = Arc::new(RwLock::new(HashMap::new()));
        
        // Test write access
        {
            let mut map = data.write().unwrap();
            map.insert(1, "test");
        }
        
        // Test multiple read access
        let mut handles = vec![];
        for _ in 0..3 {
            let data = Arc::clone(&data);
            handles.push(thread::spawn(move || {
                let map = data.read().unwrap();
                assert_eq!(map.get(&1), Some(&"test"));
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn test_bank_transfer() {
        let account1 = Arc::new(Mutex::new(BankAccount::new(1, 100)));
        let account2 = Arc::new(Mutex::new(BankAccount::new(2, 100)));
        
        let mut handles = vec![];
        for _ in 0..4 {
            let acc1 = Arc::clone(&account1);
            let acc2 = Arc::clone(&account2);
            handles.push(thread::spawn(move || {
                let mut first = acc1.lock().unwrap();
                let mut second = acc2.lock().unwrap();
                first.transfer(&mut second, 10);
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(account1.lock().unwrap().balance, 60);
        assert_eq!(account2.lock().unwrap().balance, 140);
    }
} 