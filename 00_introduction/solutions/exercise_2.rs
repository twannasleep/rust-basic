fn main() {
    // Celsius to Fahrenheit
    let celsius = 25.0;
    let fahrenheit = celsius * 1.8 + 32.0;
    println!("{}°C = {}°F", celsius, fahrenheit);

    // Number swap
    let (mut a, mut b) = (10, 20);
    (a, b) = (b, a);
    println!("Swapped: a={}, b={}", a, b);

    // Circle area
    const PI: f64 = 3.14159;
    let radius = 5.0;
    println!("Area: {}", PI * radius.powi(2));
} 