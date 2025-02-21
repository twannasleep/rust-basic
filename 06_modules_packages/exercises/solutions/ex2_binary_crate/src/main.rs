use clap::Parser;
use anyhow::Result;

mod commands;
mod processors;
mod utils;

use commands::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert {
            input,
            output,
            from_format,
            to_format,
        } => {
            commands::convert::execute(input, output, from_format, to_format)?;
        }
        Commands::Search {
            path,
            pattern,
            recursive,
            case_sensitive,
        } => {
            commands::search::execute(path, pattern, recursive, case_sensitive)?;
        }
        Commands::Stats {
            path,
            output_format,
        } => {
            commands::stats::execute(path, output_format)?;
        }
    }

    Ok(())
} 