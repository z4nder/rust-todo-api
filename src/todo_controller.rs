use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPool};
use axum::{
    extract::Extension,  
    http::StatusCode, 
    Json,
    response::IntoResponse
};

use crate::models::{Todo};

pub async fn index(Extension(pool): Extension<MySqlPool>){
    let recs: Vec<Todo> = sqlx::query_as!(Todo,
        "SELECT * FROM todos"
    ).fetch_all(&pool).await.unwrap();

    recs
    // (StatusCode::CREATED, Json(recs))
}

pub async fn make_memes() -> &'static str {
    "Hello, meme boy!" 
}