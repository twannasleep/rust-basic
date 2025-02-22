// Example: Message Passing with Channels
// This example demonstrates various patterns for inter-thread communication using channels

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};

// Basic message passing between threads
fn basic_channel() {
    println!("Basic channel example:");
    
    // Create a channel
    let (tx, rx) = mpsc::channel();
    
    // Spawn thread that sends messages
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive messages in main thread
    for received in rx {
        println!("Got: {}", received);
    }
}

// Multiple producers sending to one receiver
fn multiple_producers() {
    println!("\nMultiple producers example:");
    
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Spawn multiple sender threads
    for id in 0..3 {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            tx.send(format!("Message from thread {}", id)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }));
    }
    
    // Drop original sender
    drop(tx);
    
    // Receive all messages
    for msg in rx {
        println!("Received: {}", msg);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
}

// Synchronous channels with bounded capacity
fn sync_channel() {
    println!("\nSynchronous channel example:");
    
    // Create a bounded channel with capacity 2
    let (tx, rx) = mpsc::sync_channel(2);
    
    thread::spawn(move || {
        for i in 1..=5 {
            println!("Sending {}", i);
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });
    
    thread::sleep(Duration::from_millis(100));
    
    for _ in 1..=5 {
        thread::sleep(Duration::from_millis(200));
        println!("Received: {}", rx.recv().unwrap());
    }
}

// Custom message type for complex communication
#[derive(Debug)]
enum WorkerMessage {
    Task(String),
    ProcessData(Vec<i32>),
    Shutdown,
}

fn complex_messages() {
    println!("\nComplex message example:");
    
    let (tx, rx) = mpsc::channel();
    
    // Spawn worker thread
    let handle = thread::spawn(move || {
        loop {
            match rx.recv().unwrap() {
                WorkerMessage::Task(task) => {
                    println!("Processing task: {}", task);
                }
                WorkerMessage::ProcessData(data) => {
                    println!("Processing data: {:?}", data);
                }
                WorkerMessage::Shutdown => {
                    println!("Shutting down worker");
                    break;
                }
            }
        }
    });
    
    // Send different types of messages
    tx.send(WorkerMessage::Task("Clean up".to_string())).unwrap();
    tx.send(WorkerMessage::ProcessData(vec![1, 2, 3])).unwrap();
    tx.send(WorkerMessage::Shutdown).unwrap();
    
    handle.join().unwrap();
}

// Channel as a pipeline for data processing
fn pipeline_processing() {
    println!("\nPipeline processing example:");
    
    // Create two channels for the pipeline
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();
    
    // Stage 1: Generate numbers
    thread::spawn(move || {
        for i in 0..5 {
            input_tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Stage 2: Square the numbers
    thread::spawn(move || {
        for num in input_rx {
            output_tx.send(num * num).unwrap();
        }
    });
    
    // Stage 3: Print results
    for result in output_rx {
        println!("Pipeline result: {}", result);
    }
}

fn main() {
    basic_channel();
    multiple_producers();
    sync_channel();
    complex_messages();
    pipeline_processing();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_channel() {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            tx.send(42).unwrap();
        });
        
        assert_eq!(rx.recv().unwrap(), 42);
    }
    
    #[test]
    fn test_multiple_producers() {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];
        let message_count = 3;
        
        for _ in 0..message_count {
            let tx = tx.clone();
            handles.push(thread::spawn(move || {
                tx.send(1).unwrap();
            }));
        }
        
        drop(tx);
        
        let sum: i32 = rx.iter().sum();
        assert_eq!(sum, message_count);
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn test_sync_channel() {
        let (tx, rx) = mpsc::sync_channel(1);
        
        thread::spawn(move || {
            tx.send(1).unwrap();
            tx.send(2).unwrap();
        });
        
        assert_eq!(rx.recv().unwrap(), 1);
        assert_eq!(rx.recv().unwrap(), 2);
    }
} 