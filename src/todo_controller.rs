use axum::{
    extract::Extension,  
    http::StatusCode, 
    Json,
    response::IntoResponse
};
use crate::repository::Repository;
use crate::models::{Todo};
use crate::todo_repository::{TodoRepository};

pub async fn index(Extension(todo_repository): Extension<TodoRepository>) -> impl IntoResponse{
    let data: Vec<Todo> = todo_repository.index().await;

    (StatusCode::CREATED, Json(data))
}

pub async fn make_memes() -> &'static str {
    "Hello, meme boy!" 
}