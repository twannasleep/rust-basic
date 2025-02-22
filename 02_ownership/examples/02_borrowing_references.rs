// Example: Borrowing and References
// This example demonstrates how to use references and the borrowing rules

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_string(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    // Basic references
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);
    
    // Mutable references
    let mut s2 = String::from("hello");
    modify_string(&mut s2);
    println!("Modified string: {}", s2);
    
    // Multiple immutable references
    let s3 = String::from("multiple");
    let r1 = &s3;
    let r2 = &s3;
    println!("Multiple references: {} and {}", r1, r2);
    
    // Reference scope demonstration
    let mut s4 = String::from("hello");
    {
        let r3 = &mut s4;
        r3.push_str(" world");
    } // r3 goes out of scope here
    
    // Now we can borrow s4 again
    let r4 = &s4;
    println!("After inner scope: {}", r4);
    
    // Reference rules demonstration
    let mut s5 = String::from("hello");
    let r5 = &s5; // immutable borrow
    let r6 = &s5; // multiple immutable borrows are ok
    println!("{} and {}", r5, r6);
    // r5 and r6 are no longer used after this point
    
    let r7 = &mut s5; // mutable borrow is now ok
    r7.push_str(" world");
    println!("After mutable borrow: {}", r7);
    
    // Dangling reference prevention
    // let reference_to_nothing = dangle(); // This would not compile!
    
    // Slice references
    let s6 = String::from("hello world");
    let word = first_word(&s6);
    println!("First word: {}", word);
}

// This function prevents dangling references
// fn dangle() -> &String { // This would not compile!
//     let s = String::from("hello");
//     &s
// }

// Using string slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Demonstrating reference counting (Rc)
use std::rc::Rc;

fn demonstrate_rc() {
    let s = Rc::new(String::from("shared data"));
    
    let r1 = Rc::clone(&s);
    let r2 = Rc::clone(&s);
    
    println!("Reference count: {}", Rc::strong_count(&s));
    println!("Shared data: {}, {}, {}", s, r1, r2);
} 