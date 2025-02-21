use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{Path, State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use tokio::sync::RwLock;
use task_common::{Task, CreateTaskRequest, UpdateTaskRequest, TaskError};
use std::collections::HashMap;
use tracing::{info, error};

type TaskStore = Arc<RwLock<HashMap<u64, Task>>>;
type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self.0.downcast_ref::<TaskError>() {
            Some(TaskError::NotFound(_)) => (StatusCode::NOT_FOUND, self.0.to_string()),
            Some(TaskError::InvalidData(_)) => (StatusCode::BAD_REQUEST, self.0.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        }.into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create shared state
    let store: TaskStore = Arc::new(RwLock::new(HashMap::new()));

    // Build router
    let app = Router::new()
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id", put(update_task))
        .route("/tasks/:id", delete(delete_task))
        .with_state(store);

    // Start server
    let addr = "127.0.0.1:3000";
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_tasks(
    State(store): State<TaskStore>,
) -> Result<Json<Vec<Task>>> {
    let tasks = store.read().await;
    Ok(Json(tasks.values().cloned().collect()))
}

async fn create_task(
    State(store): State<TaskStore>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<Task>> {
    let mut task = Task::new(req.title, req.description);
    let mut store = store.write().await;
    let id = (store.len() as u64) + 1;
    task.id = Some(id);
    store.insert(id, task.clone());
    info!("Created task with id: {}", id);
    Ok(Json(task))
}

async fn get_task(
    State(store): State<TaskStore>,
    Path(id): Path<u64>,
) -> Result<Json<Task>> {
    let store = store.read().await;
    let task = store.get(&id)
        .cloned()
        .ok_or_else(|| TaskError::NotFound(id))?;
    Ok(Json(task))
}

async fn update_task(
    State(store): State<TaskStore>,
    Path(id): Path<u64>,
    Json(update): Json<UpdateTaskRequest>,
) -> Result<Json<Task>> {
    let mut store = store.write().await;
    let task = store.get_mut(&id)
        .ok_or_else(|| TaskError::NotFound(id))?;
    task.update(update);
    info!("Updated task with id: {}", id);
    Ok(Json(task.clone()))
}

async fn delete_task(
    State(store): State<TaskStore>,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    let mut store = store.write().await;
    if store.remove(&id).is_some() {
        info!("Deleted task with id: {}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(TaskError::NotFound(id).into())
    }
} 