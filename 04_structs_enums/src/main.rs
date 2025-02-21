#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Example of a tuple struct
struct Color(i32, i32, i32);

// Example of a unit struct
struct AlwaysEqual;

// Example of an enum with different variant types
#[derive(Debug)]
enum Shape {
    Circle(f64),                // radius
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => (base * height) / 2.0,
        }
    }
}

// Example of a more complex enum
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to position: ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(color) => {
                println!("Changing color to: ({}, {}, {})", color.0, color.1, color.2)
            }
        }
    }
}

// Example of using Option
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    // Demonstrating structs
    println!("Struct Examples:");
    println!("--------------");

    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("Rectangle: {:?}", rect1);
    println!("Area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Demonstrating tuple struct
    let black = Color(0, 0, 0);
    println!("\nColor: ({}, {}, {})", black.0, black.1, black.2);

    // Demonstrating unit struct
    let _subject = AlwaysEqual;
    println!("Unit struct AlwaysEqual exists!");

    // Demonstrating enums
    println!("\nEnum Examples:");
    println!("-------------");

    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let triangle = Shape::Triangle {
        base: 10.0,
        height: 15.0,
    };

    println!("Circle area: {:.2}", circle.area());
    println!("Rectangle area: {:.2}", rectangle.area());
    println!("Triangle area: {:.2}", triangle.area());

    // Demonstrating Message enum
    println!("\nMessage Examples:");
    println!("----------------");

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(Color(255, 0, 0)),
    ];

    for message in messages {
        message.call();
    }

    // Demonstrating Option
    println!("\nOption Examples:");
    println!("---------------");

    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Some(value) => println!("Result of 10/2: {}", value),
        None => println!("Division failed"),
    }

    // Using if let for cleaner Option handling
    if let Some(value) = result2 {
        println!("Result of 10/0: {}", value);
    } else {
        println!("Division by zero is undefined");
    }
} 