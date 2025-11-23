use crate::ports::{Clock, GameRepository, IdGenerator, EventBus};
use crate::domain::game::{Session, GameType, Player, GameEngineFactory, Engine};
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

    pub async fn start_game_session(&self, cmd: StartGameSessionCommand) -> Result<String> {
        let session_id = self.id_gen.new_id();
        
        let host = Player {
            id: cmd.player_id.clone(),
            name: "Host".to_string(), 
        };

        let session = Session {
            id: session_id.clone(),
            game_type: cmd.game_type,
            players: vec![host], 
            host_id: cmd.player_id.clone(), 
        };

        self.repo.save(&session.id.clone(), session).await?;
        
        info!("Game started: {}", session_id); 

        Ok(session_id)
    }

    pub async fn handle_game_command(&self, game_id: &str, command: GameCommandMessage) -> Result<()> {
        let session = self.repo.get(game_id).await?;
        let session = match session {
            Some(s) => s,
            None => bail!("Game session not found"),
        };

        // 1. Create engine (returns Box<dyn Engine>)
        let engine = self.engine_factory.create_engine(session.game_type.clone());
        
        // 2. Wrap in Mutex. Box<dyn Engine> naturally implements Send if trait does.
        // We cast explicitly to the type expected by the UseCase.
        // Note: Box<dyn Engine> satisfies Box<dyn Engine + Send> because Engine: Send.
        let engine_mutex: Arc<Mutex<Box<dyn Engine + Send>>> = Arc::new(Mutex::new(engine as Box<dyn Engine + Send>));
        
        let use_case = HandleGameCommandUseCase::new(
            self.repo.clone(),
            self.clock.clone(),
            self.command_registry.clone(),
            engine_mutex, 
        );

        use_case.execute(session, command).await
    }
}