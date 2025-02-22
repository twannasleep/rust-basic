pub mod convert;
pub mod search;
pub mod stats;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert files between different formats
    Convert {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Input format (json, csv, yaml)
        #[arg(short, long)]
        from_format: String,

        /// Output format (json, csv, yaml)
        #[arg(short, long)]
        to_format: String,
    },

    /// Search for patterns in files
    Search {
        /// Directory or file path to search
        #[arg(short, long)]
        path: PathBuf,

        /// Search pattern (regex supported)
        #[arg(short, long)]
        pattern: String,

        /// Search recursively in directories
        #[arg(short, long, default_value_t = false)]
        recursive: bool,

        /// Case sensitive search
        #[arg(short, long, default_value_t = true)]
        case_sensitive: bool,
    },

    /// Generate statistics about files
    Stats {
        /// Path to analyze
        #[arg(short, long)]
        path: PathBuf,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        output_format: String,
    },
} 