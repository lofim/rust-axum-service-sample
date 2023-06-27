use crate::error::HttpResult;
use crate::extractors::{JsonExtractor, Path};
use crate::model::{Todo, TodoInput};
use crate::use_cases::TodoInputPortArc;
use axum::{Extension, Json};
use hyper::StatusCode;
use serde_json::{json, Value};
use tracing::debug;

pub async fn healthz_handler() -> Json<Value> {
    debug!("Calling healthz handler...");
    Json(json!({
        "status": "ok"
    }))
}

pub async fn readyz_handler() -> Json<Value> {
    debug!("Calling readyz handler...");
    Json(json!({
        "status": "ok"
    }))
}

pub async fn list_todos_handler(
    Extension(todo_port): Extension<TodoInputPortArc>,
) -> HttpResult<Json<Vec<Todo>>> {
    debug!("Calling list_todo handler...");

    let todos = todo_port.list_todos().await?;

    Ok(Json(todos))
}

pub async fn get_todo_handler(
    Path(id): Path<u32>,
    Extension(todo_port): Extension<TodoInputPortArc>,
) -> HttpResult<Json<Todo>> {
    debug!("Calling get_todo handler...");

    let todo = todo_port.get_todo(id).await?;

    Ok(Json(todo))
}

pub async fn create_todo_handler(
    Extension(todo_port): Extension<TodoInputPortArc>,
    JsonExtractor(todo_create): JsonExtractor<TodoInput>,
) -> HttpResult<Json<Todo>> {
    debug!("Calling create_todo handler...");

    let todo = todo_port.create_todo(todo_create).await?;

    Ok(Json(todo))
}

pub async fn update_todo_handler(
    Path(id): Path<u32>,
    Extension(todo_port): Extension<TodoInputPortArc>,
    JsonExtractor(todo_update): JsonExtractor<TodoInput>,
) -> HttpResult<Json<Todo>> {
    debug!("Calling update_todo handler...");

    let todo = todo_port.update_todo(id, todo_update).await?;

    Ok(Json(todo))
}

pub async fn delete_todo_handler(
    Path(id): Path<u32>,
    Extension(todo_port): Extension<TodoInputPortArc>,
) -> HttpResult<StatusCode> {
    debug!("Calling create_todo handler...");

    todo_port.delete_todo(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
