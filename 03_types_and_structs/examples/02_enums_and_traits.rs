// Example: Enums, Pattern Matching, and Traits
// This example demonstrates enum variants, pattern matching, and trait implementations

// Define an enum for different shapes
#[derive(Debug)]
enum Shape {
    Circle(f64),                    // radius
    Rectangle(f64, f64),           // width, height
    Triangle(f64, f64, f64),       // sides
}

// Define a trait for area calculation
trait Area {
    fn area(&self) -> f64;
    fn description(&self) -> String {
        String::from("This is a shape")  // Default implementation
    }
}

// Implement the Area trait for Shape
impl Area for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
    
    fn description(&self) -> String {
        match self {
            Shape::Circle(radius) => format!("Circle with radius {}", radius),
            Shape::Rectangle(w, h) => format!("Rectangle {}x{}", w, h),
            Shape::Triangle(a, b, c) => format!("Triangle with sides {}, {}, {}", a, b, c),
        }
    }
}

// Message enum demonstrating different variant types
#[derive(Debug)]
enum Message {
    Quit,                          // Unit variant
    Move { x: i32, y: i32 },      // Struct variant
    Write(String),                 // Tuple variant
    ChangeColor(u8, u8, u8),      // Tuple variant
}

impl Message {
    fn call(&self) -> String {
        match self {
            Message::Quit => String::from("Quitting..."),
            Message::Move { x, y } => format!("Moving to ({}, {})", x, y),
            Message::Write(text) => format!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => format!("Changing color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // Working with Shape enum and Area trait
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    
    for shape in &shapes {
        println!("Shape: {:?}", shape);
        println!("Description: {}", shape.description());
        println!("Area: {:.2}\n", shape.area());
    }
    
    // Working with Message enum
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 128, 0),
    ];
    
    for message in &messages {
        println!("Message: {:?}", message);
        println!("Action: {}\n", message.call());
    }
    
    // Option enum example
    let numbers = vec![1, 2, 3, 4, 5];
    let first: Option<&i32> = numbers.first();
    
    match first {
        Some(value) => println!("First value: {}", value),
        None => println!("Vector is empty"),
    }
    
    // if let syntax for simpler matching
    if let Some(value) = numbers.get(1) {
        println!("Second value: {}", value);
    }
    
    // Result enum example
    fn divide(x: f64, y: f64) -> Result<f64, String> {
        if y == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(x / y)
        }
    }
    
    // Handling Result
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shape_areas() {
        let circle = Shape::Circle(1.0);
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.0001);
        
        let rectangle = Shape::Rectangle(2.0, 3.0);
        assert_eq!(rectangle.area(), 6.0);
        
        let triangle = Shape::Triangle(3.0, 4.0, 5.0);
        assert!((triangle.area() - 6.0).abs() < 0.0001);
    }
    
    #[test]
    fn test_message_call() {
        let msg = Message::Write(String::from("test"));
        assert_eq!(msg.call(), "Writing: test");
        
        let msg = Message::Move { x: 1, y: 2 };
        assert_eq!(msg.call(), "Moving to (1, 2)");
    }
} 