// Example: Basic Ownership Concepts
// This example demonstrates fundamental ownership rules and behaviors

fn take_ownership(s: String) {
    println!("Taken: {}", s);
    // s is dropped here
}

fn make_copy(x: i32) {
    println!("Copied: {}", x);
    // x is dropped here, but original still exists because i32 implements Copy
}

fn give_ownership() -> String {
    let s = String::from("hello");
    s // ownership is moved to caller
}

fn take_and_give_back(s: String) -> String {
    s // ownership is moved back to caller
}

fn main() {
    // Stack-allocated types (implement Copy trait)
    let x = 5;
    make_copy(x);
    println!("x is still valid: {}", x);
    
    // Heap-allocated types (ownership rules apply)
    let s1 = String::from("hello");
    take_ownership(s1);
    // println!("s1 is invalid now: {}", s1); // This would not compile!
    
    // Moving ownership
    let s2 = String::from("world");
    let s3 = s2;
    // println!("s2 is moved: {}", s2); // This would not compile!
    println!("s3 owns the value: {}", s3);
    
    // Cloning (deep copy)
    let s4 = String::from("clone me");
    let s5 = s4.clone();
    println!("s4: {}, s5: {}", s4, s5); // Both valid!
    
    // Ownership transfer through functions
    let s6 = give_ownership();
    println!("Got ownership: {}", s6);
    
    let s7 = take_and_give_back(s6);
    println!("Got ownership back: {}", s7);
    
    // Multiple variables with Copy types
    let n1 = 42;
    let n2 = n1;
    println!("Both numbers valid - n1: {}, n2: {}", n1, n2);
    
    // Ownership and scope
    {
        let inner_string = String::from("I will be dropped");
        println!("Inside scope: {}", inner_string);
    } // inner_string is dropped here
    
    // Demonstrating Drop trait
    struct CustomDrop(String);
    
    impl Drop for CustomDrop {
        fn drop(&mut self) {
            println!("Dropping CustomDrop with value: {}", self.0);
        }
    }
    
    let custom = CustomDrop(String::from("watch me drop"));
    println!("Created CustomDrop");
} // custom is dropped here, triggering the Drop implementation 