use sqlx::{Pool, mysql::MySqlPool, MySql, Error};

pub async fn db_connect() -> Result<Pool<MySql>, Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."); 
 
    MySqlPool::connect(&database_url).await
}