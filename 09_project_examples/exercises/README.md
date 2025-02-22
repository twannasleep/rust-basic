# Project Examples Exercises

## 🌟 Exercise 1: Task Manager CLI

Build a command-line task manager that:

```rust
// TODO: Implement a CLI task manager with:
// - Task CRUD operations
// - Priority levels
// - Due dates
// - Categories/tags
// - Data persistence

use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    due_date: Option<DateTime<Utc>>,
    tags: Vec<String>,
}

fn main() {
    // Implement CLI interface
    // Handle commands
    // Manage task data
}
```

**Skills practiced:**

- CLI argument parsing
- File I/O
- Data serialization
- Error handling
- User interaction

## 🌟🌟 Exercise 2: REST API Server

Create a RESTful API server for a blog:

```rust
// TODO: Implement a blog API with:
// - User authentication
// - CRUD for posts
// - Comments system
// - Category management
// - Search functionality

use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: u32,
    title: String,
    content: String,
    author: String,
    categories: Vec<String>,
}

// Implement endpoints:
async fn create_post(post: web::Json<BlogPost>) -> HttpResponse {
    // Handle post creation
}

async fn get_posts() -> HttpResponse {
    // Retrieve posts
}
```

**Skills practiced:**

- Web server setup
- Database integration
- Authentication
- JSON handling
- API design

## 🌟🌟 Exercise 3: Data Processing Pipeline

Implement a data processing system:

```rust
// TODO: Create a pipeline that:
// - Reads data from multiple sources
// - Processes in parallel
// - Applies transformations
// - Handles errors gracefully
// - Outputs results

use tokio;
use futures::stream::{self, StreamExt};

struct DataProcessor {
    input_sources: Vec<String>,
    batch_size: usize,
    workers: usize,
}

impl DataProcessor {
    async fn process_data(&self) -> Result<Vec<ProcessedData>, Error> {
        // Implement parallel processing
        // Handle batching
        // Manage resources
    }
}
```

**Skills practiced:**

- Async programming
- Parallel processing
- Resource management
- Error handling
- Performance optimization

## 🌟🌟🌟 Exercise 4: Distributed Cache System

Build a distributed caching system:

```rust
// TODO: Implement a distributed cache with:
// - Multiple node support
// - Data replication
// - Consistency management
// - Failure handling
// - Performance monitoring

use tokio::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct CacheNode {
    id: String,
    peers: Vec<PeerInfo>,
    data: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl CacheNode {
    async fn start(&self) -> Result<(), Error> {
        // Initialize node
        // Connect to peers
        // Handle requests
    }
}
```

**Skills practiced:**

- Distributed systems
- Network programming
- Concurrency
- State management
- System design

## 🌟🌟🌟 Exercise 5: Monitoring Dashboard

Create a system monitoring dashboard:

```rust
// TODO: Build a dashboard that:
// - Collects system metrics
// - Provides real-time updates
// - Generates alerts
// - Stores historical data
// - Visualizes statistics

use tokio;
use sysinfo::{System, SystemExt};
use plotters::prelude::*;

struct Dashboard {
    metrics_collector: MetricsCollector,
    alert_manager: AlertManager,
    storage: MetricsStorage,
    ui: WebInterface,
}

impl Dashboard {
    async fn run(&mut self) -> Result<(), Error> {
        // Collect metrics
        // Update UI
        // Handle alerts
    }
}
```

**Skills practiced:**

- Real-time processing
- Data visualization
- WebSocket handling
- Time series data
- UI integration

## Tips

1. Use proper project structure
2. Write comprehensive tests
3. Handle edge cases
4. Document API endpoints
5. Monitor performance

## Evaluation Criteria

- Code organization
- Error handling
- Performance
- Documentation
- User experience
