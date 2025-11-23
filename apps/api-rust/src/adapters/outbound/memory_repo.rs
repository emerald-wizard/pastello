// FIX: Corrected trait import name from Repo to GameRepository (E0432)
use crate::ports::GameRepository; 
use crate::domain::game::{Session, GameSessionID};
use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct MemoryRepo {
    sessions: DashMap<GameSessionID, Session>,
}

impl MemoryRepo {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
        }
    }
}

#[async_trait]
impl GameRepository for MemoryRepo {
    async fn get(&self, id: &str) -> Result<Option<Session>> {
        let id = id.to_string();
        Ok(self.sessions.get(&id).map(|entry| entry.clone()))
    }

    async fn save(&self, id: &str, session: Session) -> Result<()> {
        let id = id.to_string();
        self.sessions.insert(id, session);
        Ok(())
    }
}