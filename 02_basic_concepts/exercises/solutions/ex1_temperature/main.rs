use std::io::{self, Write};

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Temperature Converter");
    println!("--------------------");

    loop {
        let temp_str = get_user_input("Enter temperature (or 'q' to quit): ");
        
        if temp_str.to_lowercase() == "q" {
            println!("Goodbye!");
            break;
        }

        let temperature: f64 = match temp_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please enter a valid number!");
                continue;
            }
        };

        let unit = get_user_input("Enter unit (C/F): ").to_uppercase();

        match unit.as_str() {
            "C" => {
                let fahrenheit = celsius_to_fahrenheit(temperature);
                println!("{:.1}°C = {:.1}°F", temperature, fahrenheit);
            }
            "F" => {
                let celsius = fahrenheit_to_celsius(temperature);
                println!("{:.1}°F = {:.1}°C", temperature, celsius);
            }
            _ => {
                println!("Error: Please enter either 'C' or 'F'!");
                continue;
            }
        }
    }
} 