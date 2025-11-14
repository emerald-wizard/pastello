use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use std::any::Any;
use std::sync::Arc;
use crate::{
    domain::game::{GameSessionID, Session},
    ports::GameSessionRepo,
};

// Use DashMap for a concurrent-safe HashMap
pub struct MemorySessionRepo {
    store: Arc<DashMap<GameSessionID, Session>>,
}

impl MemorySessionRepo {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl GameSessionRepo for MemorySessionRepo {
    async fn save(
        &self,
        session: &Session,
        _snapshot: Option<Box<dyn Any + Send>>, // Ignored
    ) -> Result<()> {
        self.store.insert(session.id.clone(), session.clone());
        Ok(())
    }

    async fn load(
        &self,
        id: &GameSessionID,
    ) -> Result<Option<(Session, Option<Box<dyn Any + Send>>)>> {
        let entry = self.store.get(id);
        match entry {
            Some(entry) => {
                let session = entry.value().clone();
                Ok(Some((session, None)))
            }
            None => Ok(None),
        }
    }
}