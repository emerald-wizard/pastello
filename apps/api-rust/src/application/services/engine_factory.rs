use crate::ports::{Clock, Rng};
use crate::domain::game::{Engine, GameType, GameEngineFactory};
use std::sync::Arc;
use std::fmt;

// DefaultEngineFactory now implements Debug manually to satisfy trait bounds
#[derive(Clone)]
pub struct DefaultEngineFactory {
    clock: Arc<dyn Clock + Send + Sync>,
    rng: Arc<dyn Rng + Send + Sync>,
}

impl fmt::Debug for DefaultEngineFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DefaultEngineFactory")
            .field("clock", &"Arc<dyn Clock>")
            .field("rng", &"Arc<dyn Rng>")
            .finish()
    }
}

impl DefaultEngineFactory {
    pub fn new(clock: Arc<dyn Clock + Send + Sync>, rng: Arc<dyn Rng + Send + Sync>) -> Self {
        Self { clock, rng }
    }
}

impl GameEngineFactory for DefaultEngineFactory { 
    fn create_engine(&self, game_type: GameType) -> Box<dyn Engine> {
        match game_type {
            GameType::Puzzle => {
                Box::new(crate::domain::puzzle::PuzzleEngine::new(
                    self.clock.clone() as Arc<dyn crate::ports::Clock>, 
                    self.rng.clone() as Arc<dyn crate::ports::Rng>,
                    Arc::new(crate::adapters::outbound::id_gen::UuidGenerator::new()) as Arc<dyn crate::ports::IdGenerator>,
                ))
            }
            GameType::Trivia => {
                 Box::new(crate::domain::trivia::TriviaEngine::new(
                    self.clock.clone() as Arc<dyn crate::ports::Clock>,
                    self.rng.clone() as Arc<dyn crate::ports::Rng>,
                    Arc::new(crate::adapters::outbound::id_gen::UuidGenerator::new()) as Arc<dyn crate::ports::IdGenerator>,
                ))
            }
        }
    }
}