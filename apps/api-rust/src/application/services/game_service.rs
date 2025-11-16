use crate::{
    application::{
        commands::{GameCommand, StartGame},
        services::command_registry::CommandRegistry,
        // --- FIX: Removed unused imports ---
        // usecase::handle_game_command::{HandleGameCommandUseCase, HandleGameCommandIn, AppCommandResponse},
        usecase::handle_game_command::HandleGameCommandUseCase,
    },
    // --- FIX: Removed unused import ---
    // commands::AppCommand,
    domain::game::{DomainError, Session},
    ports::{EventBus, Repo},
};
use std::sync::Arc;

// GameService is the main entry point for game-related application logic.
#[derive(Clone)]
pub struct GameService {
    repo: Arc<dyn Repo<Session>>,
    bus: Arc<dyn EventBus>,
    command_registry: Arc<CommandRegistry>,
}

impl GameService {
    pub fn new(
        repo: Arc<dyn Repo<Session>>,
        bus: Arc<dyn EventBus>,
        command_registry: Arc<CommandRegistry>,
    ) -> Self {
        Self {
            repo,
            bus,
            command_registry,
        }
    }

    pub async fn start_game(&self, cmd: StartGame) -> Result<(), DomainError> {
        // 1. Create a new session
        let session = Session {
            id: cmd.session_id.clone(),
            game_type: cmd.game_type.clone(),
            players: vec![cmd.player.value], // Create a new list with the player
        };

        // 2. Save the session
        self.repo.save(&session.id, session).await;

        tracing::info!("Game started: {:?}", cmd);
        Ok(())
    }

    pub async fn handle_command(
        &self,
        cmd: GameCommand,
    ) -> Result<(), DomainError> {
        // 1. Find the right engine for the command
        let (engine, domain_command) = self
            .command_registry
            .map_command(cmd.command)?;

        // 2. Create the use case
        let use_case = HandleGameCommandUseCase::new(
            self.repo.clone(),
            self.bus.clone(),
            engine,
        );

        // 3. Re-package the command
        let cmd_with_domain_obj = GameCommand {
            command: domain_command,
            ..cmd
        };
        
        // 4. Execute
        use_case.execute(cmd_with_domain_obj).await
    }
}