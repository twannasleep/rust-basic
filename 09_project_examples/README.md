# Real-World Project Examples

## CLI Application

```rust
use clap::{App, Arg};
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("file-tool")
        .version("1.0")
        .author("Your Name")
        .about("File manipulation tool")
        .arg(Arg::with_name("input")
            .required(true)
            .help("Input file"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true))
        .get_matches();

    // Handle CLI arguments
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output")
        .unwrap_or("output.txt");

    process_file(input, output).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}
```

## Web Server

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

async fn get_user(user_id: web::Path<u32>) -> HttpResponse {
    // Database interaction
    let user = User {
        id: user_id.into_inner(),
        name: "John Doe".to_string(),
    };
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Database Interface

```rust
use tokio_postgres::{Client, NoTls};
use deadpool_postgres::{Config, Pool};

struct DatabaseConnection {
    pool: Pool,
}

impl DatabaseConnection {
    async fn new() -> Result<Self, Error> {
        let mut cfg = Config::new();
        cfg.host = Some("localhost".to_string());
        cfg.dbname = Some("mydb".to_string());
        
        let pool = cfg.create_pool(NoTls)?;
        
        Ok(DatabaseConnection { pool })
    }

    async fn get_user(&self, id: i32) -> Result<User, Error> {
        let client = self.pool.get().await?;
        let row = client
            .query_one("SELECT * FROM users WHERE id = $1", &[&id])
            .await?;
        
        Ok(User::from_row(row))
    }
}
```

## System Monitor

```rust
use sysinfo::{System, SystemExt};
use tokio::time::{self, Duration};

struct SystemMonitor {
    sys: System,
    update_interval: Duration,
}

impl SystemMonitor {
    fn new() -> Self {
        SystemMonitor {
            sys: System::new_all(),
            update_interval: Duration::from_secs(1),
        }
    }

    async fn monitor(&mut self) {
        let mut interval = time::interval(self.update_interval);
        
        loop {
            interval.tick().await;
            self.sys.refresh_all();
            
            println!("Memory: {}%", self.sys.used_memory() * 100 / self.sys.total_memory());
            println!("CPU: {}%", self.sys.global_cpu_info().cpu_usage());
        }
    }
}
```

## File Processing Pipeline

```rust
use tokio::fs;
use futures::stream::{self, StreamExt};

struct Pipeline {
    input_dir: String,
    output_dir: String,
    max_concurrent: usize,
}

impl Pipeline {
    async fn process_files(&self) -> Result<(), Error> {
        let mut entries = fs::read_dir(&self.input_dir).await?;
        let mut tasks = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let task = async move {
                let content = fs::read_to_string(entry.path()).await?;
                let processed = process_content(&content)?;
                let output_path = self.output_dir.join(entry.file_name());
                fs::write(output_path, processed).await?;
                Ok(())
            };
            tasks.push(task);
        }

        stream::iter(tasks)
            .buffer_unordered(self.max_concurrent)
            .collect::<Vec<_>>()
            .await;
            
        Ok(())
    }
}
```

## Configuration Management

```rust
use config::{Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Settings {
    debug: bool,
    server: ServerSettings,
    database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
struct ServerSettings {
    host: String,
    port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = Config::new();
        
        // Load default config
        s.merge(File::with_name("config/default"))?;
        
        // Load environment specific config
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        
        // Add environment variables
        s.merge(Environment::with_prefix("app"))?;
        
        s.try_into()
    }
}
```

## Testing Framework

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        Database {
            fn connect(&self) -> Result<(), Error>;
            fn query(&self, sql: &str) -> Result<Vec<Row>, Error>;
        }
    }

    #[test]
    async fn test_database_operations() {
        let mut mock_db = MockDatabase::new();
        mock_db.expect_connect()
            .times(1)
            .returning(|| Ok(()));
            
        mock_db.expect_query()
            .with(eq("SELECT * FROM users"))
            .times(1)
            .returning(|_| Ok(vec![Row::default()]));
            
        let service = Service::new(mock_db);
        let result = service.get_users().await;
        assert!(result.is_ok());
    }
}
```
