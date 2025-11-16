// --- FIX: Import correct pb struct ---
use crate::pb::runecraftstudios::pastello::game::types::v1::PlayerId as PbPlayerId;
use crate::domain::game::{GameSessionID, GameType, PlayerID};
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
pub struct StartGame {
    pub game_type: GameType,
    // --- FIX: Use correct pb struct ---
    pub player: PbPlayerId,
    pub session_id: GameSessionID,
}

#[derive(Debug)]
pub struct GameCommand {
    pub session_id: GameSessionID,
    pub player_id: PlayerID,
    pub command: Box<dyn Any + Send>,
}