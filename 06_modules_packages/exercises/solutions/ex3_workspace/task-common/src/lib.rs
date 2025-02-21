use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<u64>,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task not found with id: {0}")]
    NotFound(u64),
    #[error("Invalid task data: {0}")]
    InvalidData(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Server error: {0}")]
    Server(String),
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            title,
            description,
            status: TaskStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, update: UpdateTaskRequest) {
        if let Some(title) = update.title {
            self.title = title;
        }
        if let Some(description) = update.description {
            self.description = description;
        }
        if let Some(status) = update.status {
            self.status = status;
        }
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test task".to_string(), "Test description".to_string());
        assert_eq!(task.title, "Test task");
        assert_eq!(task.description, "Test description");
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.id.is_none());
    }

    #[test]
    fn test_task_update() {
        let mut task = Task::new("Test task".to_string(), "Test description".to_string());
        let update = UpdateTaskRequest {
            title: Some("Updated title".to_string()),
            description: None,
            status: Some(TaskStatus::InProgress),
        };
        task.update(update);
        assert_eq!(task.title, "Updated title");
        assert_eq!(task.description, "Test description");
        assert_eq!(task.status, TaskStatus::InProgress);
    }
} 