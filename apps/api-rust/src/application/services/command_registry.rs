use crate::domain::{
    game::{GameCommand, GameType, DomainError, GameEngineFactory},
};
use crate::pb::runecraftstudios::pastello::game::{
    puzzle::v1::{MovePieceCommand, UndoMoveCommand},
    trivia::v1::{RevealHintCommand, SubmitAnswerCommand},
    types::v1::{GameSessionId, PlayerId},
};
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
use serde::Deserialize;
use serde_json::Value;
use std::fmt;

type CommandDeserializer = Box<dyn Fn(&Value) -> Result<Box<dyn GameCommand>> + Send + Sync + 'static>;
type CommandMap = HashMap<&'static str, CommandDeserializer>;

fn build_move_piece_command(dto: MovePieceDto) -> MovePieceCommand {
    // Note: the protobuf command only has coordinates and identifiers; it does not expose a
    // standalone "piece" field. Using this builder keeps the struct initialization limited to
    // the fields that actually exist in the generated type.
    MovePieceCommand {
        session_id: Some(GameSessionId { value: dto.session_id }),
        player_id: Some(PlayerId { value: dto.player_id }),
        from_x: dto.from_x as i32,
        from_y: dto.from_y as i32,
        to_x: dto.to_x as i32,
        to_y: dto.to_y as i32,
    }
}

// --- INTERNAL DTOs ---
#[derive(Deserialize)]
struct MovePieceDto {
    pub from_x: u32,
    pub from_y: u32,
    pub to_x: u32,
    pub to_y: u32,
    pub player_id: String,
    pub session_id: String,
}

#[derive(Deserialize)]
struct UndoMoveDto {
    pub player_id: String,
    pub session_id: String,
}

#[derive(Deserialize)]
struct SubmitAnswerDto {
    pub player_id: String,
    pub session_id: String,
}

#[derive(Deserialize)]
struct RevealHintDto {
    pub session_id: String,
}

// --- REGISTRY ---

pub struct CommandRegistry {
    engine_factory: Arc<dyn GameEngineFactory>,
    commands: HashMap<GameType, CommandMap>,
}

impl fmt::Debug for CommandRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRegistry")
            .field("engine_factory", &self.engine_factory)
            .field("commands_count", &self.commands.len())
            .finish()
    }
}

impl CommandRegistry {
    pub fn new(engine_factory: Arc<dyn GameEngineFactory>) -> Self {
        Self {
            engine_factory,
            commands: Self::init_command_map(),
        }
    }

    fn init_command_map() -> HashMap<GameType, CommandMap> {
        let move_piece: CommandDeserializer = Box::new(|payload: &Value| {
            let dto: MovePieceDto = serde_json::from_value(payload.clone())?;
            let cmd = build_move_piece_command(dto);
            Ok(Box::new(cmd) as Box<dyn GameCommand>)
        });

        let undo_move: CommandDeserializer = Box::new(|payload: &Value| {
            let dto: UndoMoveDto = serde_json::from_value(payload.clone())?;
            let cmd = UndoMoveCommand {
                session_id: Some(GameSessionId { value: dto.session_id }),
                player_id: Some(PlayerId { value: dto.player_id }),
            };
            Ok(Box::new(cmd) as Box<dyn GameCommand>)
        });

        let submit_answer: CommandDeserializer = Box::new(|payload: &Value| {
            let dto: SubmitAnswerDto = serde_json::from_value(payload.clone())?;
            let cmd = SubmitAnswerCommand {
                session_id: Some(GameSessionId { value: dto.session_id }),
                player_id: Some(PlayerId { value: dto.player_id }),
                answer: "placeholder".to_string(),
            };
            Ok(Box::new(cmd) as Box<dyn GameCommand>)
        });

        let reveal_hint: CommandDeserializer = Box::new(|payload: &Value| {
            let dto: RevealHintDto = serde_json::from_value(payload.clone())?;
            let cmd = RevealHintCommand {
                session_id: Some(GameSessionId { value: dto.session_id }),
            };
            Ok(Box::new(cmd) as Box<dyn GameCommand>)
        });

        let mut puzzle_map: CommandMap = HashMap::new();
        puzzle_map.insert("PuzzleMove", move_piece);
        puzzle_map.insert("PuzzleUndo", undo_move);

        let mut trivia_map: CommandMap = HashMap::new();
        trivia_map.insert("TriviaSubmit", submit_answer);
        trivia_map.insert("TriviaHint", reveal_hint);

        let mut map = HashMap::new();
        map.insert(GameType::Puzzle, puzzle_map);
        map.insert(GameType::Trivia, trivia_map);
        map
    }

    pub fn deserialize(
        &self,
        game_type: GameType,
        command_type: &str,
        payload: &serde_json::Value,
    ) -> Result<Box<dyn GameCommand>> {
        let map = self
            .commands
            .get(&game_type)
            .ok_or_else(|| DomainError::Internal(format!("No command map for {:?}", game_type)))?;

        let deserializer = map
            .get(command_type)
            .ok_or_else(|| DomainError::InvalidCommand)?;

        deserializer(payload)
    }
}
