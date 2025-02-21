use std::fmt::{Display, Debug};
use std::ops::Add;

// Generic Types
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Generic function with trait bounds
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Traits
pub trait Summary {
    fn summarize(&self) -> String;
    
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Function that uses trait bounds
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display format: {}", item);
}

// Lifetimes
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        ImportantExcerpt { part }
    }

    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Associated Types
pub trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
    fn insert(&mut self, item: Self::Item);
}

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn get(&self) -> Option<&Self::Item> {
        self.items.last()
    }

    fn insert(&mut self, item: Self::Item) {
        self.items.push(item);
    }
}

// Example of a custom error type
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

pub fn divide(x: i32, y: i32) -> Result<i32, MathError> {
    if y == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

pub fn square_root(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = p1 + p2;
        assert_eq!(p3.x, 4);
        assert_eq!(p3.y, 6);
    }

    #[test]
    fn test_largest() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(largest(&numbers), &5);

        let chars = vec!['a', 'b', 'c'];
        assert_eq!(largest(&chars), &'c');
    }

    #[test]
    fn test_summary() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        assert_eq!(
            tweet.summarize(),
            "horse_ebooks: of course, as you probably already know, people"
        );
    }

    #[test]
    fn test_longest() {
        let string1 = String::from("short");
        let string2 = String::from("longer");
        assert_eq!(longest(&string1, &string2), "longer");
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.insert(1);
        stack.insert(2);
        assert_eq!(stack.get(), Some(&2));
    }

    #[test]
    fn test_divide() {
        assert!(divide(10, 2).is_ok());
        assert!(divide(10, 0).is_err());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_panic() {
        assert_eq!(1, 2);
    }
} 