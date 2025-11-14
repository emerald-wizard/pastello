use anyhow::Result;
use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;
use std::panic;

// --- MODULES ---
mod adapters;
mod application;
mod config;
mod domain;
mod ports;

// --- IMPORTS ---
use crate::adapters::inbound::ws::websocket_handler;
use crate::adapters::inbound::ws::auth::TokenValidator;
use crate::adapters::outbound::{
    eventbus::NopBus, id_gen::UuidGen, memory_repo::MemorySessionRepo,
    clock::SystemClock, rng::ThreadRng,
};
use crate::application::services::{
    engine_factory::EngineFactoryImpl, game_service::GameService,
    command_registry::CommandRegistry,
};
use crate::application::usecase::handle_game_command::HandleGameCommandUseCase;
use crate::config::load_config;


// --- THIS IS THE CORRECTED PB MODULE ---
mod pb {
    // This creates a module `google` that our generated code
    // can find via `super::...::google`
    pub mod google {
        pub mod protobuf {
            // We import the types from the crate
            // instead of including a file
            pub use prost_types::*;
        }
    }
    
    // This nested structure matches your protobuf packages
    // and fixes the "Envelope" name collision
    pub mod runecraftstudios {
        pub mod pastello {
            pub mod auth {
                pub mod session {
                    pub mod v1 {
                        include!("pb/runecraftstudios.pastello.auth.session.v1.rs");
                    }
                }
            }
            pub mod game {
                pub mod puzzle {
                    pub mod v1 {
                        include!("pb/runecraftstudios.pastello.game.puzzle.v1.rs");
                    }
                }
                pub mod session {
                    pub mod v1 {
                        include!("pb/runecraftstudios.pastello.game.session.v1.rs");
                    }
                }
                pub mod trivia {
                    pub mod v1 {
                        include!("pb/runecraftstudios.pastello.game.trivia.v1.rs");
                    }
                }
                pub mod types {
                    pub mod v1 {
                        include!("pb/runecraftstudios.pastello.game.types.v1.rs");
                    }
                }
            }
            pub mod web {
                pub mod auth {
                    pub mod v1 {
                        include!("pb/runecraftstudios.pastello.web.auth.v1.rs");
                    }
                }
                pub mod game {
                    pub mod v1 {
                        include!("pb/runecraftstudios.pastello.web.game.v1.rs");
                    }
                }
            }
        }
    }
}
// --- END CORRECTION ---


/// AppState holds the shared state for our Axum server
#[derive(Clone)]
struct AppState {
    command_registry: Arc<CommandRegistry>,
    token_validator: Arc<TokenValidator>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let config_path = "config.yaml";
    let config = load_config(config_path)
        .unwrap_or_else(|_| panic!("Failed to load config from {}", config_path));

    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // --- Dependency Injection ---
    
    let token_validator = TokenValidator::new(config.auth.clone())
        .await
        .unwrap_or_else(|e| {
            // Note: We've commented this out so startup doesn't hang
            // We'll add it back when the Go server is live.
            // panic!("Failed to fetch initial JWKS: {}", e);
            info!("Skipping JWKS fetch for DEV mode: {}", e);
            TokenValidator::new_without_refresh(config.auth.clone()) // We'll add this helper
        });

    // ... (rest of DI is unchanged) ...
    let repo = Arc::new(MemorySessionRepo::new());
    let bus = Arc::new(NopBus::new());
    let clock = Arc::new(SystemClock::new());
    let id_gen = Arc::new(UuidGen::new());
    let rng = Arc::new(ThreadRng::new());

    let engine_factory = Arc::new(EngineFactoryImpl::new(
        clock.clone(), rng.clone(), id_gen.clone(),
    ));
    
    let handle_uc = HandleGameCommandUseCase::new(
        repo.clone(), bus.clone(), engine_factory.clone(),
    );

    let game_service = Arc::new(GameService::new(handle_uc));

    let command_registry = Arc::new(CommandRegistry::new(game_service.clone()));

    let app_state = AppState {
        command_registry,
        token_validator: Arc::new(token_validator),
    };

    // --- Server Setup ---
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    let addr_str = "0.0.0.0:8080";
    let addr: SocketAddr = addr_str.parse()?;
    
    info!("Rust 'Game Room' WebSocket server listening on {}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await?;

    Ok(())
}