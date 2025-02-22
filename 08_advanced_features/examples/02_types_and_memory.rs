// Example: Advanced Types and Memory Management
// This example demonstrates advanced type features and memory management concepts

use std::rc::Rc;
use std::cell::RefCell;
use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;

// =============== Advanced Types Examples ===============

// Type aliases for complex types
type ThunkBox = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, AppError>;

// Custom error type
#[derive(Debug)]
struct AppError {
    message: String,
}

// Never type (!)
fn forever() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Forever...");
    }
}

// Newtype pattern
#[derive(Debug, PartialEq)]
struct Kilometers(f64);
struct Miles(f64);

impl From<Miles> for Kilometers {
    fn from(miles: Miles) -> Self {
        Kilometers(miles.0 * 1.609344)
    }
}

// Zero-sized type
struct Unit;

// Phantom types for compile-time checks
struct Token<T> {
    id: String,
    _marker: PhantomData<T>,
}

struct AdminPrivileges;
struct UserPrivileges;

impl<T> Token<T> {
    fn new(id: String) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

// =============== Memory Management Examples ===============

// Reference counting with interior mutability
#[derive(Debug)]
struct SharedState {
    data: Rc<RefCell<Vec<i32>>>,
}

impl SharedState {
    fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    fn add(&self, value: i32) {
        self.data.borrow_mut().push(value);
    }
    
    fn get_data(&self) -> Vec<i32> {
        self.data.borrow().clone()
    }
}

// Custom allocation example
struct CustomVec {
    ptr: *mut u32,
    len: usize,
    capacity: usize,
}

impl CustomVec {
    fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    unsafe fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<u32>(capacity).unwrap();
        let ptr = alloc(layout) as *mut u32;
        
        Self {
            ptr,
            len: 0,
            capacity,
        }
    }
}

impl Drop for CustomVec {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                let layout = Layout::array::<u32>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

// =============== Main Function ===============

fn main() {
    // Type aliases and newtypes
    let distance = Kilometers(100.0);
    let miles = Miles(62.1371);
    let km_from_miles: Kilometers = miles.into();
    println!("Distance: {:?}", distance);
    println!("Converted from miles: {:?}", km_from_miles);
    
    // Phantom types for type-level security
    let admin_token = Token::<AdminPrivileges>::new("admin123".to_string());
    let user_token = Token::<UserPrivileges>::new("user456".to_string());
    println!("Admin token: {}", admin_token.id);
    println!("User token: {}", user_token.id);
    
    // Reference counting and interior mutability
    let state1 = SharedState::new();
    let state2 = SharedState {
        data: Rc::clone(&state1.data),
    };
    
    state1.add(1);
    state2.add(2);
    println!("Shared state data: {:?}", state1.get_data());
    
    // Custom allocation
    unsafe {
        let mut vec = CustomVec::with_capacity(5);
        println!("CustomVec capacity: {}", vec.capacity);
        // vec is automatically deallocated when it goes out of scope
    }
    
    // Never type example (commented out as it's infinite)
    // forever();
}

// =============== Tests ===============

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kilometers_conversion() {
        let miles = Miles(10.0);
        let km: Kilometers = miles.into();
        assert_eq!(km, Kilometers(16.09344));
    }
    
    #[test]
    fn test_shared_state() {
        let state1 = SharedState::new();
        let state2 = SharedState {
            data: Rc::clone(&state1.data),
        };
        
        state1.add(1);
        state2.add(2);
        
        assert_eq!(state1.get_data(), vec![1, 2]);
        assert_eq!(state2.get_data(), vec![1, 2]);
    }
    
    #[test]
    fn test_custom_vec() {
        unsafe {
            let vec = CustomVec::with_capacity(5);
            assert_eq!(vec.capacity, 5);
            assert_eq!(vec.len, 0);
        }
    }
    
    #[test]
    fn test_phantom_types() {
        let admin_token = Token::<AdminPrivileges>::new("admin123".to_string());
        let user_token = Token::<UserPrivileges>::new("user456".to_string());
        
        assert_eq!(admin_token.id, "admin123");
        assert_eq!(user_token.id, "user456");
    }
} 