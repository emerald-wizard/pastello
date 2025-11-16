use crate::{
    domain::{
        game::{Engine, EngineFactory as DomainEngineFactory, GameType},
        puzzle::PuzzleEngine,
        trivia::TriviaEngine,
    },
    ports::{Clock, IDGen, Rng},
};
use std::{fmt::Debug, sync::Arc};

// This struct is the "Application Service" wrapper around the domain factory.
// It holds the dependencies.
#[derive(Clone)]
pub struct EngineFactory {
    domain_factory: Arc<DomainEngineFactory>,
}

// Manually implement Debug because Arc<dyn...> doesn't implement it.
impl Debug for EngineFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EngineFactory")
            .field("domain_factory", &"Arc<DomainEngineFactory>")
            .finish()
    }
}

impl EngineFactory {
    pub fn new(
        clock: Arc<dyn Clock>,
        rng: Arc<dyn Rng>,
        id_gen: Arc<dyn IDGen>,
    ) -> Self {
        Self {
            domain_factory: Arc::new(DomainEngineFactory::new(clock, rng, id_gen)),
        }
    }

    // Creates a new engine instance based on the game type.
    pub fn create(&self, game_type: &GameType) -> Box<dyn Engine> {
        self.domain_factory.create(game_type)
    }

    // A helper function to create an engine for a specific command type.
    // This is used by the CommandRegistry.
    pub fn create_for_command(
        &self,
        command_name: &str,
    ) -> Option<Box<dyn Engine>> {
        let game_type = match command_name {
            "PuzzleMove" | "PuzzleUndo" => Some(GameType::Puzzle),
            "TriviaSubmit" | "TriviaHint" => Some(GameType::Trivia),
            _ => None,
        };

        game_type.map(|gt| self.create(&gt))
    }
}

// This is the implementation for the domain-level factory,
// which is now correctly in its own module.
impl DomainEngineFactory {
    pub fn create(&self, game_type: &GameType) -> Box<dyn Engine> {
        match game_type {
            GameType::Puzzle => Box::new(PuzzleEngine::new(
                self.clock.clone(),
                self.rng.clone(),
                self.id_gen.clone(),
            )),
            GameType::Trivia => Box::new(TriviaEngine::new(
                self.clock.clone(),
                self.rng.clone(),
                self.id_gen.clone(),
            )),
            // --- FIX: Removed unreachable pattern ---
            // _ => None, 
        }
    }
}