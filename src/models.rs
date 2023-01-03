use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub done: i8,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub description: String,
    pub done: bool
}