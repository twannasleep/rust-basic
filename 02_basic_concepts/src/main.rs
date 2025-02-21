/// This is a documentation comment for the temperature conversion function
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Demonstrates different types of loops in Rust
fn demonstrate_loops() {
    // for loop with range
    println!("\nCounting with for loop:");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // while loop
    println!("\nCounting with while loop:");
    let mut count = 1;
    while count <= 5 {
        print!("{} ", count);
        count += 1;
    }
    println!();

    // loop with break
    println!("\nCounting with loop and break:");
    let mut counter = 1;
    loop {
        print!("{} ", counter);
        counter += 1;
        if counter > 5 {
            break;
        }
    }
    println!();
}

fn main() {
    // Variables and mutability
    let immutable_var = 42;
    let mut mutable_var = 42;
    mutable_var = 43;

    println!("Basic Variables:");
    println!("Immutable: {}", immutable_var);
    println!("Mutable: {}", mutable_var);

    // Data types
    // Integers
    let signed_int: i32 = -42;
    let unsigned_int: u32 = 42;

    // Floating point
    let float_32: f32 = 3.14;
    let float_64: f64 = 3.14159265359;

    // Boolean
    let is_rust_fun = true;

    // Character
    let rust_char = '🦀';

    println!("\nDifferent Data Types:");
    println!("Signed Integer: {}", signed_int);
    println!("Unsigned Integer: {}", unsigned_int);
    println!("32-bit Float: {}", float_32);
    println!("64-bit Float: {}", float_64);
    println!("Boolean: {}", is_rust_fun);
    println!("Character: {}", rust_char);

    // Compound types
    // Tuple
    let tup = (500, 6.4, "Rust");
    println!("\nTuple values: {} {} {}", tup.0, tup.1, tup.2);

    // Array
    let array = [1, 2, 3, 4, 5];
    println!("Array first element: {}", array[0]);

    // Function call
    let temp_f = 98.6;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("\nTemperature Conversion:");
    println!("{:.1}°F = {:.1}°C", temp_f, temp_c);

    // Control flow
    let number = 7;
    println!("\nControl Flow Example:");
    if number < 5 {
        println!("Number is less than 5");
    } else if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is 5");
    }

    // Demonstrate different types of loops
    demonstrate_loops();

    // Shadowing example
    let spaces = "   ";
    let spaces = spaces.len();
    println!("\nShadowing Example:");
    println!("Number of spaces: {}", spaces);
} 