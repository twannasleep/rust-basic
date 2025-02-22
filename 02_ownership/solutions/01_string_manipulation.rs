// Solution: String Manipulation with Ownership
// This solution demonstrates various string operations while respecting ownership rules

fn to_uppercase(s: String) -> String {
    // Takes ownership of the string and returns a new uppercase version
    s.to_uppercase()
}

fn count_vowels(s: &str) -> usize {
    // Borrows the string immutably to count vowels
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

fn concatenate(s1: String, s2: String) -> String {
    // Takes ownership of both strings and returns a new concatenated string
    let mut result = s1;
    result.push_str(&s2);
    result
}

// Demonstrates proper cleanup with Drop trait
struct StringWrapper {
    content: String,
}

impl Drop for StringWrapper {
    fn drop(&mut self) {
        println!("Cleaning up StringWrapper containing: {}", self.content);
    }
}

fn main() {
    // Demonstrate ownership with to_uppercase
    let original = String::from("hello world");
    println!("Original string: {}", original);
    
    let uppercase = to_uppercase(original);
    println!("Uppercase version: {}", uppercase);
    // println!("Original is invalid now: {}", original); // This would not compile!
    
    // Demonstrate borrowing with count_vowels
    let text = String::from("rust programming");
    let vowel_count = count_vowels(&text);
    println!("Text '{}' has {} vowels", text, vowel_count);
    println!("Can still use text: {}", text); // Still valid because we only borrowed it
    
    // Demonstrate concatenation and ownership
    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust!");
    let combined = concatenate(s1, s2);
    println!("Combined string: {}", combined);
    // println!("s1 and s2 are invalid: {} {}", s1, s2); // This would not compile!
    
    // Demonstrate scoped cleanup
    {
        let wrapper = StringWrapper {
            content: String::from("temporary string"),
        };
        println!("Created wrapper with: {}", wrapper.content);
    } // wrapper is dropped here, cleanup message is printed
    
    // Demonstrate Clone when we need to keep the original
    let original = String::from("clone me");
    let cloned = original.clone();
    println!("Original: {}, Clone: {}", original, cloned); // Both are valid
    
    // Demonstrate String slices
    let text = String::from("slice example");
    let slice = &text[0..5];
    println!("Full text: {}, Slice: {}", text, slice);
    
    // Demonstrate building strings
    let mut builder = String::with_capacity(100);
    builder.push_str("Building ");
    builder.push_str("a ");
    builder.push_str("string");
    println!("Built string: {}", builder);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_uppercase() {
        let input = String::from("hello");
        assert_eq!(to_uppercase(input), "HELLO");
    }
    
    #[test]
    fn test_count_vowels() {
        let text = "hello world";
        assert_eq!(count_vowels(text), 3);
    }
    
    #[test]
    fn test_concatenate() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("World!");
        assert_eq!(concatenate(s1, s2), "Hello, World!");
    }
    
    #[test]
    fn test_empty_strings() {
        let empty = String::new();
        assert_eq!(count_vowels(&empty), 0);
        assert_eq!(to_uppercase(empty), "");
    }
} 