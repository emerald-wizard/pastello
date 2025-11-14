use anyhow::Result;
use std::sync::Arc;
use crate::{
    domain::game::{GameSessionID, PlayerID},
    application::{
        commands::AppCommand,
        services::game_service::GameService,
        usecase::handle_game_command::AppCommandResponse,
    },
};

/// Thin dispatcher, per the Go design.
/// Based on internal/application/services/command_registry.go
#[derive(Clone)]
pub struct CommandRegistry {
    svc: Arc<GameService>,
}

impl CommandRegistry {
    pub fn new(svc: Arc<GameService>) -> Self {
        Self { svc }
    }

    /// Handles an incoming command for a specific player and session.
    pub async fn handle(
        &self,
        session_id: GameSessionID,
        player_id: PlayerID, // This is the authenticated player
        cmd: AppCommand,
    ) -> Result<AppCommandResponse> {
        // Pass all info, including the authenticated player_id,
        // to the game service for processing.
        self.svc.handle_game_command(session_id, player_id, cmd).await
    }
}