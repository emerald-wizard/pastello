use crate::domain::game::{GameSessionID, GameType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartGameSessionCommand {
    // FIX: Use String instead of the non-serializable Protobuf struct.
    pub player_id: String, 
    pub game_type: GameType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCommandMessage {
    pub session_id: GameSessionID,
    pub r#type: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub player_id: String,
}