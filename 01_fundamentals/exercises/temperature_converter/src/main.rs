use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit: ");

    let mut f_input = String::new();

    io::stdin()
        .read_line(&mut f_input)
        .expect("Failed to read line");
    
    // convert to float
    let fahrenheit: f64 = f_input.trim().parse().unwrap();
    
    if fahrenheit < -459.67 {
        println!("Temperature below absolute zero!");
        return;
    }

    let celsius = f_to_c(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);  
}

fn f_to_c (f: f64) -> f64 { 
    (f - 32.0) * (5.0 / 9.0)
}

fn c_to_f (c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}
