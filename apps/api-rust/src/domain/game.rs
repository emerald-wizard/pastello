use chrono::{DateTime, Utc};
use thiserror::Error;
use async_trait::async_trait;
use std::any::Any;
use crate::ports::Clock;

// --- From type.go ---
pub type GameSessionID = String;
pub type PlayerID = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    Unspecified,
    Trivia,
    Puzzle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    Unspecified,
    Created,
    Active,
    Ended,
    Cancelled,
}

// --- From errors.go ---
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("wrong engine for command")]
    WrongEngine,
    #[error("unsupported command")]
    UnsupportedCommand,
    #[error("action is out of bounds")]
    OutOfBounds,
    #[error("nothing to undo")]
    NothingToUndo,
    #[error("bad snapshot type")]
    BadSnapshotType,
}

// --- From events.go ---
pub trait DomainEvent: Send + Sync {
    fn name(&self) -> &'static str;
    fn occurred_at(&self) -> DateTime<Utc>;

    // FIX: Change 'Any' to 'Any + Send + Sync'
    fn as_any(&self) -> &(dyn Any + Send + Sync); 

    // NEW: Add this helper for cloning
    fn clone_box(&self) -> Box<dyn DomainEvent>;
}

#[derive(Debug, Clone)]
pub struct EventMeta {
    pub at: DateTime<Utc>,
}
impl EventMeta {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self { at }
    }
}
pub fn new_meta(clock: &dyn Clock) -> EventMeta {
    EventMeta::new(clock.now())
}

// --- From session.go ---
#[derive(Debug, Clone)]
pub struct Session {
    pub id: GameSessionID,
    pub game_type: GameType,
    pub player_ids: Vec<PlayerID>,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
    pub ruleset_id: String,
}
impl Session {
    pub fn new(
        id: GameSessionID,
        game_type: GameType,
        players: Vec<PlayerID>,
        at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            game_type,
            player_ids: players,
            status: SessionStatus::Created,
            created_at: at,
            ruleset_id: "".to_string(),
        }
    }
}

// --- From engine.go ---
#[async_trait]
pub trait Engine: Send + Sync {
    fn game_type(&self) -> GameType;
    
    /// Pure(ish): given session + command, decide next state & events.
    async fn apply(
        &self,
        session: &Session,
        cmd: Box<dyn Any + Send>,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), DomainError>;
}

// --- From stateful.go ---
pub trait StatefulEngine: Engine {
    fn snapshot(&self) -> Box<dyn Any + Send>;
    fn restore(&mut self, snap: Box<dyn Any + Send>) -> Result<(), DomainError>;
}