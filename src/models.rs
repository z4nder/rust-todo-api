use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub done: i8,
}