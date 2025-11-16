use crate::ports::{Clock, IDGen, Rng};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;

pub type GameSessionID = String;
pub type PlayerID = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameType {
    Puzzle,
    Trivia,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: GameSessionID,
    pub game_type: GameType,
    pub players: Vec<PlayerID>,
}

#[derive(Debug, Clone)]
pub struct EventMeta {
    pub at: DateTime<Utc>,
}

pub fn new_meta(clock: &dyn Clock) -> EventMeta {
    EventMeta { at: clock.now() }
}

// --- FIX: Added Debug trait bound ---
pub trait DomainEvent: Any + Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn as_any(&self) -> &(dyn Any + Send + Sync);
    
    // --- FIX: Return Box<dyn Any + Send + Sync> for upcasting ---
    fn clone_box(&self) -> Box<dyn Any + Send + Sync>;
}

// --- FIX: Removed incorrect `impl Clone for Box<dyn DomainEvent>` ---

#[async_trait]
pub trait Engine: Send + Sync + Debug {
    fn game_type(&self) -> GameType;
    async fn apply(
        &self,
        session: &Session,
        cmd: Box<dyn Any + Send>,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), DomainError>;
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
    #[error("Unknown domain error")]
    Unknown,
}

// --- FIX: Removed derive(Debug) ---
#[derive(Clone)]
pub struct EngineFactory {
    clock: Arc<dyn Clock>,
    rng: Arc<dyn Rng>,
    id_gen: Arc<dyn IDGen>,
}

// --- FIX: Manually implement Debug ---
impl Debug for EngineFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EngineFactory")
            .field("clock", &"Arc<dyn Clock>")
            .field("rng", &"Arc<dyn Rng>")
            .field("id_gen", &"Arc<dyn IDGen>")
            .finish()
    }
}


impl EngineFactory {
    pub fn new(
        clock: Arc<dyn Clock>,
        rng: Arc<dyn Rng>,
        id_gen: Arc<dyn IDGen>,
    ) -> Self {
        Self { clock, rng, id_gen }
    }

    pub fn create(&self, game_type: &GameType) -> Box<dyn Engine> {
        match game_type {
            GameType::Puzzle => Box::new(crate::domain::puzzle::PuzzleEngine::new(
                self.clock.clone(),
                self.rng.clone(),
                self.id_gen.clone(),
            )),
            GameType::Trivia => Box::new(crate::domain::trivia::TriviaEngine::new(
                self.clock.clone(),
                self.rng.clone(),
                self.id_gen.clone(),
            )),
        }
    }
}