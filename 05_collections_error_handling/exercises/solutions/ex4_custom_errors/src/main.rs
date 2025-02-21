use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use url::Url;

// Custom error types for different components
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] io::Error),
    
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid value for {field}: {value}")]
    InvalidValue {
        field: String,
        value: String,
    },
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    
    #[error("API response error: {status} - {message}")]
    ResponseError {
        status: u16,
        message: String,
    },
    
    #[error("Rate limit exceeded. Try again in {0} seconds")]
    RateLimitExceeded(u64),
}

#[derive(Error, Debug)]
pub enum DataProcessError {
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    
    #[error("Value out of range: {value} (allowed range: {min}-{max})")]
    OutOfRange {
        value: i64,
        min: i64,
        max: i64,
    },
    
    #[error("Duplicate entry found: {0}")]
    DuplicateEntry(String),
}

// Application error that combines all error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("API error: {0}")]
    Api(#[from] ApiError),
    
    #[error("Data processing error: {0}")]
    DataProcess(#[from] DataProcessError),
    
    #[error("Unexpected error: {0}")]
    Other(String),
}

// Configuration structure
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_url: String,
    api_key: String,
    max_retries: u32,
    timeout_seconds: u64,
}

impl Config {
    fn load(path: PathBuf) -> Result<Self, ConfigError> {
        // Read config file
        let content = fs::read_to_string(path)?;
        
        // Parse JSON
        let config: Config = serde_json::from_str(&content)?;
        
        // Validate fields
        if config.api_key.is_empty() {
            return Err(ConfigError::MissingField("api_key".to_string()));
        }
        
        if config.max_retries == 0 {
            return Err(ConfigError::InvalidValue {
                field: "max_retries".to_string(),
                value: "0".to_string(),
            });
        }
        
        Ok(config)
    }
}

// API client
struct ApiClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    api_key: String,
}

impl ApiClient {
    fn new(config: &Config) -> Result<Self, ApiError> {
        let base_url = Url::parse(&config.api_url)?;
        
        Ok(ApiClient {
            client: reqwest::blocking::Client::new(),
            base_url,
            api_key: config.api_key.clone(),
        })
    }

    fn make_request(&self, endpoint: &str) -> Result<String, ApiError> {
        let url = self.base_url.join(endpoint)?;
        
        let response = self.client
            .get(url)
            .header("Authorization", &self.api_key)
            .send()?;
            
        match response.status().as_u16() {
            200 => Ok(response.text()?),
            429 => Err(ApiError::RateLimitExceeded(60)),
            status => Err(ApiError::ResponseError {
                status,
                message: response.text()?,
            }),
        }
    }
}

// Data processor
struct DataProcessor {
    min_value: i64,
    max_value: i64,
    processed_items: Vec<String>,
}

impl DataProcessor {
    fn new(min: i64, max: i64) -> Self {
        DataProcessor {
            min_value: min,
            max_value: max,
            processed_items: Vec::new(),
        }
    }

    fn process_data(&mut self, data: &str) -> Result<(), DataProcessError> {
        // Check format
        if !data.chars().all(|c| c.is_digit(10) || c == ',') {
            return Err(DataProcessError::InvalidFormat(
                "Data must contain only numbers and commas".to_string()
            ));
        }

        // Process each value
        for item in data.split(',') {
            let value = item.parse::<i64>().map_err(|_| {
                DataProcessError::InvalidFormat(format!("Invalid number: {}", item))
            })?;

            // Check range
            if value < self.min_value || value > self.max_value {
                return Err(DataProcessError::OutOfRange {
                    value,
                    min: self.min_value,
                    max: self.max_value,
                });
            }

            // Check duplicates
            let item_str = item.to_string();
            if self.processed_items.contains(&item_str) {
                return Err(DataProcessError::DuplicateEntry(item_str));
            }

            self.processed_items.push(item_str);
        }

        Ok(())
    }
}

// Application that uses all components
struct Application {
    config: Config,
    api_client: ApiClient,
    processor: DataProcessor,
}

impl Application {
    fn new(config_path: PathBuf) -> Result<Self, AppError> {
        // Load configuration
        let config = Config::load(config_path)?;
        
        // Initialize API client
        let api_client = ApiClient::new(&config)?;
        
        // Create processor
        let processor = DataProcessor::new(-1000, 1000);
        
        Ok(Application {
            config,
            api_client,
            processor,
        })
    }

    fn run(&mut self) -> Result<(), AppError> {
        println!("Fetching data from API...");
        let data = self.api_client.make_request("data")?;
        
        println!("Processing data...");
        self.processor.process_data(&data)?;
        
        println!("Successfully processed {} items", self.processor.processed_items.len());
        Ok(())
    }
}

fn main() {
    println!("Custom Error Handling Demo");
    println!("-----------------------\n");

    // Example usage with error handling
    let config_path = PathBuf::from("config.json");
    
    // Create example config file
    let config = Config {
        api_url: "https://api.example.com".to_string(),
        api_key: "secret_key".to_string(),
        max_retries: 3,
        timeout_seconds: 30,
    };
    
    // Write config to file
    if let Err(e) = fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap()) {
        println!("Failed to write config file: {}", e);
        return;
    }

    // Try to run application
    match Application::new(config_path.clone()) {
        Ok(mut app) => {
            println!("Application initialized successfully!");
            match app.run() {
                Ok(()) => println!("Application completed successfully!"),
                Err(e) => println!("Application error: {}", e),
            }
        }
        Err(e) => println!("Failed to initialize application: {}", e),
    }

    // Clean up
    let _ = fs::remove_file(config_path);
}
