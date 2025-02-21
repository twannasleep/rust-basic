use std::io::{self, Write};

// Takes ownership of the string and returns a new reversed string
fn reverse_string(s: String) -> String {
    // Convert string to chars, collect to vec for easier manipulation
    let chars: Vec<char> = s.chars().collect();
    let mut reversed = Vec::with_capacity(chars.len());
    
    // Reverse the characters
    for i in (0..chars.len()).rev() {
        reversed.push(chars[i]);
    }
    
    // Convert back to string
    reversed.into_iter().collect()
}

// Borrows the string immutably to count words
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

// Takes a mutable reference to modify the string in place
fn replace_word(s: &mut String, target: &str, replacement: &str) {
    // Create a new string with replacements
    let replaced = s.replace(target, replacement);
    // Update the original string
    *s = replaced;
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("String Manipulator");
    println!("-----------------");

    loop {
        println!("\nOptions:");
        println!("1. Reverse string");
        println!("2. Count words");
        println!("3. Replace word");
        println!("4. Quit");

        let choice = get_user_input("\nSelect option (1-4): ");

        if choice == "4" {
            println!("Goodbye!");
            break;
        }

        let input = get_user_input("Enter a string: ");
        
        match choice.as_str() {
            "1" => {
                // input is moved into reverse_string
                let reversed = reverse_string(input);
                println!("Reversed: {}", reversed);
            }
            "2" => {
                // input is borrowed immutably
                let word_count = count_words(&input);
                println!("Word count: {}", word_count);
                // input can still be used here because it was only borrowed
                println!("Original string: {}", input);
            }
            "3" => {
                let target = get_user_input("Enter word to replace: ");
                let replacement = get_user_input("Enter replacement word: ");
                
                // Create a mutable copy of input for replacement
                let mut input_copy = input.clone();
                // Borrow input_copy mutably for replacement
                replace_word(&mut input_copy, &target, &replacement);
                println!("Result: {}", input_copy);
            }
            _ => println!("Invalid option! Please select 1-4."),
        }
    }
} 