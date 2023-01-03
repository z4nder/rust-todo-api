use sqlx::MySqlPool;
use async_trait::async_trait;

use crate::repository::Repository;
use crate::models::{Todo, CreateTodo, UpdateTodo};

#[derive(Clone)]
pub struct TodoRepository {
    pub db_connection: MySqlPool,
}

#[async_trait]
impl Repository<Todo, CreateTodo, UpdateTodo> for TodoRepository {
    async fn index(&self) -> Vec<Todo> {
        sqlx::query_as!(Todo,
            "SELECT * FROM todos"
        ).fetch_all(&self.db_connection).await.unwrap()    
    }

    async fn find(&self, id: u64) -> Todo {
        sqlx::query_as!(Todo,
            "SELECT * FROM todos WHERE id = ?"
        , id)
        .fetch_one(&self.db_connection).await.unwrap() 
    }

    async fn create(&self, payload: CreateTodo) -> u64{
        sqlx::query_as!(Todo,
            "INSERT INTO todos (description) VALUES (?)",
            payload.description
        ).execute(&self.db_connection)
        .await
        .unwrap().
        last_insert_id()
    }

    async fn update(&self, id: u64, payload: UpdateTodo){
        sqlx::query!( 
            "UPDATE todos SET description = ?, done = ? WHERE id = ?",
            payload.description, 
            payload.done,
            id
        ).execute(&self.db_connection).await.unwrap();
    }
    
    async fn delete(&self, id: u64) {
       sqlx::query_as!(Todo, 
        "DELETE FROM todos WHERE id = ?", id
       ).execute(&self.db_connection).await.unwrap();        
    }
}