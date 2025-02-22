// Example: HashMap Operations and Patterns
// This example demonstrates various ways to work with HashMaps in Rust

use std::collections::HashMap;

// Custom type for HashMap keys
#[derive(Hash, Eq, PartialEq, Debug)]
struct StudentId {
    year: u16,
    number: u32,
}

// Custom type for HashMap values
#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<u8>,
}

impl Student {
    fn new(name: &str) -> Self {
        Student {
            name: name.to_string(),
            grades: Vec::new(),
        }
    }
    
    fn add_grade(&mut self, grade: u8) {
        self.grades.push(grade);
    }
    
    fn average_grade(&self) -> Option<f64> {
        if self.grades.is_empty() {
            None
        } else {
            let sum: u32 = self.grades.iter().map(|&x| x as u32).sum();
            Some(sum as f64 / self.grades.len() as f64)
        }
    }
}

fn main() {
    // Creating HashMaps
    println!("Basic HashMap operations:");
    let mut scores = HashMap::new();
    
    // Inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("Scores: {:?}", scores);
    
    // Accessing values
    match scores.get("Blue") {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team not found"),
    }
    
    // Updating values
    println!("\nUpdating values:");
    scores.insert(String::from("Blue"), 25); // Overwriting
    *scores.entry(String::from("Yellow")).or_insert(50) += 10; // Update or insert
    println!("Updated scores: {:?}", scores);
    
    // Word frequency counter
    println!("\nWord frequency:");
    let text = "hello world hello rust world programming rust";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word count: {:?}", word_count);
    
    // Using custom types
    println!("\nCustom types in HashMap:");
    let mut students = HashMap::new();
    
    // Adding students
    let student1 = StudentId { year: 2023, number: 1 };
    let student2 = StudentId { year: 2023, number: 2 };
    
    students.insert(student1, Student::new("Alice"));
    students.insert(student2, Student::new("Bob"));
    
    // Updating student grades
    if let Some(student) = students.get_mut(&student1) {
        student.add_grade(85);
        student.add_grade(92);
        student.add_grade(88);
    }
    
    // Displaying student information
    for (id, student) in &students {
        println!("Student {:?}:", id);
        println!("  Name: {}", student.name);
        println!("  Grades: {:?}", student.grades);
        if let Some(avg) = student.average_grade() {
            println!("  Average: {:.2}", avg);
        }
    }
    
    // Advanced HashMap operations
    println!("\nAdvanced operations:");
    
    // Entry API
    let mut population = HashMap::new();
    population.entry("New York").or_insert(8_400_000);
    population.entry("Tokyo").or_insert(37_400_000);
    
    // Updating based on presence
    for city in &["New York", "London"] {
        population.entry(city)
            .and_modify(|pop| *pop += 100_000)
            .or_insert(0);
    }
    println!("Population: {:?}", population);
    
    // Removing entries
    population.remove("London");
    
    // Checking existence
    println!("Contains Tokyo? {}", population.contains_key("Tokyo"));
    
    // Merging HashMaps
    println!("\nMerging maps:");
    let mut map1 = HashMap::new();
    map1.insert(1, "one");
    map1.insert(2, "two");
    
    let mut map2 = HashMap::new();
    map2.insert(3, "three");
    map2.insert(4, "four");
    
    map1.extend(map2);
    println!("Merged map: {:?}", map1);
    
    // Compound keys
    println!("\nCompound keys:");
    let mut matrix = HashMap::new();
    matrix.insert((0, 0), "Top left");
    matrix.insert((0, 1), "Top right");
    matrix.insert((1, 0), "Bottom left");
    matrix.insert((1, 1), "Bottom right");
    
    for ((x, y), value) in &matrix {
        println!("Position ({}, {}): {}", x, y, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_student_grades() {
        let mut student = Student::new("Test Student");
        assert_eq!(student.average_grade(), None);
        
        student.add_grade(90);
        student.add_grade(80);
        assert_eq!(student.average_grade(), Some(85.0));
    }
    
    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1", 1);
        assert_eq!(map.get("key1"), Some(&1));
        assert_eq!(map.get("key2"), None);
    }
    
    #[test]
    fn test_entry_api() {
        let mut map = HashMap::new();
        map.entry("key").or_insert(1);
        *map.entry("key").or_insert(0) += 1;
        assert_eq!(map.get("key"), Some(&2));
    }
} 