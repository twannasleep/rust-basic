use std::io::{self, Write};
use rand::Rng;

fn main() -> io::Result<()> {
    println!("Welcome to the Number Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        print!("Please guess a number (1-100): ");
        io::stdout().flush()?;

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! You guessed it in {} attempts!", attempts);
                break;
            }
        }
    }

    Ok(())
} 