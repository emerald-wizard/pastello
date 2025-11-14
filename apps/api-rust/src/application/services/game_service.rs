use anyhow::Result;
use std::sync::Arc;
use crate::{
    domain::game::{GameSessionID, PlayerID},
    application::{
        usecase::handle_game_command::{HandleGameCommandUseCase, HandleGameCommandIn, AppCommandResponse},
        commands::AppCommand,
    }
};

/// Facade over the application use cases.
/// Based on internal/application/services/game_service.go
#[derive(Clone)]
pub struct GameService {
    handle_uc: Arc<HandleGameCommandUseCase>,
}

impl GameService {
    pub fn new(handle_uc: HandleGameCommandUseCase) -> Self {
        Self {
            handle_uc: Arc::new(handle_uc),
        }
    }

    pub async fn handle_game_command(
        &self,
        session_id: GameSessionID,
        player_id: PlayerID, // <-- AuthZ
        cmd: AppCommand,
    ) -> Result<AppCommandResponse> {
        let input = HandleGameCommandIn { session_id, player_id, command: cmd };
        let out = self.handle_uc.execute(input).await?;
        Ok(out.payload)
    }
}