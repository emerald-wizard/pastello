use crate::domain::game::{
    DomainError, DomainEvent, EventMeta, GameCommand, GameSessionID, GameType, PlayerID, Session,
};
use crate::ports::{Clock, IdGenerator, Rng};
use crate::pb::runecraftstudios::pastello::game::trivia::v1::{SubmitAnswerCommand, RevealHintCommand};
use anyhow::Result;
use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;

// --- IMPLEMENT GameCommand for Protobuf structs ---

impl GameCommand for SubmitAnswerCommand {
    fn get_type(&self) -> String { "SubmitAnswerCommand".to_string() }
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> { self }
}

impl GameCommand for RevealHintCommand {
    fn get_type(&self) -> String { "RevealHintCommand".to_string() }
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> { self }
}

// --- ENGINE DEPENDENCIES ---

#[derive(Clone)] 
pub struct EngineDependencies {
    clock: Arc<dyn Clock>,
    rng: Arc<dyn Rng>,
    id_gen: Arc<dyn IdGenerator>,
}

impl fmt::Debug for EngineDependencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EngineDependencies")
            .field("clock", &"Arc<dyn Clock>")
            .field("rng", &"Arc<dyn Rng>")
            .field("id_gen", &"Arc<dyn IdGenerator>")
            .finish()
    }
}

// --- EVENTS ---

#[derive(Debug, Clone)] 
pub struct AnswerAccepted {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
    pub player_id: PlayerID,
    pub points_awarded: i32,
}

impl DomainEvent for AnswerAccepted {
    fn event_type(&self) -> &'static str { "trivia.answer_accepted" }
    fn session_id(&self) -> &GameSessionID { &self.session_id }
    fn to_any_box(self: Box<Self>) -> Box<dyn Any + Send> { self }
    fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)] 
pub struct HintRevealed {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
}

impl DomainEvent for HintRevealed {
    fn event_type(&self) -> &'static str { "trivia.hint_revealed" }
    fn session_id(&self) -> &GameSessionID { &self.session_id }
    fn to_any_box(self: Box<Self>) -> Box<dyn Any + Send> { self }
    fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }
}

// --- ENGINE STATE ---

#[derive(Debug, Clone)] 
pub struct State {
    pub scores: HashMap<PlayerID, i32>, 
    pub question_index: u32,
}

// --- ENGINE IMPLEMENTATION ---

#[derive(Debug, Clone)] 
pub struct TriviaEngine {
    state: State,
    deps: EngineDependencies,
}

impl TriviaEngine {
    pub fn new(
        clock: Arc<dyn Clock>,
        rng: Arc<dyn Rng>,
        id_gen: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            state: State {
                scores: HashMap::new(),
                question_index: 0,
            },
            deps: EngineDependencies { clock, rng, id_gen },
        }
    }

    fn submit_answer(
        &mut self,
        _session_id: &GameSessionID,
        cmd: &SubmitAnswerCommand,
    ) -> Result<Vec<Box<dyn DomainEvent>>, DomainError> {
        let player_id_str = cmd.player_id.as_ref().map(|p| p.value.clone()).unwrap_or_default();
        
        *self.state.scores.entry(player_id_str.clone()).or_insert(0) += 10;
        self.state.question_index += 1;
        
        let event = AnswerAccepted {
            meta: crate::domain::game::new_meta(self.deps.clock.as_ref()),
            session_id: _session_id.clone(),
            player_id: player_id_str,
            points_awarded: 10,
        };

        Ok(vec![Box::new(event)])
    }

    fn reveal_hint(
        &mut self,
        _session_id: &GameSessionID,
    ) -> Result<Vec<Box<dyn DomainEvent>>, DomainError> {
        let event = HintRevealed {
            meta: crate::domain::game::new_meta(self.deps.clock.as_ref()),
            session_id: _session_id.clone(),
        };

        Ok(vec![Box::new(event)])
    }
}

#[async_trait]
impl crate::domain::game::Engine for TriviaEngine {
    fn game_type(&self) -> GameType {
        GameType::Trivia
    }

    async fn apply(
        &self,
        session: Session,
        _cmd: Box<dyn Any + Send>,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), DomainError> {
        Ok((session, vec![]))
    }

    fn execute_command(
        &mut self,
        command: Box<dyn crate::domain::game::GameCommand>,
    ) -> Result<(), DomainError> {
        let command_type = command.get_type();

        match command_type.as_str() {
            "SubmitAnswerCommand" => {
                // FIX: Cast to Any before downcasting
                let any_cmd = command.into_any();
                let cmd = any_cmd.downcast::<SubmitAnswerCommand>().map_err(|_| DomainError::Internal("Downcast failed".into()))?;
                let session_id = cmd.session_id.as_ref().map(|s| s.value.clone()).unwrap_or_default();
                self.submit_answer(&session_id, &cmd)?;
            }
            "RevealHintCommand" => {
                 let any_cmd = command.into_any();
                let cmd = any_cmd.downcast::<RevealHintCommand>().map_err(|_| DomainError::Internal("Downcast failed".into()))?;
                let session_id = cmd.session_id.as_ref().map(|s| s.value.clone()).unwrap_or_default();
                self.reveal_hint(&session_id)?;
            }
            _ => return Err(DomainError::InvalidCommand),
        }

        Ok(())
    }
}