use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file '{}': {}", filename, e);
            return Ok(());
        }
    };

    let reader = BufReader::new(file);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                line_count += 1;
                word_count += line.split_whitespace().count();
                char_count += line.chars().count();
            }
            Err(e) => {
                println!("Error reading line: {}", e);
                return Ok(());
            }
        }
    }

    println!("File: {}", filename);
    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);

    Ok(())
} 