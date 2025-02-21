use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::collections::HashMap;
use std::time::SystemTime;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub file_types: HashMap<String, usize>,
    pub largest_files: Vec<(PathBuf, u64)>,
    pub newest_files: Vec<(PathBuf, SystemTime)>,
}

impl FileStats {
    fn new() -> Self {
        FileStats {
            total_files: 0,
            total_dirs: 0,
            total_size: 0,
            file_types: HashMap::new(),
            largest_files: Vec::new(),
            newest_files: Vec::new(),
        }
    }

    fn add_file(&mut self, path: &Path, metadata: &fs::Metadata) {
        self.total_files += 1;
        self.total_size += metadata.len();

        // Track file extension
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            *self.file_types.entry(ext).or_insert(0) += 1;
        } else {
            *self.file_types.entry("no_extension".to_string()).or_insert(0) += 1;
        }

        // Track largest files
        self.largest_files.push((path.to_path_buf(), metadata.len()));
        self.largest_files.sort_by(|a, b| b.1.cmp(&a.1));
        if self.largest_files.len() > 10 {
            self.largest_files.truncate(10);
        }

        // Track newest files
        if let Ok(modified) = metadata.modified() {
            self.newest_files.push((path.to_path_buf(), modified));
            self.newest_files.sort_by(|a, b| b.1.cmp(&a.1));
            if self.newest_files.len() > 10 {
                self.newest_files.truncate(10);
            }
        }
    }
}

pub fn collect_stats(path: &Path) -> Result<FileStats> {
    let mut stats = FileStats::new();
    collect_stats_recursive(path, &mut stats)?;
    Ok(stats)
}

fn collect_stats_recursive(path: &Path, stats: &mut FileStats) -> Result<()> {
    if path.is_file() {
        let metadata = fs::metadata(path)?;
        stats.add_file(path, &metadata);
    } else if path.is_dir() {
        stats.total_dirs += 1;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            collect_stats_recursive(&entry.path(), stats)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_collect_stats() -> Result<()> {
        let dir = tempdir()?;
        
        // Create test files
        let file1_path = dir.path().join("test1.txt");
        let mut file1 = File::create(&file1_path)?;
        writeln!(file1, "File one content")?;

        let file2_path = dir.path().join("test2.json");
        let mut file2 = File::create(&file2_path)?;
        writeln!(file2, "{{\"key\": \"value\"}}")?;

        // Create subdirectory with file
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir)?;
        let file3_path = subdir.join("test3.txt");
        let mut file3 = File::create(&file3_path)?;
        writeln!(file3, "File three content")?;

        let stats = collect_stats(dir.path())?;

        assert_eq!(stats.total_files, 3);
        assert_eq!(stats.total_dirs, 1);
        assert!(stats.total_size > 0);
        assert_eq!(stats.file_types.get("txt").unwrap(), &2);
        assert_eq!(stats.file_types.get("json").unwrap(), &1);

        Ok(())
    }
} 