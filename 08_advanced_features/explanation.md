# 🚀 Advanced Features in Rust

## 📋 Overview

Rust provides powerful advanced features that enable low-level control, high-level abstractions, and safe systems programming.

## ⚠️ Unsafe Rust

### When to Use Unsafe

Unsafe Rust is necessary for:

- Dereferencing raw pointers
- Calling unsafe functions
- Implementing unsafe traits
- Accessing or modifying mutable static variables
- Interfacing with C code (FFI)

```rust
// Basic unsafe block
unsafe fn dangerous() {
    // Unsafe operations here
}

// Raw pointers
fn raw_pointers() {
    let mut num = 5;
    
    let r1 = &num as *const i32;    // Immutable raw pointer
    let r2 = &mut num as *mut i32;  // Mutable raw pointer
    
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("r2 is: {}", *r2);
    }
}

// FFI (Foreign Function Interface)
#[link(name = "c")]
extern "C" {
    fn abs(input: i32) -> i32;
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

unsafe fn call_c_functions() {
    let result = abs(-42);
    println!("Absolute value: {}", result);
    
    let ptr = malloc(100);
    // Use ptr...
    free(ptr);
}
```

## 🎨 Advanced Traits

### Associated Types and Generic Parameters

```rust
// Associated types
trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
    fn insert(&mut self, item: Self::Item);
}

impl Container for Vec<String> {
    type Item = String;
    
    fn get(&self) -> Option<&Self::Item> {
        self.first()
    }
    
    fn insert(&mut self, item: Self::Item) {
        self.push(item)
    }
}

// Default type parameters
trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Supertraits
trait Display: Debug {
    fn format(&self) -> String;
}

// Trait objects and dynamic dispatch
fn draw_all(shapes: &[Box<dyn Draw>]) {
    for shape in shapes {
        shape.draw();
    }
}
```

### Advanced Trait Features

```rust
// Marker traits
trait Marker {}

// Auto traits
unsafe auto trait Send {}

// Conditional trait implementations
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

// Associated constants
trait Number {
    const ZERO: Self;
    const ONE: Self;
}

// Trait aliases
trait MyError = Error + Send + Sync;
```

## 🔧 Advanced Types

### Type System Features

```rust
// Type aliases
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, MyError>;

// Never type
fn never_returns() -> ! {
    panic!("This function never returns");
}

// Sized trait
fn generic<T: Sized>(t: T) {
    // ...
}

// Dynamically sized types
fn print_str(s: &str) {  // str is DST
    println!("{}", s);
}

// Zero-sized types
struct Unit;
struct PhantomData<T>;

// Custom smart pointers
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

## 🎭 Macros

### Declarative Macros

```rust
// Simple macro
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// Macro with parameters
macro_rules! vec_strs {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

// Pattern matching in macros
macro_rules! math {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
}
```

### Procedural Macros

```rust
use proc_macro;

// Derive macro
#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Implementation
}

// Attribute macro
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Implementation
}

// Function-like macro
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Implementation
}
```

## 🧮 Advanced Memory Management

### Custom Allocators

```rust
use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static ALLOCATOR: MyAllocator = MyAllocator;

// Memory mapping
unsafe fn custom_allocation() {
    let layout = Layout::new::<u32>();
    let ptr = alloc(layout);
    // Use ptr...
    dealloc(ptr, layout);
}
```

### Reference Counting and Interior Mutability

```rust
use std::rc::Rc;
use std::cell::{RefCell, Cell};

struct SharedData {
    data: Rc<RefCell<Vec<i32>>>,
    counter: Cell<usize>,
}

impl SharedData {
    fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(Vec::new())),
            counter: Cell::new(0),
        }
    }
    
    fn add(&self, value: i32) {
        self.data.borrow_mut().push(value);
        self.counter.set(self.counter.get() + 1);
    }
}
```

## 🎯 Advanced Patterns

### Pattern Matching

```rust
// Pattern matching with guards
match value {
    Some(x) if x < 0 => println!("Negative"),
    Some(x) if x > 0 => println!("Positive"),
    _ => println!("Zero or None"),
}

// Range patterns
match x {
    1..=5 => println!("One through five"),
    _ => println!("Other"),
}

// Binding with @
match value {
    x @ 1..=5 => println!("Got: {}", x),
    _ => println!("Other"),
}

// Struct patterns
match point {
    Point { x: 0, y } => println!("On y axis: {}", y),
    Point { x, y: 0 } => println!("On x axis: {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

## ⚡ Zero-Cost Abstractions

### Compile-Time Features

```rust
// Const functions
const fn compute_at_compile_time(x: u32) -> u32 {
    x * 2
}

// SIMD operations
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// Custom iterators
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}
```

## 🔍 Best Practices

### 1. Using Unsafe Code

```rust
// Safe wrapper around unsafe code
pub struct SafeWrapper {
    ptr: *mut u8,
    len: usize,
}

impl SafeWrapper {
    pub fn new(size: usize) -> Self {
        let layout = Layout::array::<u8>(size).unwrap();
        let ptr = unsafe { alloc(layout) };
        Self { ptr, len: size }
    }
}

impl Drop for SafeWrapper {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr, Layout::array::<u8>(self.len).unwrap());
        }
    }
}
```

### 2. Error Handling

```rust
#[derive(Debug)]
pub enum AdvancedError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl std::error::Error for AdvancedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Parse(e) => Some(e),
            Self::Custom(_) => None,
        }
    }
}
```

### 3. Performance Optimization

```rust
// Use const generics
struct Array<T, const N: usize> {
    data: [T; N],
}

// Leverage type system for compile-time checks
struct NotSend;
struct ThreadSafe;

struct Connection<State> {
    state: std::marker::PhantomData<State>,
}

impl Connection<NotSend> {
    fn make_thread_safe(self) -> Connection<ThreadSafe> {
        Connection { state: std::marker::PhantomData }
    }
}
```

Remember: With great power comes great responsibility - use advanced features judiciously! 🚀
