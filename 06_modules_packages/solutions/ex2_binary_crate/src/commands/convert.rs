use std::path::PathBuf;
use anyhow::{Result, Context};
use crate::processors::data::{read_data, write_data, DataFormat};

pub fn execute(
    input: PathBuf,
    output: PathBuf,
    from_format: String,
    to_format: String,
) -> Result<()> {
    // Parse formats
    let from_format = DataFormat::from_str(&from_format)
        .with_context(|| format!("Invalid input format: {}", from_format))?;
    let to_format = DataFormat::from_str(&to_format)
        .with_context(|| format!("Invalid output format: {}", to_format))?;

    // Read input data
    let data = read_data(&input, from_format)
        .with_context(|| format!("Failed to read input file: {:?}", input))?;

    // Write output data
    write_data(&output, &data, to_format)
        .with_context(|| format!("Failed to write output file: {:?}", output))?;

    println!("Successfully converted {:?} to {:?}", input, output);
    Ok(())
} 