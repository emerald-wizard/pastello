use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::any::Any;
use tokio_stream::Stream;

#[async_trait]
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

#[async_trait]
pub trait Rng: Send + Sync {
    async fn rand_int(&self, min: i32, max: i32) -> i32;
}

#[async_trait]
pub trait IDGen: Send + Sync {
    async fn new_id(&self) -> String;
}

#[async_trait]
// --- FIX: Add pub ---
pub trait Repo<T: Clone + Send + Sync>: Send + Sync {
    async fn find(&self, id: &str) -> Option<T>;
    async fn save(&self, id: &str, item: T);
}

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, topic: &str, payload: Box<dyn Any + Send>) -> anyhow::Result<()>;
    async fn subscribe(
        &self,
        topic: &str,
    ) -> Box<dyn Stream<Item = Box<dyn Any + Send>> + Send + Unpin>;
}