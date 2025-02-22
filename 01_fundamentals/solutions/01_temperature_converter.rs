// Solution: Temperature Converter
// This program implements temperature conversion between Fahrenheit and Celsius

use std::io;

// Convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

// Parse temperature input with error handling
fn parse_temperature(input: &str) -> Result<f64, String> {
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| String::from("Invalid temperature value"))
}

fn main() {
    println!("Temperature Converter");
    println!("--------------------");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    println!("Choose conversion (1/2): ");
    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");
    
    let choice = choice.trim();
    
    println!("Enter temperature: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature");
    
    match parse_temperature(&temp) {
        Ok(temperature) => {
            match choice {
                "1" => {
                    let celsius = fahrenheit_to_celsius(temperature);
                    println!(
                        "{:.1}°F is equal to {:.1}°C",
                        temperature, celsius
                    );
                }
                "2" => {
                    let fahrenheit = celsius_to_fahrenheit(temperature);
                    println!(
                        "{:.1}°C is equal to {:.1}°F",
                        temperature, fahrenheit
                    );
                }
                _ => println!("Invalid choice. Please select 1 or 2."),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Additional examples
    println!("\nSome example conversions:");
    let test_temps = vec![32.0, 0.0, 100.0, 98.6];
    
    for temp in test_temps {
        if temp == 32.0 || temp == 98.6 {
            // These are Fahrenheit temperatures
            let celsius = fahrenheit_to_celsius(temp);
            println!("{:.1}°F = {:.1}°C", temp, celsius);
        } else {
            // These are Celsius temperatures
            let fahrenheit = celsius_to_fahrenheit(temp);
            println!("{:.1}°C = {:.1}°F", temp, fahrenheit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_freezing_point() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }
    
    #[test]
    fn test_boiling_point() {
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }
    
    #[test]
    fn test_body_temperature() {
        assert!((fahrenheit_to_celsius(98.6) - 37.0).abs() < 0.1);
        assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 0.1);
    }
} 