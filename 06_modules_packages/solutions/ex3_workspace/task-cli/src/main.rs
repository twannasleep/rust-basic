use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use task_common::{Task, CreateTaskRequest, UpdateTaskRequest, TaskStatus};
use reqwest::Client;

const API_URL: &str = "http://localhost:3000";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all tasks
    List,
    
    /// Get a task by ID
    Get {
        /// Task ID
        #[arg(short, long)]
        id: u64,
    },
    
    /// Create a new task
    Create {
        /// Task title
        #[arg(short, long)]
        title: String,
        
        /// Task description
        #[arg(short, long)]
        description: String,
    },
    
    /// Update a task
    Update {
        /// Task ID
        #[arg(short, long)]
        id: u64,
        
        /// New title (optional)
        #[arg(short, long)]
        title: Option<String>,
        
        /// New description (optional)
        #[arg(short, long)]
        description: Option<String>,
        
        /// New status (optional: pending, in_progress, completed)
        #[arg(short, long)]
        status: Option<String>,
    },
    
    /// Delete a task
    Delete {
        /// Task ID
        #[arg(short, long)]
        id: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::List => {
            let tasks: Vec<Task> = client.get(&format!("{}/tasks", API_URL))
                .send()
                .await?
                .json()
                .await?;
            
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in tasks {
                    print_task(&task);
                }
            }
        }
        
        Commands::Get { id } => {
            let task: Task = client.get(&format!("{}/tasks/{}", API_URL, id))
                .send()
                .await?
                .json()
                .await?;
            print_task(&task);
        }
        
        Commands::Create { title, description } => {
            let request = CreateTaskRequest { title, description };
            let task: Task = client.post(&format!("{}/tasks", API_URL))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            println!("Task created successfully:");
            print_task(&task);
        }
        
        Commands::Update { id, title, description, status } => {
            let status = match status.as_deref() {
                Some("pending") => Some(TaskStatus::Pending),
                Some("in_progress") => Some(TaskStatus::InProgress),
                Some("completed") => Some(TaskStatus::Completed),
                Some(s) => anyhow::bail!("Invalid status: {}. Valid values are: pending, in_progress, completed", s),
                None => None,
            };
            
            let request = UpdateTaskRequest {
                title,
                description,
                status,
            };
            
            let task: Task = client.put(&format!("{}/tasks/{}", API_URL, id))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            println!("Task updated successfully:");
            print_task(&task);
        }
        
        Commands::Delete { id } => {
            client.delete(&format!("{}/tasks/{}", API_URL, id))
                .send()
                .await?;
            println!("Task deleted successfully.");
        }
    }

    Ok(())
}

fn print_task(task: &Task) {
    println!("Task {}:", task.id.unwrap_or(0));
    println!("  Title: {}", task.title);
    println!("  Description: {}", task.description);
    println!("  Status: {:?}", task.status);
    println!("  Created: {}", task.created_at);
    println!("  Updated: {}", task.updated_at);
    println!();
} 