use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

// Custom string type to demonstrate ownership concepts
struct AnalyzableText {
    content: String,
    words: Vec<String>,
    word_frequency: HashMap<String, usize>,
}

impl AnalyzableText {
    // Takes ownership of the string
    fn new(text: String) -> Self {
        let words: Vec<String> = text
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();

        let mut word_frequency = HashMap::new();
        for word in &words {
            *word_frequency.entry(word.clone()).or_insert(0) += 1;
        }

        AnalyzableText {
            content: text,
            words,
            word_frequency,
        }
    }

    // Returns a reference to the frequency map
    fn get_word_frequency(&self) -> &HashMap<String, usize> {
        &self.word_frequency
    }

    // Returns the number of unique words
    fn unique_word_count(&self) -> usize {
        self.word_frequency.len()
    }

    // Returns a slice of the content between start and end indices
    fn get_slice(&self, start: usize, end: usize) -> &str {
        &self.content[start..end.min(self.content.len())]
    }

    // Returns sentences as vector of string slices
    fn get_sentences(&self) -> Vec<&str> {
        self.content
            .split(&['.', '!', '?'])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    }

    // Returns total word count
    fn total_word_count(&self) -> usize {
        self.words.len()
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut content = String::new();
    
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }
    
    Ok(content)
}

fn main() -> io::Result<()> {
    println!("Text Analyzer");
    println!("-------------");

    loop {
        println!("\nOptions:");
        println!("1. Analyze file");
        println!("2. Analyze input text");
        println!("3. Quit");

        let choice = get_user_input("\nSelect option (1-3): ");

        if choice == "3" {
            println!("Goodbye!");
            break;
        }

        let text = match choice.as_str() {
            "1" => {
                let filename = get_user_input("Enter filename: ");
                match read_file(Path::new(&filename)) {
                    Ok(content) => content,
                    Err(e) => {
                        println!("Error reading file: {}", e);
                        continue;
                    }
                }
            }
            "2" => {
                println!("Enter text (empty line to finish):");
                let mut text = String::new();
                loop {
                    let line = get_user_input("");
                    if line.is_empty() {
                        break;
                    }
                    text.push_str(&line);
                    text.push('\n');
                }
                text
            }
            _ => {
                println!("Invalid option! Please select 1-3.");
                continue;
            }
        };

        let analyzed_text = AnalyzableText::new(text);

        println!("\nAnalysis Results:");
        println!("-----------------");
        println!("Total words: {}", analyzed_text.total_word_count());
        println!("Unique words: {}", analyzed_text.unique_word_count());
        
        println!("\nWord frequency (top 5):");
        let mut freq: Vec<_> = analyzed_text.get_word_frequency().iter().collect();
        freq.sort_by(|a, b| b.1.cmp(a.1));
        for (word, count) in freq.iter().take(5) {
            println!("{}: {}", word, count);
        }

        println!("\nSentences:");
        for (i, sentence) in analyzed_text.get_sentences().iter().enumerate() {
            println!("{}. {}", i + 1, sentence);
        }
    }

    Ok(())
} 