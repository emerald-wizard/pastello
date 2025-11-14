use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::any::Any;
use crate::domain::game::{GameSessionID, Session};

/// Replaces internal/ports/repo.go
#[async_trait]
pub trait GameSessionRepo: Send + Sync {
    async fn save(
        &self,
        session: &Session,
        snapshot: Option<Box<dyn Any + Send>>,
    ) -> Result<()>;
    
    async fn load(
        &self,
        id: &GameSessionID,
    ) -> Result<Option<(Session, Option<Box<dyn Any + Send>>)>>;
}

/// Replaces internal/ports/clock.go
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

/// Replaces internal/ports/eventbus.go
#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, topic: &str, payload: Box<dyn Any + Send>) -> Result<()>;
}

/// Replaces internal/ports/idgen.go
pub trait IDGen: Send + Sync {
    fn new_id(&self) -> String;
}

/// Replaces internal/ports/rng.go
pub trait Rng: Send + Sync {
    fn intn(&self, n: i32) -> i32;
}