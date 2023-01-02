use axum::{routing::get, Router, Extension};
use std::net::SocketAddr;
use dotenv::dotenv;

mod todo_controller;
mod database;
mod models;

use crate::todo_controller::{index, make_memes};
use crate::database::db_connect;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // build our application with a route

    let pool = db_connect().await.unwrap();


    let app = Router::new()
    .route("/", get(index))
    .route("/memes", get(make_memes))
    .layer(Extension(pool));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}