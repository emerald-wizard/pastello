use std::sync::Arc;
use crate::{
    domain::{
        game::{Engine, GameType},
        puzzle::PuzzleEngine,
        trivia::TriviaEngine,
    },
    ports::{Clock, Rng, IDGen},
};

pub trait EngineFactory: Send + Sync {
    fn for_type(&self, game_type: GameType) -> Option<Box<dyn Engine>>;
}

pub struct EngineFactoryImpl {
    clock: Arc<dyn Clock>,
    rng: Arc<dyn Rng>,
    id_gen: Arc<dyn IDGen>,
}

impl EngineFactoryImpl {
    pub fn new(
        clock: Arc<dyn Clock>,
        rng: Arc<dyn Rng>,
        id_gen: Arc<dyn IDGen>,
    ) -> Self {
        Self { clock, rng, id_gen }
    }
}

impl EngineFactory for EngineFactoryImpl {
    fn for_type(&self, game_type: GameType) -> Option<Box<dyn Engine>> {
        match game_type {
            GameType::Puzzle => Some(Box::new(PuzzleEngine::new(
                self.clock.clone(),
                self.rng.clone(),
                self.id_gen.clone(),
            ))),
            GameType::Trivia => Some(Box::new(TriviaEngine::new(
                self.clock.clone(),
                self.rng.clone(),
                self.id_gen.clone(),
            ))),
            _ => None,
        }
    }
}