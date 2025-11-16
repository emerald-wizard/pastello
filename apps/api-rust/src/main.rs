use adapters::outbound::clock::SystemClock;
use adapters::outbound::eventbus::NopEventBus;
use adapters::outbound::id_gen::UuidGenerator;
use adapters::outbound::memory_repo::MemoryRepo;
use adapters::outbound::rng::ThreadRng;
use application::services::command_registry::CommandRegistry;
use application::services::engine_factory::EngineFactory;
// --- FIX: Use correct re-export from game_service module ---
pub use application::services::game_service::GameService;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// --- FIX: Import commands to re-export them ---
pub use application::commands::{GameCommand, StartGame};
// --- FIX: Use full, correct path ---
use crate::adapters::inbound::ws::auth::{RemoteJwkValidator, TokenValidator};
use crate::adapters::inbound::ws::websocket_handler;
use crate::config::Config;
// --- FIX: Import StreamExt to bring .split() into scope ---
//use futures_util::StreamExt;
use crate::ports::EventBus;

mod adapters;
mod application;
mod config;
mod domain;
mod ports;

// --- Protobuf Module ---
// This block includes all auto-generated .rs files from the /pb directory
#[allow(clippy::all)]
pub mod pb {
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
// --- End Protobuf Module ---

#[derive(Clone)]
pub struct AppState {
    pub command_registry: Arc<CommandRegistry>,
    pub token_validator: Arc<dyn TokenValidator>,
    // --- FIX: Add missing fields ---
    pub game_service: Arc<GameService>,
    pub bus: Arc<dyn EventBus>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load("config.yaml").expect("Failed to load config");

    // --- Create Dependencies (Ports/Adapters) ---
    let clock = Arc::new(SystemClock);
    let id_gen = Arc::new(UuidGenerator);
    let rng = Arc::new(ThreadRng);
    let game_repo = Arc::new(MemoryRepo::new());
    // --- FIX: Use bus ---
    let bus = Arc::new(NopEventBus::new());
    let engine_factory = Arc::new(EngineFactory::new(
        clock.clone(),
        rng.clone(),
        id_gen.clone(),
    ));

    // --- Create Services ---
    let command_registry = Arc::new(CommandRegistry::new(engine_factory.clone()));
    // --- FIX: Pass all required dependencies ---
    let game_service = Arc::new(GameService::new(
        game_repo.clone(),
        bus.clone(),
        command_registry.clone(),
    ));

    // --- Create Auth Validator ---
    let token_validator = Arc::new(RemoteJwkValidator::new(
        config.auth.jwks_url.clone(),
    ));

    // --- Create AppState ---
    let state = AppState {
        command_registry: command_registry.clone(),
        token_validator: token_validator.clone(),
        // --- FIX: Initialize missing fields ---
        game_service: game_service.clone(),
        bus: bus.clone(),
    };

    // --- Create Router ---
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::debug!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}