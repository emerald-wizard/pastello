use crate::application::commands::GameCommand;
use crate::domain::game::{DomainError, Engine, GameType, Session};
use crate::pb::runecraftstudios::pastello::game::{
    puzzle::v1 as puzzle, trivia::v1 as trivia,
};
use crate::ports::{EventBus, Repo};
use std::any::Any;
use std::sync::Arc;

pub struct HandleGameCommandUseCase {
    repo: Arc<dyn Repo<Session>>,
    bus: Arc<dyn EventBus>,
    engine: Arc<dyn Engine>,
}

impl HandleGameCommandUseCase {
    pub fn new(
        repo: Arc<dyn Repo<Session>>,
        bus: Arc<dyn EventBus>,
        engine: Arc<dyn Engine>,
    ) -> Self {
        Self { repo, bus, engine }
    }

    pub async fn execute(
        &self,
        input: GameCommand,
    ) -> Result<(), DomainError> {
        // 1. Find the session
        let session = self.repo.find(&input.session_id).await;
        let mut session = match session {
            Some(s) => s,
            None => {
                tracing::warn!(
                    "Session not found for command: {}",
                    &input.session_id
                );
                return Err(DomainError::SessionNotFound(
                    input.session_id.clone(),
                ));
            }
        };

        // 2. Authorize
        // --- FIX: Typo player_ids -> players ---
        if !session.players.contains(&input.player_id) {
            tracing::warn!(
                "Player {} not in session {}",
                &input.player_id,
                &input.session_id
            );
            return Err(DomainError::SessionNotFound(
                input.session_id.clone(),
            ));
        }

        // --- FIX: Removed invalid check ---
        // `GameType` enum does not have an `Unspecified` variant
        // if session.game_type == GameType::Unspecified {
        //     tracing::error!("Session has unspecified game type: {}", &input.session_id);
        //     return Err(DomainError::WrongEngine);
        // }

        if session.game_type != self.engine.game_type() {
            tracing::error!(
                "Command sent to wrong engine. Session: {:?}, Engine: {:?}",
                session.game_type,
                self.engine.game_type()
            );
            return Err(DomainError::WrongEngine);
        }

        // 3. Execute
        let (next_session, events) =
            self.engine.apply(&session, input.command).await?;

        // 4. Save
        session = next_session;
        self.repo.save(&session.id, session).await;

        // 5. Publish events
        for event in events {
            let _ = self.bus.publish(event.name(), event.clone_box()).await;
        }

        Ok(())
    }
}

// TODO: This logic should be moved into the `Engine` implementations
// This is a temporary solution to map from the `Any` type.
pub fn map_command(
    game_type: &GameType,
    cmd: Box<dyn Any + Send>,
) -> Option<Box<dyn Any + Send>> {
    match game_type {
        GameType::Puzzle => {
            if let Ok(c) = cmd.downcast::<puzzle::MovePieceCommand>() {
                // --- FIX: Use correct struct name ---
                Some(Box::new(crate::domain::puzzle::Command::MovePiece(
                    puzzle::MovePieceCommand { from_x: c.from_x, from_y: c.from_y, to_x: c.to_x, to_y: c.to_y }
                )))
            } else if cmd.downcast::<puzzle::UndoMoveCommand>().is_ok() {
                // --- FIX: Use correct struct name and syntax ---
                Some(Box::new(crate::domain::puzzle::Command::UndoMove(
                    puzzle::UndoMoveCommand {}
                )))
            } else {
                None
            }
        }
        GameType::Trivia => {
            if let Ok(c) = cmd.downcast::<trivia::SubmitAnswerCommand>() {
                // --- FIX: Use correct struct name ---
                Some(Box::new(crate::domain::trivia::Command::SubmitAnswer(
                    trivia::SubmitAnswerCommand { player_id: c.player_id.clone(), answer: c.answer.clone() }
                )))
            } else if cmd.downcast::<trivia::RevealHintCommand>().is_ok() {
                // --- FIX: Use correct struct name and syntax ---
                Some(Box::new(crate::domain::trivia::Command::RevealHint(
                    trivia::RevealHintCommand {}
                )))
            } else {
                None
            }
        }
    }
}