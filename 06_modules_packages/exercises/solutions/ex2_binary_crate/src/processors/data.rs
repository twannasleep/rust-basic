use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use anyhow::{Result, Context};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
}

#[derive(Debug, Clone, Copy)]
pub enum DataFormat {
    Json,
    Csv,
}

impl DataFormat {
    pub fn from_str(s: &str) -> Result<Self, DataError> {
        match s.to_lowercase().as_str() {
            "json" => Ok(DataFormat::Json),
            "csv" => Ok(DataFormat::Csv),
            _ => Err(DataError::UnsupportedFormat(s.to_string())),
        }
    }
}

pub fn read_data(path: &Path, format: DataFormat) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match format {
        DataFormat::Json => {
            serde_json::from_reader(reader).context("Failed to parse JSON")
        }
        DataFormat::Csv => {
            let mut csv_reader = csv::Reader::from_reader(reader);
            let headers = csv_reader.headers()?.clone();
            let records: Result<Vec<_>, _> = csv_reader.records().collect();
            let records = records?;

            // Convert CSV to JSON-compatible format
            let mut array = Vec::with_capacity(records.len());
            for record in records {
                let mut map = serde_json::Map::new();
                for (header, field) in headers.iter().zip(record.iter()) {
                    map.insert(header.to_string(), Value::String(field.to_string()));
                }
                array.push(Value::Object(map));
            }
            Ok(Value::Array(array))
        }
    }
}

pub fn write_data(path: &Path, data: &Value, format: DataFormat) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    match format {
        DataFormat::Json => {
            serde_json::to_writer_pretty(writer, data)
                .context("Failed to write JSON")
        }
        DataFormat::Csv => {
            let mut csv_writer = csv::Writer::from_writer(writer);

            match data {
                Value::Array(array) if !array.is_empty() => {
                    // Extract headers from the first object
                    if let Some(Value::Object(first)) = array.first() {
                        // Write headers
                        let headers: Vec<_> = first.keys().collect();
                        csv_writer.write_record(&headers)?;

                        // Write records
                        for value in array {
                            if let Value::Object(map) = value {
                                let record: Vec<_> = headers
                                    .iter()
                                    .map(|&h| map.get(h).unwrap_or(&Value::Null).to_string())
                                    .collect();
                                csv_writer.write_record(&record)?;
                            }
                        }
                    }
                }
                _ => return Err(DataError::UnsupportedFormat(
                    "Data must be an array of objects for CSV conversion".to_string()
                ).into()),
            }
            csv_writer.flush()?;
            Ok(())
        }
    }
} 