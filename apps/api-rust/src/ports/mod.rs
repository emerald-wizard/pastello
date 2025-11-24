use crate::domain::game::Session;
use async_trait::async_trait;
use anyhow::Result;
use std::any::Any; // Needed for EventBus trait

#[async_trait]
pub trait Clock: Send + Sync {
    fn now_utc(&self) -> chrono::DateTime<chrono::Utc>;
}

pub trait EventBus: Send + Sync {
    fn publish(&self, _event: Box<dyn Any + Send>) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn get(&self, id: &str) -> Result<Option<Session>>;
    async fn save(&self, id: &str, session: Session) -> Result<()>;
}

pub trait IdGenerator: Send + Sync {
    fn new_id(&self) -> String;
}

#[async_trait]
pub trait Rng: Send + Sync {
    async fn rand_int(&self, min: i32, max: i32) -> i32;
}
