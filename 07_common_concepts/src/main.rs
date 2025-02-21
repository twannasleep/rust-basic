use common_concepts::{
    Point, Summary, NewsArticle, Tweet, Stack, Container,
    longest, divide, square_root, ImportantExcerpt,
};

fn demonstrate_generics() {
    println!("Generic Types Example:");
    println!("--------------------");
    
    // Using Point with integers
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let p3 = p1 + p2;
    println!("Integer points: {:?} + {:?} = {:?}", 
        Point::new(1, 2), Point::new(3, 4), p3);

    // Using Point with floating-point numbers
    let p4: Point<f64> = Point::new(1.5, 2.5);
    let p5: Point<f64> = Point::new(3.5, 4.5);
    let p6 = p4 + p5;
    println!("Float points: {:?} + {:?} = {:?}\n", p4, p5, p6);
}

fn demonstrate_traits() {
    println!("Traits Example:");
    println!("--------------");
    
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released!"),
        location: String::from("San Francisco"),
        author: String::from("Jane Doe"),
        content: String::from("Exciting new features..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Just announced: Rust 2.0!"),
        reply: false,
        retweet: false,
    };

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    println!("Default summary: {}\n", tweet.default_summary());
}

fn demonstrate_lifetimes() {
    println!("Lifetimes Example:");
    println!("-----------------");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt::new(first_sentence);

    println!("Excerpt: {}", excerpt.part);
    
    let s1 = String::from("short");
    let s2 = String::from("longer");
    let result = longest(&s1, &s2);
    println!("Longest string: {}\n", result);
}

fn demonstrate_containers() {
    println!("Containers Example:");
    println!("------------------");
    
    let mut stack = Stack::new();
    stack.insert(1);
    stack.insert(2);
    stack.insert(3);

    println!("Top of stack: {:?}", stack.get());
    println!();
}

fn demonstrate_error_handling() {
    println!("Error Handling Example:");
    println!("----------------------");
    
    // Division
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    // Square root
    match square_root(16.0) {
        Ok(result) => println!("√16 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match square_root(-1.0) {
        Ok(result) => println!("√-1 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
    println!("Common Programming Concepts in Rust");
    println!("==================================\n");

    demonstrate_generics();
    demonstrate_traits();
    demonstrate_lifetimes();
    demonstrate_containers();
    demonstrate_error_handling();
} 