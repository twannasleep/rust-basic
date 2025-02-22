use std::f64::consts::PI;
use std::io::{self, Write};

// Error type for invalid dimensions
#[derive(Debug)]
enum ShapeError {
    NegativeDimension,
    ZeroDimension,
    InvalidScale,
}

// Shape types
#[derive(Debug)]
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}

// Circle struct
#[derive(Debug)]
struct Circle {
    radius: f64,
}

// Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Triangle struct
#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
    side1: f64,
    side2: f64,
}

// Implementation for Circle
impl Circle {
    fn new(radius: f64) -> Result<Circle, ShapeError> {
        if radius < 0.0 {
            Err(ShapeError::NegativeDimension)
        } else if radius == 0.0 {
            Err(ShapeError::ZeroDimension)
        } else {
            Ok(Circle { radius })
        }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        if factor <= 0.0 {
            Err(ShapeError::InvalidScale)
        } else {
            self.radius *= factor;
            Ok(())
        }
    }
}

// Implementation for Rectangle
impl Rectangle {
    fn new(width: f64, height: f64) -> Result<Rectangle, ShapeError> {
        if width < 0.0 || height < 0.0 {
            Err(ShapeError::NegativeDimension)
        } else if width == 0.0 || height == 0.0 {
            Err(ShapeError::ZeroDimension)
        } else {
            Ok(Rectangle { width, height })
        }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        if factor <= 0.0 {
            Err(ShapeError::InvalidScale)
        } else {
            self.width *= factor;
            self.height *= factor;
            Ok(())
        }
    }
}

// Implementation for Triangle
impl Triangle {
    fn new(base: f64, height: f64, side1: f64, side2: f64) -> Result<Triangle, ShapeError> {
        if base < 0.0 || height < 0.0 || side1 < 0.0 || side2 < 0.0 {
            Err(ShapeError::NegativeDimension)
        } else if base == 0.0 || height == 0.0 || side1 == 0.0 || side2 == 0.0 {
            Err(ShapeError::ZeroDimension)
        } else {
            Ok(Triangle {
                base,
                height,
                side1,
                side2,
            })
        }
    }

    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64 {
        self.base + self.side1 + self.side2
    }

    fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        if factor <= 0.0 {
            Err(ShapeError::InvalidScale)
        } else {
            self.base *= factor;
            self.height *= factor;
            self.side1 *= factor;
            self.side2 *= factor;
            Ok(())
        }
    }
}

// Implementation for Shape enum
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.area(),
            Shape::Rectangle(r) => r.area(),
            Shape::Triangle(t) => t.area(),
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.perimeter(),
            Shape::Rectangle(r) => r.perimeter(),
            Shape::Triangle(t) => t.perimeter(),
        }
    }

    fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        match self {
            Shape::Circle(c) => c.scale(factor),
            Shape::Rectangle(r) => r.scale(factor),
            Shape::Triangle(t) => t.scale(factor),
        }
    }
}

fn get_user_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn main() {
    println!("Shape Calculator");
    println!("---------------");

    loop {
        println!("\nSelect shape:");
        println!("1. Circle");
        println!("2. Rectangle");
        println!("3. Triangle");
        println!("4. Quit");

        let choice = get_user_input("\nEnter choice (1-4): ") as i32;

        let mut shape = match choice {
            1 => {
                let radius = get_user_input("Enter radius: ");
                match Circle::new(radius) {
                    Ok(circle) => Shape::Circle(circle),
                    Err(e) => {
                        println!("Error creating circle: {:?}", e);
                        continue;
                    }
                }
            }
            2 => {
                let width = get_user_input("Enter width: ");
                let height = get_user_input("Enter height: ");
                match Rectangle::new(width, height) {
                    Ok(rectangle) => Shape::Rectangle(rectangle),
                    Err(e) => {
                        println!("Error creating rectangle: {:?}", e);
                        continue;
                    }
                }
            }
            3 => {
                let base = get_user_input("Enter base: ");
                let height = get_user_input("Enter height: ");
                let side1 = get_user_input("Enter side 1: ");
                let side2 = get_user_input("Enter side 2: ");
                match Triangle::new(base, height, side1, side2) {
                    Ok(triangle) => Shape::Triangle(triangle),
                    Err(e) => {
                        println!("Error creating triangle: {:?}", e);
                        continue;
                    }
                }
            }
            4 => break,
            _ => {
                println!("Invalid choice!");
                continue;
            }
        };

        println!("\nShape created successfully!");
        println!("Area: {:.2}", shape.area());
        println!("Perimeter: {:.2}", shape.perimeter());

        let scale_factor = get_user_input("\nEnter scale factor (or 0 to skip): ");
        if scale_factor > 0.0 {
            match shape.scale(scale_factor) {
                Ok(_) => {
                    println!("\nShape scaled by {}", scale_factor);
                    println!("New area: {:.2}", shape.area());
                    println!("New perimeter: {:.2}", shape.perimeter());
                }
                Err(e) => println!("Error scaling shape: {:?}", e),
            }
        }
    }
} 