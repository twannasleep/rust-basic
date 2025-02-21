// Function that takes ownership and returns ownership
fn process_string(mut s: String) -> String {
    s.push_str(" processed");
    s // Return ownership of the modified string
}

// Function that borrows a string reference
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function that borrows a mutable reference
fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

// Function demonstrating string slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // Ownership examples
    println!("Ownership Examples:");
    println!("-----------------");
    
    // Basic ownership
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // This would cause an error
    println!("s2: {}", s2);

    // Cloning
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    // Stack-only data (Copy)
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // Ownership with functions
    let s5 = String::from("hello");
    let s6 = process_string(s5);
    println!("Processed string: {}", s6);

    println!("\nBorrowing Examples:");
    println!("-----------------");

    // Immutable borrowing
    let s7 = String::from("hello");
    let len = calculate_length(&s7);
    println!("Length of '{}' is {}", s7, len);

    // Mutable borrowing
    let mut s8 = String::from("hello");
    append_exclamation(&mut s8);
    println!("After mutation: {}", s8);

    // Multiple immutable references
    let s9 = String::from("hello");
    let r1 = &s9;
    let r2 = &s9;
    println!("Multiple references: {} {}", r1, r2);

    println!("\nSlice Examples:");
    println!("--------------");

    // String slices
    let s10 = String::from("hello world");
    let hello = &s10[0..5];
    let world = &s10[6..];
    println!("Slices: '{}' '{}'", hello, world);

    // Finding first word
    let s11 = String::from("Hello World");
    let word = first_word(&s11);
    println!("First word: {}", word);

    // Array slices
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("Array slice: {:?}", slice);

    println!("\nScope and Dropping:");
    println!("-----------------");
    {
        let s12 = String::from("hello");
        println!("s12 is valid here: {}", s12);
    } // s12 goes out of scope and is dropped

    // Demonstrating ownership rules with vectors
    println!("\nOwnership with Vectors:");
    println!("--------------------");
    let mut vec = Vec::new();
    vec.push(String::from("first"));
    vec.push(String::from("second"));

    // Accessing elements
    let first = &vec[0]; // Immutable borrow
    println!("First element: {}", first);

    // After immutable borrow ends, we can do mutable borrow
    vec.push(String::from("third")); // Mutable borrow
    println!("Vector: {:?}", vec);
} 