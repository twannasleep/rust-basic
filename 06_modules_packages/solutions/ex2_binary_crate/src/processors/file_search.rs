use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use anyhow::Result;
use regex::Regex;

pub type LineMatch = (usize, String);
pub type SearchResults = Vec<(PathBuf, Vec<LineMatch>)>;

pub fn search_file(path: &Path, pattern: &Regex) -> Result<Vec<LineMatch>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut matches = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if pattern.is_match(&line) {
            matches.push((line_num + 1, line));
        }
    }

    Ok(matches)
}

pub fn search_dir(path: &Path, pattern: &Regex, recursive: bool) -> Result<SearchResults> {
    let mut results = Vec::new();

    if recursive {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Ok(matches) = search_file(&path, pattern) {
                    if !matches.is_empty() {
                        results.push((path, matches));
                    }
                }
            } else if path.is_dir() {
                results.extend(search_dir(&path, pattern, recursive)?);
            }
        }
    } else {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Ok(matches) = search_file(&path, pattern) {
                    if !matches.is_empty() {
                        results.push((path, matches));
                    }
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_search_file() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "Line one\nLine two\nLine three")?;

        let pattern = Regex::new("two").unwrap();
        let matches = search_file(&file_path, &pattern)?;

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].0, 2); // Line number
        assert_eq!(matches[0].1, "Line two");

        Ok(())
    }

    #[test]
    fn test_search_dir() -> Result<()> {
        let dir = tempdir()?;
        
        // Create test files
        let file1_path = dir.path().join("test1.txt");
        let mut file1 = File::create(&file1_path)?;
        writeln!(file1, "File one line one\nFile one line two")?;

        let file2_path = dir.path().join("test2.txt");
        let mut file2 = File::create(&file2_path)?;
        writeln!(file2, "File two line one\nFile two line two")?;

        let pattern = Regex::new("two").unwrap();
        let results = search_dir(dir.path(), &pattern, false)?;

        assert_eq!(results.len(), 2);
        for (_, matches) in &results {
            assert_eq!(matches.len(), 1);
            assert!(matches[0].1.contains("two"));
        }

        Ok(())
    }
} 