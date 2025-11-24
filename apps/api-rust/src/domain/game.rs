use crate::ports::Clock;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;
use thiserror::Error;

pub type GameSessionID = String;
pub type PlayerID = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameType {
    Puzzle,
    Trivia,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerID,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: GameSessionID,
    pub host_id: PlayerID,
    pub game_type: GameType,
    pub players: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMeta {
    pub at: DateTime<Utc>,
}

pub fn new_meta(clock: &dyn Clock) -> EventMeta {
    EventMeta { at: clock.now_utc() }
}

// --- DOMAIN EVENTS ---
pub trait DomainEvent: Any + Send + Sync + Debug {
    fn event_type(&self) -> &'static str;
    fn session_id(&self) -> &GameSessionID;
    fn to_any_box(self: Box<Self>) -> Box<dyn Any + Send>;
    fn clone_box(&self) -> Box<dyn DomainEvent>;
}

impl Clone for Box<dyn DomainEvent> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// --- COMMANDS ---

pub trait GameCommand: Send + Sync + Debug {
    fn get_type(&self) -> String;
    // FIX: Helper to allow downcasting
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send>;
}

// --- ENGINE ---

#[async_trait]
pub trait Engine: Send + Sync + Debug {
    fn game_type(&self) -> GameType;
    fn execute_command(&mut self, command: Box<dyn GameCommand>) -> Result<(), DomainError>;

    async fn apply(
        &self,
        session: Session,
        cmd: Box<dyn Any + Send>,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), DomainError>;
}

#[async_trait]
pub trait GameEngineFactory: Send + Sync + Debug {
    fn create_engine(&self, game_type: GameType) -> Box<dyn Engine>;
}

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Game session not found: {0}")]
    SessionNotFound(GameSessionID),
    #[error("Command sent to wrong game engine")]
    WrongEngine,
    #[error("Move is out of bounds")]
    OutOfBounds,
    #[error("Nothing to undo")]
    NothingToUndo,
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Internal domain error: {0}")]
    Internal(String),
}
