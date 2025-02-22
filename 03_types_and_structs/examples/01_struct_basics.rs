// Example: Struct Basics and Implementation
// This example demonstrates different types of structs and their usage

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
    
    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (not a method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Tuple struct
#[derive(Debug)]
struct Color(u8, u8, u8);

// Unit struct
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    // Creating instances
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 20);
    let rect3 = Rectangle::square(25);
    
    // Using methods
    println!("Rectangle 1: {:?}", rect1);
    println!("Area of rectangle 1: {}", rect1.area());
    println!("Can rectangle 1 hold rectangle 2? {}", rect1.can_hold(&rect2));
    println!("Square: {:?}", rect3);
    
    // Tuple struct usage
    let red = Color(255, 0, 0);
    println!("Red color: {:?}", red);
    println!("Red value: {}", red.0);
    
    // Unit struct usage
    let _subject = AlwaysEqual;
    
    // Struct update syntax
    let rect4 = Rectangle {
        width: rect1.width,
        ..rect2
    };
    println!("Rectangle 4: {:?}", rect4);
    
    // Destructuring
    let Rectangle { width, height } = rect1;
    println!("Destructured width: {}, height: {}", width, height);
    
    // Demonstrating ownership with structs
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        active: bool,
    }
    
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("user123"),
        active: true,
    };
    
    // Using struct update syntax with owned fields
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // username is moved here
    };
    
    println!("User 2: {:?}", user2);
    // println!("User 1's username: {}", user1.username); // This would not compile!
    println!("User 1's active status: {}", user1.active); // This works because bool implements Copy
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(30, 50);
        assert_eq!(rect.area(), 1500);
    }
    
    #[test]
    fn test_can_hold() {
        let larger = Rectangle::new(30, 50);
        let smaller = Rectangle::new(10, 20);
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn test_square() {
        let square = Rectangle::square(20);
        assert_eq!(square.width, square.height);
        assert_eq!(square.width, 20);
    }
} 