use axum::{
    extract,
    Extension,  
    http::StatusCode, 
    Json,
    response::IntoResponse
};
use crate::repository::Repository;
use crate::models::{Todo};
use crate::todo_repository::{TodoRepository};

pub async fn index(todo_repository: Extension<TodoRepository>) -> impl IntoResponse{
    let data: Vec<Todo> = todo_repository.index().await;

    (StatusCode::CREATED, Json(data))
}

pub async fn find(
    todo_repository: Extension<TodoRepository>, 
    extract::Path(id): extract::Path<u64>,
) -> impl IntoResponse {
    let data: Todo = todo_repository.find(id).await;

    (StatusCode::CREATED, Json(data))
}