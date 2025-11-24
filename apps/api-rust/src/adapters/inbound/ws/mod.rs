pub mod auth;

// FIX: AuthError is now correctly in scope from auth.rs
use crate::adapters::inbound::ws::auth::{AuthError, Authenticator};
use crate::AppState;
use crate::domain::game::Session;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{State};
use axum::response::IntoResponse;
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

pub fn router() -> axum::Router<AppState> {
    // The route expects State<Arc<dyn Authenticator>> which is now derivable from AppState
    axum::Router::new().route("/ws/game", axum::routing::get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>, // Use _state since we only extract what's needed
    State(authenticator): State<Arc<dyn Authenticator>>, // This now works due to FromRef<AppState> in main.rs
) -> impl IntoResponse {
    info!("Upgrading WebSocket connection");
    ws.on_upgrade(move |socket| handle_socket(socket, _state, authenticator))
}

async fn handle_socket(stream: WebSocket, _state: AppState, authenticator: Arc<dyn Authenticator>) {
    info!("New WebSocket connection");
    let (mut tx, mut rx) = stream.split();

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
            // FIX: Use host_id instead of user_id
            info!("Auth successful for user: {}", session.host_id);
            tx.send(Message::Text("AUTH_SUCCESS".to_string().into()))
                .await
                .ok(); // Send success
            Some(session)
        }
        Ok(Err(e)) => {
            warn!("Auth failed: {:?}", e);
            tx.send(Message::Text("AUTH_FAILED".to_string().into()))
                .await
                .ok();
            None
        }
        Err(_) => {
            let e = AuthError::Timeout;
            warn!("Auth failed: {:?}", e);
            tx.send(Message::Text("AUTH_FAILED".to_string().into()))
                .await
                .ok();
            None
        }
    };

    if session.is_none() {
        warn!("Closing connection due to auth failure.");
        return;
    }

    let session = session.unwrap();
    // FIX: Use host_id instead of user_id
    let player_id = session.host_id.clone();

    // FIX (E0282): Explicitly type the Mutex contents
    let _game_state_stream = Arc::new(Mutex::new(None as Option<Box<dyn std::any::Any + Send>>));

    // Main game loop
    loop {
        tokio::select! {
            // Listen for messages from the client
            Some(msg) = rx.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        info!("Received message from {}: {}", player_id, text);
                        // TODO: Deserialize message, handle game logic
                        // _state.game_service.handle_command(...)
                    }
                    Ok(Message::Binary(_bin)) => {
                        info!("Received binary message from {}", player_id);
                        // TODO: Deserialize protobuf, handle game logic
                    }
                    Ok(Message::Close(_)) => {
                        info!("Connection closed by {}", player_id);
                        break;
                    }
                    Err(e) => {
                        error!("Error receiving message from {}: {:?}", player_id, e);
                        break;
                    }
                    _ => {} // Ignore other message types (Ping, Pong)
                }
            }
            // Listen for updates from the game state
            // Some(game_state) = game_state_stream.lock().await.next() => {
            //     // TODO: Serialize game state and send to client
            //     let payload = "TODO: serialize game state".to_string();
            //     if tx.send(Message::Text(payload)).await.is_err() {
            //         warn!("Failed to send game state to {}, connection closed?", player_id);
            //         break;
            //     }
            // }
            else => {
                break; // Both streams are closed
            }
        }
    }

    info!("WebSocket connection handler finished for {}", player_id);
    // TODO: Handle player disconnect logic (e.g., remove from game)
    // _state.game_service.handle_disconnect(player_id).await;
}
