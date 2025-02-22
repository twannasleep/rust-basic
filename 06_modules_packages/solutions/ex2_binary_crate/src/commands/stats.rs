use std::path::PathBuf;
use anyhow::{Result, Context};
use serde_json;
use crate::processors::stats::{collect_stats, FileStats};

pub fn execute(path: PathBuf, output_format: String) -> Result<()> {
    // Collect statistics
    let stats = collect_stats(&path)
        .with_context(|| format!("Failed to collect stats for {:?}", path))?;

    // Output statistics based on format
    match output_format.to_lowercase().as_str() {
        "text" => output_text(&stats),
        "json" => output_json(&stats)?,
        _ => println!("Unsupported output format: {}. Using text format.", output_format),
    }

    Ok(())
}

fn output_text(stats: &FileStats) {
    println!("File Statistics:");
    println!("---------------");
    println!("Total files: {}", stats.total_files);
    println!("Total directories: {}", stats.total_dirs);
    println!("Total size: {} bytes", stats.total_size);
    
    println!("\nFile types:");
    for (ext, count) in &stats.file_types {
        println!("  {}: {}", ext, count);
    }

    println!("\nLargest files:");
    for (path, size) in stats.largest_files.iter().take(5) {
        println!("  {:?}: {} bytes", path, size);
    }

    println!("\nNewest files:");
    for (path, modified) in stats.newest_files.iter().take(5) {
        println!("  {:?}: {:?}", path, modified);
    }
}

fn output_json(stats: &FileStats) -> Result<()> {
    let json = serde_json::to_string_pretty(stats)
        .context("Failed to serialize stats to JSON")?;
    println!("{}", json);
    Ok(())
} 