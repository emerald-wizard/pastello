use crate::domain::game::*;
use crate::ports::{Clock, Rng, IDGen};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

// --- From models.go ---
#[derive(Debug, Clone)]
pub struct State {
    pub scores: HashMap<PlayerID, i32>,
    pub hints: Vec<String>,
}
impl State {
    pub fn new() -> Self {
        Self { scores: HashMap::new(), hints: Vec::new() }
    }
}

// --- From rules.go ---
#[derive(Debug, Clone)]
pub struct TriviaRules {
    // ... fields
}

// --- From commands.go ---
#[derive(Debug, Clone)]
pub struct SubmitAnswer { pub player_id: PlayerID, pub answer: String }
#[derive(Debug, Clone)]
pub struct RevealHint;
#[derive(Debug, Clone)]
pub enum Command {
    SubmitAnswer(SubmitAnswer),
    RevealHint(RevealHint),
}

// --- From events.go ---
#[derive(Debug, Clone)]
pub struct AnswerAccepted {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
    pub player_id: PlayerID,
    pub delta: i32,
    pub total: i32,
}
impl DomainEvent for AnswerAccepted {
    fn name(&self) -> &'static str { "trivia.answer_accepted" }
    fn occurred_at(&self) -> DateTime<Utc> { self.meta.at }
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct HintRevealed {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
    pub hint: String,
}
impl DomainEvent for HintRevealed {
    fn name(&self) -> &'static str { "trivia.hint_revealed" }
    fn occurred_at(&self) -> DateTime<Utc> { self.meta.at }
    fn as_any(&self) -> &dyn Any { self }
}

// --- From engine.go ---
#[derive(Debug, Clone)]
pub struct TriviaEngine {
    deps: EngineDeps,
    state: State,
}
#[derive(Clone)]
pub struct EngineDeps {
    clock: Arc<dyn Clock>,
    _rng: Arc<dyn Rng>,
    _id_gen: Arc<dyn IDGen>,
}
impl std::fmt::Debug for EngineDeps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EngineDeps")
         .field("clock", &"Arc<dyn Clock>")
         .field("_rng", &"Arc<dyn Rng>")
         .field("_id_gen", &"Arc<dyn IDGen>")
         .finish()
    }
}

impl TriviaEngine {
    pub fn new(clock: Arc<dyn Clock>, rng: Arc<dyn Rng>, id_gen: Arc<dyn IDGen>) -> Self {
        Self {
            deps: EngineDeps { clock, _rng: rng, _id_gen: id_gen },
            state: State::new(),
        }
    }

    fn submit_answer(&mut self, session_id: &GameSessionID, cmd: &SubmitAnswer) -> Vec<Box<dyn DomainEvent>> {
        const DELTA: i32 = 10; // Logic from Go
        let current_score = self.state.scores.entry(cmd.player_id.clone()).or_insert(0);
        *current_score += DELTA;
        let total = *current_score;

        let evt = AnswerAccepted {
            meta: new_meta(self.deps.clock.as_ref()),
            session_id: session_id.clone(),
            player_id: cmd.player_id.clone(),
            delta: DELTA,
            total,
        };
        vec![Box::new(evt)]
    }

    fn reveal_hint(&mut self, session_id: &GameSessionID) -> Vec<Box<dyn DomainEvent>> {
        let hint = "This is a hint.".to_string(); // Logic from Go
        self.state.hints.push(hint.clone());

        let evt = HintRevealed {
            meta: new_meta(self.deps.clock.as_ref()),
            session_id: session_id.clone(),
            hint,
        };
        vec![Box::new(evt)]
    }
}

#[async_trait]
impl Engine for TriviaEngine {
    fn game_type(&self) -> GameType { GameType::Trivia }

    async fn apply(
        &self,
        session: &Session,
        cmd: Box<dyn Any + Send>,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), DomainError> {
        let mut engine_clone = self.clone();
        let domain_cmd = cmd.downcast::<Command>()
            .map_err(|_| DomainError::WrongEngine)?;

        let events = match *domain_cmd {
            Command::SubmitAnswer(ref c) => engine_clone.submit_answer(&session.id, c),
            Command::RevealHint(_) => engine_clone.reveal_hint(&session.id),
        };
        
        let next_session = session.clone();
        Ok((next_session, events))
    }
}

fn as_any(&self) -> &(dyn Any + Send + Sync) { self }
fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }

// Inside impl DomainEvent for HintRevealed
fn as_any(&self) -> &(dyn Any + Send + Sync) { self }
fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }