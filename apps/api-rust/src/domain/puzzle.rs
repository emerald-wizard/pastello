use crate::domain::game::{
    DomainError, DomainEvent, EventMeta, GameCommand, GameSessionID, GameType, PlayerID, Session,
};
use crate::pb::runecraftstudios::pastello::game::puzzle::v1::{MovePieceCommand, UndoMoveCommand};
use crate::ports::{Clock, IdGenerator, Rng};
use anyhow::Result;
use async_trait::async_trait;
use std::any::Any;
use std::collections::VecDeque;
use std::sync::Arc;
use std::fmt;

// --- IMPLEMENT GameCommand for Protobuf structs ---

impl GameCommand for MovePieceCommand {
    fn get_type(&self) -> String { "MovePieceCommand".to_string() }
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> { self }
}

impl GameCommand for UndoMoveCommand {
    fn get_type(&self) -> String { "UndoMoveCommand".to_string() }
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> { self }
}

// --- ENGINE DEPENDENCIES ---

#[derive(Clone)]
pub struct EngineDependencies {
    clock: Arc<dyn Clock>,
    _rng: Arc<dyn Rng>,
    _id_gen: Arc<dyn IdGenerator>,
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
pub struct PieceMoved {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
    pub player_id: PlayerID,
}

impl DomainEvent for PieceMoved {
    fn event_type(&self) -> &'static str { "puzzle.piece_moved" }
    fn session_id(&self) -> &GameSessionID { &self.session_id }
    fn to_any_box(self: Box<Self>) -> Box<dyn Any + Send> { self }
    fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct MoveUndone {
    pub meta: EventMeta,
    pub session_id: GameSessionID,
    pub player_id: PlayerID,
}

impl DomainEvent for MoveUndone {
    fn event_type(&self) -> &'static str { "puzzle.move_undone" }
    fn session_id(&self) -> &GameSessionID { &self.session_id }
    fn to_any_box(self: Box<Self>) -> Box<dyn Any + Send> { self }
    fn clone_box(&self) -> Box<dyn DomainEvent> { Box::new(self.clone()) }
}

// --- ENGINE STATE ---

#[derive(Debug, Clone)]
pub struct State {
    _board: Vec<Vec<u32>>,
    move_history: VecDeque<MovePieceCommand>,
}

// --- ENGINE IMPLEMENTATION ---

#[derive(Debug, Clone)]
pub struct PuzzleEngine {
    state: State,
    deps: EngineDependencies,
}

impl PuzzleEngine {
    pub fn new(
        clock: Arc<dyn Clock>,
        rng: Arc<dyn Rng>,
        id_gen: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            state: State {
                _board: vec![vec![0; 3]; 3],
                move_history: VecDeque::new(),
            },
            deps: EngineDependencies { clock, _rng: rng, _id_gen: id_gen },
        }
    }

    fn move_piece(
        &mut self,
        session_id: &GameSessionID,
        cmd: &MovePieceCommand,
    ) -> Result<Vec<Box<dyn DomainEvent>>, DomainError> {
        if cmd.to_x > 10 || cmd.to_y > 10 {
            return Err(DomainError::OutOfBounds);
        }

        self.state.move_history.push_back(cmd.clone());

        let event = PieceMoved {
            meta: crate::domain::game::new_meta(self.deps.clock.as_ref()),
            session_id: session_id.clone(),
            player_id: cmd
                .player_id
                .as_ref()
                .map(|p| p.value.clone())
                .unwrap_or_default(),
        };

        Ok(vec![Box::new(event)])
    }

    fn undo_move(
        &mut self,
        session_id: &GameSessionID,
        player_id: &PlayerID,
    ) -> Result<Vec<Box<dyn DomainEvent>>, DomainError> {
        if self.state.move_history.pop_back().is_none() {
            return Err(DomainError::NothingToUndo);
        }

        let event = MoveUndone {
            meta: crate::domain::game::new_meta(self.deps.clock.as_ref()),
            session_id: session_id.clone(),
            player_id: player_id.clone(),
        };

        Ok(vec![Box::new(event)])
    }
}

#[async_trait]
impl crate::domain::game::Engine for PuzzleEngine {
    fn game_type(&self) -> GameType {
        GameType::Puzzle
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
            "MovePieceCommand" => {
                // FIX: Cast to Any first using helper
                let any_cmd = command.into_any();
                let cmd = any_cmd.downcast::<MovePieceCommand>().map_err(|_| DomainError::Internal("Downcast failed".into()))?;

                let session_id = cmd
                    .session_id
                    .as_ref()
                    .map(|s| s.value.clone())
                    .unwrap_or_default();
                self.move_piece(&session_id, &cmd)?;
            }
            "UndoMoveCommand" => {
                let any_cmd = command.into_any();
                let cmd = any_cmd.downcast::<UndoMoveCommand>().map_err(|_| DomainError::Internal("Downcast failed".into()))?;

                let session_id = cmd
                    .session_id
                    .as_ref()
                    .map(|s| s.value.clone())
                    .unwrap_or_default();
                let player_id = cmd
                    .player_id
                    .as_ref()
                    .map(|p| p.value.clone())
                    .unwrap_or_default();

                self.undo_move(&session_id, &player_id)?;
            }
            _ => return Err(DomainError::InvalidCommand),
        }

        Ok(())
    }
}
