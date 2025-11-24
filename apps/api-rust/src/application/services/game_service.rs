use crate::ports::{Clock, GameRepository, IdGenerator, EventBus};
use crate::domain::game::{Session, Player, GameEngineFactory, Engine, GameCommand};
use crate::application::usecase::handle_game_command::HandleGameCommandUseCase;
use crate::application::commands::{StartGameSessionCommand, GameCommandMessage};
use crate::application::services::command_registry::CommandRegistry;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{Result, bail};
use tracing::info;

#[derive(Clone)]
pub struct GameService {
    repo: Arc<dyn GameRepository>,
    _event_bus: Arc<dyn EventBus>,
    clock: Arc<dyn Clock>,
    id_gen: Arc<dyn IdGenerator>,
    engine_factory: Arc<dyn GameEngineFactory>,
    command_registry: Arc<CommandRegistry>,
}

impl GameService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        repo: Arc<dyn GameRepository>,
        event_bus: Arc<dyn EventBus>,
        clock: Arc<dyn Clock>,
        id_gen: Arc<dyn IdGenerator>,
        engine_factory: Arc<dyn GameEngineFactory>,
        command_registry: Arc<CommandRegistry>,
    ) -> Self {
        Self {
            repo,
            _event_bus: event_bus,
            clock,
            id_gen,
            engine_factory,
            command_registry,
        }
    }

    // Helper for WebSocket auth flow to ensure session exists
    pub async fn force_save_session(&self, session: Session) -> Result<()> {
        self.repo.save(&session.id.clone(), session).await
    }

    pub async fn start_game_session(&self, cmd: StartGameSessionCommand) -> Result<String> {
        let session_id = self.id_gen.new_id();

        let host = Player {
            id: cmd.player_id.clone(),
            name: "Host".to_string(),
        };

        let session = Session {
            id: session_id.clone(),
            host_id: cmd.player_id.clone(),
            game_type: cmd.game_type,
            players: vec![host],
        };

        self.repo.save(&session.id.clone(), session).await?;
        info!("Game started: {}", session_id);
        Ok(session_id)
    }

    // Legacy JSON handler (kept for compatibility)
    pub async fn handle_game_command(&self, game_id: &str, command: GameCommandMessage) -> Result<()> {
         // This logic is now largely superseded by handle_domain_command for WS
         // But keeping the structure for reference
         bail!("Use handle_domain_command for Protobuf messages");
    }

    // NEW: Direct handler for Typed Domain Commands (from Protobuf)
    pub async fn handle_domain_command(&self, game_id: &str, command: Box<dyn GameCommand>) -> Result<()> {
        let session = self.repo.get(game_id).await?;
        let session = match session {
            Some(s) => s,
            None => bail!("Game session not found for id: {}", game_id),
        };

        let engine = self.engine_factory.create_engine(session.game_type.clone());
        let engine_mutex: Arc<Mutex<Box<dyn Engine + Send>>> = Arc::new(Mutex::new(engine as Box<dyn Engine + Send>));

        let use_case = HandleGameCommandUseCase::new(
            self.repo.clone(),
            self.clock.clone(),
            self.command_registry.clone(),
            engine_mutex,
        );

        // We pass the command directly, bypassing the JSON registry
        use_case.execute_direct(session, command).await
    }
}