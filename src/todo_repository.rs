use sqlx::MySqlPool;
use async_trait::async_trait;

use crate::repository::Repository;
use crate::models::{Todo};

#[derive(Clone)]
pub struct TodoRepository {
    pub db_connection: MySqlPool,
}

#[async_trait]
impl Repository<Todo> for TodoRepository {
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

    async fn save(&self, description: String) -> u64{
        sqlx::query_as!(Todo,
            "INSERT INTO todos (description) VALUES (?)",
            description
        ).execute(&self.db_connection)
        .await
        .unwrap().
        last_insert_id()
    }

    async fn update(&self, description: String, done: bool){
        sqlx::query_as!(Todo, 
            "UPDATE todos SET description = ? WHERE done = ?",
            description, 
            done
        ).fetch_one(&self.db_connection).await.unwrap();    
    }
    
    async fn delete(&self, id: u64) {
       sqlx::query_as!(Todo, 
        "DELETE FROM todos WHERE id = ?", id
       ).fetch_one(&self.db_connection).await.unwrap();        
    }
}