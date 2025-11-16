use crate::{
    application::{
        // --- FIX: Removed unused AppCommand ---
        // commands::AppCommand,
        services::engine_factory::EngineFactory,
    },
    domain::game::{DomainError, Engine},
    pb::runecraftstudios::pastello::game::{
        puzzle::v1 as puzzle, trivia::v1 as trivia,
    },
    // --- FIX: Removed unused AppCommandResponse ---
    // application::
    //     usecase::handle_game_command::AppCommandResponse,
};
use std::{any::Any, sync::Arc};

#[derive(Clone, Debug)]
pub struct CommandRegistry {
    engine_factory: Arc<EngineFactory>,
}

impl CommandRegistry {
    pub fn new(engine_factory: Arc<EngineFactory>) -> Self {
        Self { engine_factory }
    }

    // This method takes a Protobuf command, finds the right engine,
    // and maps it to a domain command.
    pub fn map_command(
        &self,
        cmd: Box<dyn Any + Send>,
    ) -> Result<(Box<dyn Engine>, Box<dyn Any + Send>), DomainError> {
        let (command_name, domain_command): (&str, Box<dyn Any + Send>) =
            if let Some(c) = cmd.downcast_ref::<puzzle::MovePieceCommand>() {
                ("PuzzleMove", Box::new(crate::domain::puzzle::Command::MovePiece(c.clone())))
            } else if cmd.downcast_ref::<puzzle::UndoMoveCommand>().is_ok() {
                ("PuzzleUndo", Box::new(crate::domain::puzzle::Command::UndoMove(puzzle::UndoMoveCommand {})))
            } else if let Some(c) = cmd.downcast_ref::<trivia::SubmitAnswerCommand>() {
                ("TriviaSubmit", Box::new(crate::domain::trivia::Command::SubmitAnswer(c.clone())))
            } else if cmd.downcast_ref::<trivia::RevealHintCommand>().is_ok() {
                ("TriviaHint", Box::new(crate::domain::trivia::Command::RevealHint(trivia::RevealHintCommand {})))
            } else {
                return Err(DomainError::Unknown); // Or a more specific error
            };

        let engine = self
            .engine_factory
            .create_for_command(command_name)
            .ok_or(DomainError::WrongEngine)?;

        Ok((engine, domain_command))
    }
}