pub mod auth;

use crate::adapters::inbound::ws::auth::{AuthError, Authenticator};
use crate::AppState;
use crate::domain::game::{Session, GameCommand, GameType};
use crate::pb::runecraftstudios::pastello::web::game::v1::{
    ClientEnvelope, client_envelope, 
    GameCommandEnvelope, game_command_envelope
};
use crate::pb::runecraftstudios::pastello::game::types::v1::GameSessionId;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{State};
use axum::response::IntoResponse;
use futures_util::{StreamExt, SinkExt};
use prost::Message as ProstMessage;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/ws/game", axum::routing::get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>, 
    State(authenticator): State<Arc<dyn Authenticator>>, 
) -> impl IntoResponse {
    info!("Upgrading WebSocket connection");
    ws.on_upgrade(move |socket| handle_socket(socket, state, authenticator))
}

async fn handle_socket(stream: WebSocket, state: AppState, authenticator: Arc<dyn Authenticator>) {
    info!("New WebSocket connection");
    let (mut tx, mut rx) = stream.split();

    // 1. Authentication Handshake
    let auth_result =
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            if let Some(Ok(Message::Text(token))) = rx.next().await {
                authenticator.authenticate(&token).await
            } else {
                Err(AuthError::NoToken)
            }
        })
        .await;

    let session: Option<Session> = match auth_result {
        Ok(Ok(session)) => {
            info!("Auth successful for user: {}", session.host_id);
            // Notify client of success
            // Note: Real impl would send a ServerEnvelope::AuthStatus
            tx.send(Message::Text("AUTH_SUCCESS".to_string().into())).await.ok();
            Some(session)
        }
        Ok(Err(e)) => {
            warn!("Auth failed: {:?}", e);
            tx.send(Message::Text("AUTH_FAILED".to_string().into())).await.ok();
            None
        }
        Err(_) => {
            let e = AuthError::Timeout;
            warn!("Auth failed: {:?}", e);
            tx.send(Message::Text("AUTH_FAILED".to_string().into())).await.ok();
            None
        }
    };

    if session.is_none() {
        return; // Close connection
    }

    let mut session = session.unwrap();
    let player_id = session.host_id.clone();

    // 2. CRITICAL: Persist the session so the Service can find it later
    // In a real app, StartGame would create it, but for this test flow we ensure it exists.
    if let Err(e) = state.game_service.force_save_session(session.clone()).await {
        error!("Failed to initialize session in repo: {}", e);
        return;
    }

    // 3. Main Game Loop
    loop {
        tokio::select! {
            Some(msg) = rx.next() => {
                match msg {
                    Ok(Message::Binary(bin)) => {
                        // Decode Protobuf
                        match ClientEnvelope::decode(&bin[..]) {
                            Ok(envelope) => {
                                handle_client_message(&state, &mut session, envelope).await;
                            }
                            Err(e) => error!("Failed to decode Protobuf: {}", e),
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("Connection closed by {}", player_id);
                        break;
                    }
                    Err(e) => {
                        error!("Error receiving message from {}: {:?}", player_id, e);
                        break;
                    }
                    _ => {} // Ignore Text/Ping/Pong for game logic
                }
            }
            else => break,
        }
    }

    info!("WebSocket connection handler finished for {}", player_id);
}

async fn handle_client_message(state: &AppState, session: &mut Session, env: ClientEnvelope) {
    match env.message {
        Some(client_envelope::Message::StartGame(cmd)) => {
            info!("Received StartGame command for type: {:?}", cmd.game_type);
            // Logic to switch session game type could go here
        },
        Some(client_envelope::Message::GameCommand(wrapper)) => {
            dispatch_game_command(state, session, wrapper).await;
        },
        None => warn!("Received empty envelope"),
    }
}

async fn dispatch_game_command(state: &AppState, session: &Session, wrapper: GameCommandEnvelope) {
    // Extract the inner command and convert to Box<dyn GameCommand>
    // Ideally this mapping logic lives in a mapper, but for the fix we put it here 
    // to bridge the gap immediately.
    
    let command: Option<Box<dyn GameCommand>> = match wrapper.command {
        Some(game_command_envelope::Command::PuzzleMove(cmd)) => Some(Box::new(cmd)),
        Some(game_command_envelope::Command::PuzzleUndo(cmd)) => Some(Box::new(cmd)),
        Some(game_command_envelope::Command::TriviaSubmit(cmd)) => Some(Box::new(cmd)),
        Some(game_command_envelope::Command::TriviaHint(cmd)) => Some(Box::new(cmd)),
        None => None,
    };

    if let Some(cmd) = command {
        // Use the session ID from the active socket session, 
        // effectively ignoring the one in the message if it differs (security)
        match state.game_service.handle_domain_command(&session.id, cmd).await {
            Ok(_) => info!("Command handled successfully"),
            Err(e) => error!("Failed to handle command: {:?}", e),
        }
    } else {
        warn!("Unknown or empty game command received");
    }
}