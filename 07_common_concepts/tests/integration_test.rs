use common_concepts::{
    Point, Summary, NewsArticle, Tweet, Stack, Container,
    longest, divide, MathError,
};

#[test]
fn test_point_operations() {
    let p1: Point<i32> = Point::new(1, 2);
    let p2: Point<i32> = Point::new(3, 4);
    let p3 = p1 + p2;
    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);
}

#[test]
fn test_summary_implementations() {
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released"),
        location: String::from("San Francisco"),
        author: String::from("Jane Doe"),
        content: String::from("Exciting new features..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Big news coming!"),
        reply: false,
        retweet: false,
    };

    assert!(article.summarize().contains("Rust 2.0 Released"));
    assert!(tweet.summarize().contains("rustlang"));
}

#[test]
fn test_lifetime_functions() {
    let string1 = String::from("short");
    let string2 = String::from("longer");
    let result = longest(string1.as_str(), string2.as_str());
    assert_eq!(result, "longer");
}

#[test]
fn test_container_implementation() {
    let mut stack = Stack::new();
    stack.insert(42);
    assert_eq!(stack.get(), Some(&42));
}

#[test]
fn test_error_handling() {
    match divide(10, 0) {
        Ok(_) => panic!("Expected an error"),
        Err(e) => match e {
            MathError::DivisionByZero => (),
            _ => panic!("Wrong error type"),
        },
    }
}

// Test multiple types with generic Point
#[test]
fn test_generic_point() {
    let p1: Point<f64> = Point::new(1.5, 2.5);
    let p2: Point<f64> = Point::new(3.5, 4.5);
    let p3 = p1 + p2;
    assert_eq!(p3.x, 5.0);
    assert_eq!(p3.y, 7.0);
} 