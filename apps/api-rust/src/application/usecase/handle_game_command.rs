use crate::application::commands::GameCommandMessage;
use crate::application::services::command_registry::CommandRegistry;
use crate::domain::game::{Engine, Session, GameCommand};
use crate::ports::{Clock, GameRepository};
use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{warn, info};

pub struct HandleGameCommandUseCase {
    repo: Arc<dyn GameRepository>,
    _clock: Arc<dyn Clock>,
    command_registry: Arc<CommandRegistry>,
    engine: Arc<Mutex<Box<dyn Engine + Send>>>,
}

impl HandleGameCommandUseCase {
    pub fn new(
        repo: Arc<dyn GameRepository>,
        clock: Arc<dyn Clock>,
        command_registry: Arc<CommandRegistry>,
        engine: Arc<Mutex<Box<dyn Engine + Send>>>,
    ) -> Self {
        Self {
            repo,
            _clock: clock,
            command_registry,
            engine,
        }
    }

    // JSON-based entry point (Optional/Legacy)
    pub async fn execute(&self, session: Session, command: GameCommandMessage) -> Result<()> {
        let command_type = command.r#type.as_str();
        let game_type = session.game_type.clone();

        let game_command = match self.command_registry.deserialize(game_type, command_type, &command.payload) {
            Ok(c) => c,
            Err(e) => {
                warn!("Failed to deserialize command {}: {:?}", command_type, e);
                bail!(e.to_string());
            }
        };
        
        self.run_engine(session, game_command).await
    }

    // NEW: Direct entry point for Pre-parsed Commands
    pub async fn execute_direct(&self, session: Session, command: Box<dyn GameCommand>) -> Result<()> {
        self.run_engine(session, command).await
    }

    async fn run_engine(&self, session: Session, command: Box<dyn GameCommand>) -> Result<()> {
        let mut engine_lock = self.engine.lock().await;
        
        info!("Executing command: {:?}", command.get_type());
        
        engine_lock.execute_command(command)?;

        // Save state
        self.repo.save(&session.id.clone(), session).await?;
        
        Ok(())
    }
}