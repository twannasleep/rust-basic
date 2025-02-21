use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::marker::PhantomData;

// Custom vector implementation
pub struct CustomVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
    _marker: PhantomData<T>,
}

impl<T> CustomVec<T> {
    pub fn new() -> Self {
        CustomVec {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).unwrap();
        // Safety: layout size is not zero as capacity > 0
        let ptr = unsafe { NonNull::new_unchecked(alloc::alloc(layout) as *mut T) };

        CustomVec {
            ptr,
            len: 0,
            capacity,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            std::ptr::write(self.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(std::ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                Some(&*self.ptr.as_ptr().add(index))
            }
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                Some(&mut *self.ptr.as_ptr().add(index))
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::realloc(
                    self.ptr.as_ptr() as *mut u8,
                    old_layout,
                    new_layout.size(),
                )
            }
        };

        // Update pointer and capacity
        self.ptr = unsafe { NonNull::new_unchecked(new_ptr as *mut T) };
        self.capacity = new_capacity;
    }
}

impl<T> Drop for CustomVec<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            // Drop all elements
            while self.pop().is_some() {}
            
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

// Example usage and tests
fn main() {
    // Create a new vector
    let mut vec = CustomVec::new();
    
    // Push some values
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    println!("Vector length: {}", vec.len());
    println!("Vector capacity: {}", vec.capacity());
    
    // Get references
    if let Some(value) = vec.get(1) {
        println!("Value at index 1: {}", value);
    }
    
    // Modify through mutable reference
    if let Some(value) = vec.get_mut(1) {
        *value = 20;
    }
    
    // Pop values
    while let Some(value) = vec.pop() {
        println!("Popped: {}", value);
    }
    
    // Test with strings
    let mut string_vec = CustomVec::new();
    string_vec.push(String::from("Hello"));
    string_vec.push(String::from("World"));
    
    // Get reference to string
    if let Some(s) = string_vec.get(0) {
        println!("First string: {}", s);
    }
    
    // Modify string through mutable reference
    if let Some(s) = string_vec.get_mut(1) {
        s.push_str("!");
    }
    
    // Pop strings
    while let Some(s) = string_vec.pop() {
        println!("Popped string: {}", s);
    }
} 