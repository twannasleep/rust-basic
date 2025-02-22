// Solution: Task Management System
// This solution implements a task management system using vectors and custom types

use chrono::{DateTime, Utc};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone)]
pub struct Task {
    id: u32,
    title: String,
    description: String,
    status: TaskStatus,
    due_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        due_date: DateTime<Utc>,
    ) -> Self {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Todo,
            due_date,
            created_at: Utc::now(),
        }
    }
    
    pub fn mark_in_progress(&mut self) {
        self.status = TaskStatus::InProgress;
    }
    
    pub fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
    }
    
    pub fn is_overdue(&self) -> bool {
        self.status != TaskStatus::Done && self.due_date < Utc::now()
    }
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_task(
        &mut self,
        title: String,
        description: String,
        due_date: DateTime<Utc>,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        
        let task = Task::new(id, title, description, due_date);
        self.tasks.push(task);
        id
    }
    
    pub fn remove_task(&mut self, id: u32) -> Option<Task> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }
    
    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }
    
    pub fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }
    
    pub fn update_status(&mut self, id: u32, status: TaskStatus) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.status = status;
            true
        } else {
            false
        }
    }
    
    pub fn list_all(&self) -> &[Task] {
        &self.tasks
    }
    
    pub fn list_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.status == status)
            .collect()
    }
    
    pub fn list_overdue(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.is_overdue())
            .collect()
    }
    
    pub fn sort_by_due_date(&mut self) {
        self.tasks.sort_by(|a, b| a.due_date.cmp(&b.due_date));
    }
    
    pub fn sort_by_status_and_date(&mut self) {
        self.tasks.sort_by(|a, b| {
            let status_order = match (&a.status, &b.status) {
                (TaskStatus::Todo, TaskStatus::InProgress) => Ordering::Less,
                (TaskStatus::Todo, TaskStatus::Done) => Ordering::Less,
                (TaskStatus::InProgress, TaskStatus::Todo) => Ordering::Greater,
                (TaskStatus::InProgress, TaskStatus::Done) => Ordering::Less,
                (TaskStatus::Done, TaskStatus::Todo) => Ordering::Greater,
                (TaskStatus::Done, TaskStatus::InProgress) => Ordering::Greater,
                _ => Ordering::Equal,
            };
            
            status_order.then(a.due_date.cmp(&b.due_date))
        });
    }
    
    pub fn search(&self, query: &str) -> Vec<&Task> {
        let query = query.to_lowercase();
        self.tasks
            .iter()
            .filter(|task| {
                task.title.to_lowercase().contains(&query)
                    || task.description.to_lowercase().contains(&query)
            })
            .collect()
    }
}

fn main() {
    // Example usage
    let mut manager = TaskManager::new();
    
    // Add some tasks
    let task1_id = manager.add_task(
        "Complete project".to_string(),
        "Finish the Rust project".to_string(),
        Utc::now() + chrono::Duration::days(7),
    );
    
    let task2_id = manager.add_task(
        "Write documentation".to_string(),
        "Document the code".to_string(),
        Utc::now() + chrono::Duration::days(3),
    );
    
    let task3_id = manager.add_task(
        "Review code".to_string(),
        "Review pull requests".to_string(),
        Utc::now() + chrono::Duration::days(1),
    );
    
    // Update task statuses
    manager.update_status(task2_id, TaskStatus::InProgress);
    manager.update_status(task3_id, TaskStatus::Done);
    
    // Display tasks by status
    println!("Todo tasks:");
    for task in manager.list_by_status(TaskStatus::Todo) {
        println!("- {} (Due: {})", task.title, task.due_date);
    }
    
    println!("\nIn Progress tasks:");
    for task in manager.list_by_status(TaskStatus::InProgress) {
        println!("- {} (Due: {})", task.title, task.due_date);
    }
    
    println!("\nCompleted tasks:");
    for task in manager.list_by_status(TaskStatus::Done) {
        println!("- {} (Due: {})", task.title, task.due_date);
    }
    
    // Sort tasks by due date
    manager.sort_by_due_date();
    println!("\nAll tasks sorted by due date:");
    for task in manager.list_all() {
        println!(
            "- {} (Status: {:?}, Due: {})",
            task.title, task.status, task.due_date
        );
    }
    
    // Search for tasks
    println!("\nSearch results for 'code':");
    for task in manager.search("code") {
        println!("- {} (Status: {:?})", task.title, task.status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_task(manager: &mut TaskManager, days_from_now: i64) -> u32 {
        manager.add_task(
            "Test Task".to_string(),
            "Description".to_string(),
            Utc::now() + chrono::Duration::days(days_from_now),
        )
    }
    
    #[test]
    fn test_add_and_remove_task() {
        let mut manager = TaskManager::new();
        let task_id = create_test_task(&mut manager, 1);
        
        assert_eq!(manager.tasks.len(), 1);
        assert!(manager.remove_task(task_id).is_some());
        assert_eq!(manager.tasks.len(), 0);
    }
    
    #[test]
    fn test_update_status() {
        let mut manager = TaskManager::new();
        let task_id = create_test_task(&mut manager, 1);
        
        assert!(manager.update_status(task_id, TaskStatus::InProgress));
        assert_eq!(
            manager.get_task(task_id).unwrap().status,
            TaskStatus::InProgress
        );
    }
    
    #[test]
    fn test_list_by_status() {
        let mut manager = TaskManager::new();
        let task_id = create_test_task(&mut manager, 1);
        manager.update_status(task_id, TaskStatus::Done);
        
        assert_eq!(manager.list_by_status(TaskStatus::Done).len(), 1);
        assert_eq!(manager.list_by_status(TaskStatus::Todo).len(), 0);
    }
    
    #[test]
    fn test_sort_by_due_date() {
        let mut manager = TaskManager::new();
        let task1_id = create_test_task(&mut manager, 3);
        let task2_id = create_test_task(&mut manager, 1);
        
        manager.sort_by_due_date();
        let tasks = manager.list_all();
        assert_eq!(tasks[0].id, task2_id);
        assert_eq!(tasks[1].id, task1_id);
    }
    
    #[test]
    fn test_search() {
        let mut manager = TaskManager::new();
        manager.add_task(
            "Test Search".to_string(),
            "Find this".to_string(),
            Utc::now(),
        );
        
        assert_eq!(manager.search("search").len(), 1);
        assert_eq!(manager.search("nonexistent").len(), 0);
    }
} 