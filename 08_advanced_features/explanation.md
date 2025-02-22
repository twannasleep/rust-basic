# Advanced Features in Rust

## Unsafe Rust

```rust
// Basic unsafe block
unsafe fn dangerous() {
    // Unsafe operations here
}

// Raw pointers
fn raw_pointers() {
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
    }
}

// FFI (Foreign Function Interface)
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn call_c_function() {
    let result = abs(-3);
    println!("Absolute value: {}", result);
}
```

## Advanced Traits

```rust
// Associated types
trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
    fn insert(&mut self, item: Self::Item);
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

// Trait objects
fn draw_all(shapes: &[Box<dyn Draw>]) {
    for shape in shapes {
        shape.draw();
    }
}
```

## Advanced Types

```rust
// Type aliases
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

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
```

## Macros

```rust
// Declarative macros
macro_rules! vec_strs {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

// Procedural macros
use proc_macro;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Implementation
}

// Attribute macros
#[route(GET, "/")]
fn index() {}

// Function-like macros
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

## Advanced Memory Management

```rust
// Custom allocators
#[global_allocator]
static ALLOCATOR: MyAllocator = MyAllocator;

// Memory mapping
use std::alloc::{alloc, dealloc, Layout};

unsafe fn custom_allocation() {
    let layout = Layout::new::<u32>();
    let ptr = alloc(layout);
    dealloc(ptr, layout);
}

// Reference counting
use std::rc::Rc;
use std::cell::RefCell;

struct SharedData {
    data: Rc<RefCell<Vec<i32>>>,
}
```

## Advanced Patterns

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
```

## Zero-Cost Abstractions

```rust
// Compile-time guarantees
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

## Advanced Error Handling

```rust
// Custom error types with source
#[derive(Debug)]
struct AppError {
    kind: ErrorKind,
    source: Option<Box<dyn Error>>,
}

// Error conversion
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: ErrorKind::Io,
            source: Some(Box::new(error)),
        }
    }
}

// Backtrace support
use std::backtrace::Backtrace;

struct ErrorWithBacktrace {
    message: String,
    backtrace: Backtrace,
}
```
