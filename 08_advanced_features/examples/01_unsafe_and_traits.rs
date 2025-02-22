// Example: Unsafe Rust and Advanced Traits
// This example demonstrates unsafe operations and advanced trait features

#![allow(dead_code)]

use std::fmt::Debug;
use std::ops::Add;

// =============== Unsafe Rust Examples ===============

// Example of unsafe function
unsafe fn dangerous_operation(ptr: *mut i32) {
    *ptr = 42;  // Directly dereferencing raw pointer
}

// Safe wrapper around unsafe code
fn safe_wrapper(value: &mut i32) {
    let ptr = value as *mut i32;
    unsafe {
        dangerous_operation(ptr);
    }
}

// Example of FFI (Foreign Function Interface)
extern "C" {
    fn abs(input: i32) -> i32;
}

// Safe wrapper for C function
fn safe_abs(input: i32) -> i32 {
    unsafe { abs(input) }
}

// =============== Advanced Traits Examples ===============

// Associated Types in Traits
trait Container {
    type Item;
    
    fn get(&self) -> Option<&Self::Item>;
    fn insert(&mut self, item: Self::Item);
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;
    
    fn get(&self) -> Option<&Self::Item> {
        self.items.last()
    }
    
    fn insert(&mut self, item: Self::Item) {
        self.items.push(item);
    }
}

// Default Type Parameters
#[derive(Debug, PartialEq)]
struct Complex<T = f64> {
    real: T,
    imag: T,
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// Supertraits
trait Drawable: Debug {
    fn draw(&self) -> String;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

// Trait Objects with Dynamic Dispatch
fn draw_shapes(shapes: &[Box<dyn Drawable>]) {
    for shape in shapes {
        println!("{}", shape.draw());
    }
}

// =============== Main Function ===============

fn main() {
    // Unsafe examples
    let mut value = 0;
    safe_wrapper(&mut value);
    println!("Value after unsafe operation: {}", value);
    
    let abs_result = safe_abs(-42);
    println!("Absolute value: {}", abs_result);
    
    // Container with associated type
    let mut stack = Stack { items: Vec::new() };
    stack.insert(42);
    println!("Stack top: {:?}", stack.get());
    
    // Default type parameters
    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 3.0, imag: 4.0 };
    let sum = c1 + c2;
    println!("Complex sum: {:?}", sum);
    
    // Trait objects and dynamic dispatch
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Circle { radius: 2.0 }),
    ];
    draw_shapes(&shapes);
}

// =============== Tests ===============

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_wrapper() {
        let mut value = 0;
        safe_wrapper(&mut value);
        assert_eq!(value, 42);
    }
    
    #[test]
    fn test_container() {
        let mut stack = Stack { items: Vec::new() };
        stack.insert(1);
        assert_eq!(stack.get(), Some(&1));
    }
    
    #[test]
    fn test_complex_add() {
        let c1 = Complex { real: 1.0, imag: 2.0 };
        let c2 = Complex { real: 3.0, imag: 4.0 };
        let sum = c1 + c2;
        assert_eq!(sum, Complex { real: 4.0, imag: 6.0 });
    }
    
    #[test]
    fn test_drawable() {
        let circle = Circle { radius: 1.0 };
        assert_eq!(circle.draw(), "Circle with radius 1");
    }
} 