use std::io::{self, Write};

fn print_pyramid(height: u32) {
    for i in 0..height {
        // Print spaces
        for _ in 0..(height - i - 1) {
            print!(" ");
        }
        // Print stars
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn print_diamond(height: u32) {
    // Upper half (including middle)
    for i in 0..height {
        for _ in 0..(height - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
    // Lower half
    for i in (0..height-1).rev() {
        for _ in 0..(height - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn print_hollow_square(size: u32) {
    for i in 0..size {
        for j in 0..size {
            if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Pattern Printer");
    println!("--------------");

    loop {
        println!("\nAvailable patterns:");
        println!("1. Pyramid");
        println!("2. Diamond");
        println!("3. Hollow Square");
        println!("4. Quit");

        let choice = get_user_input("\nSelect pattern (1-4): ");
        
        if choice == "4" {
            println!("Goodbye!");
            break;
        }

        let height_str = get_user_input("Enter height/size (1-20): ");
        let height: u32 = match height_str.parse() {
            Ok(num) if num > 0 && num <= 20 => num,
            Ok(_) => {
                println!("Please enter a number between 1 and 20!");
                continue;
            }
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("\nHere's your pattern:\n");
        match choice.as_str() {
            "1" => print_pyramid(height),
            "2" => print_diamond(height),
            "3" => print_hollow_square(height),
            _ => println!("Invalid choice! Please select 1-4."),
        }
    }
} 