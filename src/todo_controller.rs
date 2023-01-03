use axum::{
    extract,
    Extension,  
    http::StatusCode, 
    Json,
    response::IntoResponse
};
use crate::repository::Repository;
use crate::models::{Todo, CreateTodo, UpdateTodo};
use crate::todo_repository::{TodoRepository};



pub async fn index(todo_repository: Extension<TodoRepository>) -> impl IntoResponse{
    let data: Vec<Todo> = todo_repository.index().await;

    (StatusCode::OK, Json(data))
}

pub async fn find(
    todo_repository: Extension<TodoRepository>, 
    extract::Path(id): extract::Path<u64>,
) -> impl IntoResponse {
    let data: Todo = todo_repository.find(id).await;

    (StatusCode::OK, Json(data))
}

pub async fn create(
    todo_repository: Extension<TodoRepository>, 
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let created_id: u64 = todo_repository.create(payload).await;

    (StatusCode::CREATED, Json(created_id))
}

pub async fn update(
    todo_repository: Extension<TodoRepository>, 
    extract::Path(id): extract::Path<u64>,
    Json(payload): Json<UpdateTodo>,
) -> impl IntoResponse {
    todo_repository.update(id, payload).await;

    StatusCode::OK
}

pub async fn destroy(
    todo_repository: Extension<TodoRepository>, 
    extract::Path(id): extract::Path<u64>,
) -> impl IntoResponse {
    todo_repository.delete(id).await;

    StatusCode::OK
}