// Solution 2: Temperature Converter
// This program converts between Celsius and Fahrenheit

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}

fn main() {
    // Test some temperature conversions
    let test_temps = vec![0.0, 16.0, 25.0, 100.0];
    
    println!("Temperature Conversion Table:");
    println!("----------------------------");
    println!("Celsius    | Fahrenheit");
    println!("----------------------------");
    
    for temp in test_temps {
        let fahrenheit = celsius_to_fahrenheit(temp);
        println!("{:8.1}°C | {:8.1}°F", temp, fahrenheit);
    }
    
    println!("\nReverse Conversion:");
    println!("----------------------------");
    println!("Fahrenheit | Celsius");
    println!("----------------------------");
    
    let test_temps_f = vec![32.0, 68.0, 98.6, 212.0];
    for temp in test_temps_f {
        let celsius = fahrenheit_to_celsius(temp);
        println!("{:8.1}°F | {:8.1}°C", temp, celsius);
    }
} 