use async_trait::async_trait;

#[async_trait]
pub trait Repository<T, U, C> {
    async fn index(&self) -> Vec<T>;
    async fn find(&self, id: u64) -> T;
    async fn create(&self, payload: U) -> u64;
    async fn update(&self, id: u64, payload: C) -> ();
    async fn delete(&self, id: u64) -> ();
}