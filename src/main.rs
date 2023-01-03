use axum::{routing::get, routing::post, routing::put, routing::delete, Router, Extension};
use std::net::SocketAddr;
use dotenv::dotenv;

mod todo_controller;
mod todo_repository;
mod database;
mod models;
mod repository;

use crate::todo_controller::{index, find, create, update, destroy};
use crate::database::db_connect;
use crate::todo_repository::{TodoRepository};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db_connect().await.unwrap();

    
    let todo_repository = TodoRepository{
        db_connection: pool
    };


    let app = Router::new()
    .route("/todos", get(index))
    .route("/todos/:id", get(find))
    .route("/todos", post(create))
    .route("/todos/:id", put(update))
    .route("/todos/:id", delete(destroy))
    .layer(Extension(todo_repository));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}