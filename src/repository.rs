use async_trait::async_trait;

#[async_trait]
pub trait Repository<T> {
    async fn index(&self) -> Vec<T>;
    async fn find(&self, id: u64) -> T;
    async fn save(&self, description: String) -> u64;
    async fn update(&self, description: String, done: bool) -> ();
    async fn delete(&self, id: u64) -> ();
}