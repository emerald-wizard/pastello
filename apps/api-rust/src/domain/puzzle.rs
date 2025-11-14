use crate::domain::game::*;
use crate::ports::{Clock, Rng, IDGen};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::any::Any;
use std::sync::Arc;

// --- From models.go ---
#[derive(Debug, Clone, Copy)]
pub struct Pos { pub x: i32, pub y: i32 }
#[derive(Debug, Clone, Copy)]
pub struct Move { pub from: Pos, pub to: Pos }
#[derive(Debug, Clone)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub history: Vec<Move>,
}
impl State {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height, history: Vec::new() }
    }
}

// --- From rules.go ---
#[derive(Debug, Clone)]
pub struct PuzzleRules {
    pub difficulty: String,
    pub allow_hints: bool,
    pub time_limit_seconds: i32,
    pub max_players: i32,
}

// --- From commands.go ---
#[derive(Debug, Clone)]
pub struct MovePiece { pub from_x: i32, pub from_y: i32, pub to_x: i32, pub to_y: i32 }
#[derive(Debug, Clone)]
pub struct UndoMove;
#[derive(Debug, Clone)]
pub enum Command {
    MovePiece(MovePiece),
    UndoMove(UndoMove),
}

// --- From events.go ---
#[derive(Debug, Clone)]
pub struct PieceMoved {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
    pub from_x: i32, pub from_y: i32,
    pub to_x: i32, pub to_y: i32,
}
impl DomainEvent for PieceMoved {
    fn name(&self) -> &'static str { "puzzle.piece_moved" }
    fn occurred_at(&self) -> DateTime<Utc> { self.meta.at }
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MoveUndone {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
}
impl DomainEvent for MoveUndone {
    fn name(&self) -> &'static str { "puzzle.move_undone" }
    fn occurred_at(&self) -> DateTime<Utc> { self.meta.at }
    fn as_any(&self) -> &dyn Any { self }
}

// --- From engine.go ---
#[derive(Debug, Clone)]
pub struct PuzzleEngine {
    deps: EngineDeps,
    state: State,
}
#[derive(Clone)]
pub struct EngineDeps {
    clock: Arc<dyn Clock>,
    _rng: Arc<dyn Rng>, // Mark as unused if not used yet
    _id_gen: Arc<dyn IDGen>, // Mark as unused if not used yet
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

impl PuzzleEngine {
    pub fn new(clock: Arc<dyn Clock>, rng: Arc<dyn Rng>, id_gen: Arc<dyn IDGen>) -> Self {
        Self {
            deps: EngineDeps { clock, _rng: rng, _id_gen: id_gen },
            state: State::new(4, 4), // Example default
        }
    }

    fn move_piece(&mut self, session_id: &GameSessionID, cmd: &MovePiece) -> Result<Vec<Box<dyn DomainEvent>>, DomainError> {
        if !self.in_bounds(cmd.from_x, cmd.from_y) || !self.in_bounds(cmd.to_x, cmd.to_y) {
            return Err(DomainError::OutOfBounds);
        }
        
        self.state.history.push(Move {
            from: Pos { x: cmd.from_x, y: cmd.from_y },
            to: Pos { x: cmd.to_x, y: cmd.to_y },
        });

        let evt = PieceMoved {
            meta: new_meta(self.deps.clock.as_ref()),
            session_id: session_id.clone(),
            from_x: cmd.from_x, from_y: cmd.from_y,
            to_x: cmd.to_x, to_y: cmd.to_y,
        };
        Ok(vec![Box::new(evt)])
    }
    
    fn undo_move(&mut self, session_id: &GameSessionID) -> Result<Vec<Box<dyn DomainEvent>>, DomainError> {
        if self.state.history.pop().is_none() {
            return Err(DomainError::NothingToUndo);
        }

        let evt = MoveUndone {
            meta: new_meta(self.deps.clock.as_ref()),
            session_id: session_id.clone(),
        };
        Ok(vec![Box::new(evt)])
    }
    
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.state.width && y >= 0 && y < self.state.height
    }
}

#[async_trait]
impl Engine for PuzzleEngine {
    fn game_type(&self) -> GameType { GameType::Puzzle }

    async fn apply(
        &self,
        session: &Session,
        cmd: Box<dyn Any + Send>,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), DomainError> {
        let mut engine_clone = self.clone();

        let domain_cmd = cmd.downcast::<Command>()
            .map_err(|_| DomainError::WrongEngine)?;

        let events = match *domain_cmd {
            Command::MovePiece(ref c) => engine_clone.move_piece(&session.id, c)?,
            Command::UndoMove(_) => engine_clone.undo_move(&session.id)?,
        };
        
        let next_session = session.clone();
        Ok((next_session, events))
    }
}

fn as_any(&self) -> &(dyn Any + Send + Sync) { self }
fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }

// Inside impl DomainEvent for MoveUndone
fn as_any(&self) -> &(dyn Any + Send + Sync) { self }
fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }