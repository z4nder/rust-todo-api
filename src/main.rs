use axum::{routing::get, Router, Extension};
use std::net::SocketAddr;
use dotenv::dotenv;

mod todo_controller;
mod todo_repository;
mod database;
mod models;
mod repository;

use crate::todo_controller::{index, find};
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
    .route("/", get(index))
    .route("/:id", get(find))
    .layer(Extension(todo_repository));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}