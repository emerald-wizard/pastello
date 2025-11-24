use crate::application::commands::GameCommandMessage;
use crate::application::services::command_registry::CommandRegistry;
use crate::domain::game::{Engine, Session};
use crate::ports::{Clock, GameRepository};
use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::warn;

pub struct HandleGameCommandUseCase {
    repo: Arc<dyn GameRepository>,
    _clock: Arc<dyn Clock>,
    command_registry: Arc<CommandRegistry>,
    // FIX: Engine stored in Mutex for safe mutable access across async tasks.
    // We explicitly require `Send` because this struct will be moved across thread boundaries.
    engine: Arc<Mutex<Box<dyn Engine + Send>>>,
}

impl HandleGameCommandUseCase {
    pub fn new(
        repo: Arc<dyn GameRepository>,
        clock: Arc<dyn Clock>,
        command_registry: Arc<CommandRegistry>,
        // FIX: The constructor accepts the engine wrapped in a concurrency-safe container
        engine: Arc<Mutex<Box<dyn Engine + Send>>>,
    ) -> Self {
        Self {
            repo,
            _clock: clock,
            command_registry,
            engine,
        }
    }

    pub async fn execute(&self, session: Session, command: GameCommandMessage) -> Result<()> {
        let command_type = command.r#type.as_str();

        // Clone session.game_type to prevent partial move errors when we pass it to the registry
        let game_type = session.game_type.clone();

        // Find and deserialize the command payload using the registry
        let game_command =
            match self
                .command_registry
                .deserialize(game_type, command_type, &command.payload)
            {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to deserialize command {}: {:?}", command_type, e);
                    bail!(e.to_string());
                }
            };

        // FIX: Lock the Mutex to gain mutable access to the engine instance.
        // This blocks the current task until the lock is acquired, ensuring exclusive access.
        let mut engine_lock = self.engine.lock().await;

        // Now we have a mutable reference to the Engine trait object, satisfying the `&mut self` requirement
        engine_lock.execute_command(game_command)?;

        // The Mutex guard `engine_lock` is dropped at the end of this scope, releasing the lock.

        // Save the updated session state to the repository
        // We clone the ID because `session` is moved into `save`
        self.repo.save(&session.id.clone(), session).await?;

        Ok(())
    }
}
