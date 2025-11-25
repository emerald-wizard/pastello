// --- MODULE DECLARATIONS ---
pub mod adapters;
pub mod application;
pub mod config;
pub mod domain;
pub mod pb;
pub mod ports;

// --- APPSTATE (Composition Root) ---
use crate::application::services::game_service::GameService;
use crate::adapters::inbound::ws::auth::Authenticator; 
use std::sync::Arc;
use axum::extract::FromRef; 

#[derive(Clone)]
pub struct AppState {
    pub game_service: Arc<GameService>,
    pub authenticator: Arc<dyn Authenticator>, 
}

impl AppState {
    pub fn new(game_service: Arc<GameService>, authenticator: Arc<dyn Authenticator>) -> Self {
        Self { game_service, authenticator }
    }
}

// Implement FromRef for Authenticator to be extracted from AppState
impl FromRef<AppState> for Arc<dyn Authenticator> {
    fn from_ref(state: &AppState) -> Self {
        state.authenticator.clone()
    }
}

// --- IMPORTS ---
use crate::{
    adapters::outbound::{
        clock::SystemClock, eventbus::NopEventBus, id_gen::UuidGenerator,
        memory_repo::MemoryRepo,
        rng::SystemRng,
    },
    application::services::{
        command_registry::CommandRegistry, engine_factory::DefaultEngineFactory, 
    },
    config::load_config,
};
use adapters::inbound::ws::router as ws_router;
use axum::{
    routing::get, 
    Router,
};
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate::adapters::inbound::ws::auth::StubAuthenticator;

#[tokio::main]
async fn main() {
    // --- Tracing & Config Setup ---
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Loading configuration...");
    let config = load_config().expect("Failed to load config");
    let server_addr = config.server.host.clone();
    let server_port = config.server.port.clone();

    // --- Dependency Injection (Adapters) ---
    let game_repo = Arc::new(MemoryRepo::new());
    let event_bus = Arc::new(NopEventBus::new());
    let clock = Arc::new(SystemClock::new());
    let id_gen = Arc::new(UuidGenerator::new());
    let rng = Arc::new(SystemRng::new());
    
    // Initialize authenticator
    let authenticator: Arc<dyn Authenticator> = Arc::new(StubAuthenticator::new( 
        &config.firebase.user,
    ));

    // --- Dependency Injection (Services) ---
    // NOTE: For trait objects used in AppState or shared, we must specify Send/Sync/'static bounds.
    let engine_factory = Arc::new(DefaultEngineFactory::new(
        clock.clone() as Arc<dyn ports::Clock + Send + Sync>, 
        rng.clone() as Arc<dyn ports::Rng + Send + Sync>,
    ));
    
    // Pass engine_factory to CommandRegistry::new()
    let command_registry = Arc::new(CommandRegistry::new(engine_factory.clone()));

    let game_service = Arc::new(GameService::new(
        game_repo.clone(),
        event_bus.clone(),
        clock.clone(),
        id_gen.clone(),
        engine_factory.clone(),
        command_registry.clone(),
    ));

    // --- State Construction ---
    // Pass authenticator to AppState::new
    let app_state = AppState::new(game_service, authenticator); 
    
    // --- Router Setup ---
    let app = Router::new()
        .route("/", get(|| async { "Pastello API is running" }))
        .nest("/v1", ws_router())
        .with_state(app_state);

    // --- Server Start ---
    let addr = format!("{}:{}", server_addr, server_port)
        .parse::<SocketAddr>()
        .expect("Invalid server address");
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}