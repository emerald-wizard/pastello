use anyhow::{Result, anyhow, bail};
use std::sync::Arc;
use std::any::Any;
use crate::{
    domain::{
        game::{GameSessionID, DomainEvent, GameType, PlayerID},
        puzzle, trivia,
    },
    ports::{GameSessionRepo, EventBus},
    application::commands::AppCommand,
    application::services::engine_factory::EngineFactory,
};

// --- Input/Output Ports ---
pub struct HandleGameCommandIn {
    pub session_id: GameSessionID,
    pub player_id: PlayerID, // The authenticated user
    pub command: AppCommand,
}

#[derive(Debug, Clone)]
pub enum AppCommandResponse {
    PuzzlePieceMoved { from_x: i32, from_y: i32, to_x: i32, to_y: i32 },
    PuzzleMoveUndone,
    TriviaAnswerAccepted { delta: i32, total: i32 },
    TriviaHintRevealed { hint: String },
    NoReply,
}

pub struct HandleGameCommandOut {
    pub payload: AppCommandResponse,
}

// --- Use Case ---
#[derive(Clone)]
pub struct HandleGameCommandUseCase {
    repo: Arc<dyn GameSessionRepo>,
    bus: Arc<dyn EventBus>,
    engines: Arc<dyn EngineFactory>,
}

impl HandleGameCommandUseCase {
    pub fn new(
        repo: Arc<dyn GameSessionRepo>,
        bus: Arc<dyn EventBus>,
        engines: Arc<dyn EngineFactory>,
    ) -> Self {
        Self { repo, bus, engines }
    }

    pub async fn execute(&self, input: HandleGameCommandIn) -> Result<HandleGameCommandOut> {
        // 1) Load session
        let (session, _snapshot) = self.repo.load(&input.session_id).await?
            .ok_or_else(|| anyhow!("session not found: {}", input.session_id))?;

        // 2) AUTHORIZATION (AuthZ) CHECK
        if !session.player_ids.contains(&input.player_id) {
            bail!("authorization failed: player {} is not in session {}", input.player_id, input.session_id);
        }

        if session.game_type == GameType::Unspecified {
            return Err(anyhow!("session has no game type"));
        }

        // 3) Select engine
        let engine = self.engines.for_type(session.game_type)
            .ok_or_else(|| anyhow!("no engine for session type: {:?}", session.game_type))?;

        // 4) Map app command -> engine command
        let engine_cmd = map_to_engine_cmd(session.game_type, &input.command)
            .ok_or_else(|| anyhow!("unsupported command for session type"))?;

        // 5) Apply
        let (next_session, events) = engine.apply(&session, engine_cmd)
            .await
            .map_err(|e| anyhow!(e))?;

        // 6) Persist
        self.repo.save(&next_session, None).await?;

        // 7) Publish domain events
        for event in &events {
            // Downcast to call the trait method
            let _ = self.bus.publish(event.name(), event.clone_box()).await;
        }

        // 8) Return flattened payload
        let payload = flatten_events(events);
        
        Ok(HandleGameCommandOut { payload })
    }
}


// --- Helpers ---
fn map_to_engine_cmd(game_type: GameType, cmd: &AppCommand) -> Option<Box<dyn Any + Send>> {
    match game_type {
        GameType::Puzzle => match cmd {
            AppCommand::PuzzleMovePiece(c) => Some(Box::new(puzzle::Command::MovePiece(
                puzzle::MovePiece { from_x: c.from_x, from_y: c.from_y, to_x: c.to_x, to_y: c.to_y }
            ))),
            AppCommand::PuzzleUndoMove(_) => Some(Box::new(puzzle::Command::UndoMove(
                puzzle::UndoMove
            ))),
            _ => None,
        },
        GameType::Trivia => match cmd {
            AppCommand::TriviaSubmitAnswer(c) => Some(Box::new(trivia::Command::SubmitAnswer(
                trivia::SubmitAnswer { player_id: c.player_id.clone(), answer: c.answer.clone() }
            ))),
            AppCommand::TriviaRevealHint(_) => Some(Box::new(trivia::Command::RevealHint(
                trivia::RevealHint
            ))),
            _ => None,
        },
        _ => None,
    }
}

fn flatten_events(events: Vec<Box<dyn DomainEvent>>) -> AppCommandResponse {
    if let Some(event) = events.get(0) {
        if let Some(e) = event.as_any().downcast_ref::<trivia::AnswerAccepted>() {
            return AppCommandResponse::TriviaAnswerAccepted { delta: e.delta, total: e.total };
        }
        if let Some(e) = event.as_any().downcast_ref::<trivia::HintRevealed>() {
            return AppCommandResponse::TriviaHintRevealed { hint: e.hint.clone() };
        }
        if let Some(e) = event.as_any().downcast_ref::<puzzle::PieceMoved>() {
            return AppCommandResponse::PuzzlePieceMoved {
                from_x: e.from_x, from_y: e.from_y, to_x: e.to_x, to_y: e.to_y
            };
        }
        if event.as_any().downcast_ref::<puzzle::MoveUndone>().is_some() {
            return AppCommandResponse::PuzzleMoveUndone;
        }
    }
    AppCommandResponse::NoReply
}