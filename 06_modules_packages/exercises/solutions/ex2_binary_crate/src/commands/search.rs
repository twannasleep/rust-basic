use std::path::PathBuf;
use anyhow::{Result, Context};
use regex::RegexBuilder;
use crate::processors::file_search::{search_file, search_dir};

pub fn execute(
    path: PathBuf,
    pattern: String,
    recursive: bool,
    case_sensitive: bool,
) -> Result<()> {
    // Build regex pattern
    let regex = RegexBuilder::new(&pattern)
        .case_insensitive(!case_sensitive)
        .build()
        .with_context(|| format!("Invalid regex pattern: {}", pattern))?;

    // Perform search based on path type
    if path.is_file() {
        let matches = search_file(&path, &regex)
            .with_context(|| format!("Failed to search file: {:?}", path))?;
        
        if matches.is_empty() {
            println!("No matches found in {:?}", path);
        } else {
            println!("Matches in {:?}:", path);
            for (line_num, line) in matches {
                println!("{}:{}", line_num, line);
            }
        }
    } else if path.is_dir() {
        let results = search_dir(&path, &regex, recursive)
            .with_context(|| format!("Failed to search directory: {:?}", path))?;
        
        if results.is_empty() {
            println!("No matches found in {:?}", path);
        } else {
            for (file_path, matches) in results {
                if !matches.is_empty() {
                    println!("\nMatches in {:?}:", file_path);
                    for (line_num, line) in matches {
                        println!("{}:{}", line_num, line);
                    }
                }
            }
        }
    } else {
        println!("Path {:?} does not exist", path);
    }

    Ok(())
} 