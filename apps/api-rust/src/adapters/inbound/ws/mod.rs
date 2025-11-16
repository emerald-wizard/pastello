// --- FIX: Make auth module public ---
pub mod auth;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
// --- FIX: Add futures_util::SinkExt ---
use futures_util::SinkExt;
// --- FIX: Use tokio_stream::StreamExt and alias it ---
//use tokio_stream::StreamExt as _;

// --- FIX: Import types from crate root ---
use crate::{
    // --- FIX: Removed unused application imports ---
    AppState,
    // --- FIX: Use full path for pb imports & CORRECT struct names ---
    pb::runecraftstudios::pastello::{
        web::game::v1::{
            ClientEnvelope, 
            client_envelope::Message as ClientMessage, // <-- FIX: Import oneof
            game_command_envelope::Command as GameCommandMessage, // <-- FIX: Import oneof
        },
        game::{
            types::v1::{GameType as PbGameType, PlayerId as PbPlayerId},
            puzzle::v1::{MovePieceCommand, UndoMoveCommand},
            trivia::v1::{SubmitAnswerCommand, RevealHintCommand},
        },
    },
    // --- FIX: Import from crate root ---
    GameCommand, GameService, StartGame,
};

use crate::{
    domain::game::{GameSessionID, PlayerID},
    // --- FIX: Removed unused EventBus import ---
    // ports::EventBus,
};


pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: AppState) {
    // --- FIX: Use stream.split() ---
    let (mut tx, mut rx) = stream.split();

    // --- Authentication Flow ---
    // 1. Expect a single text message containing the auth token
    let (user_id, session_id) = loop {
        // --- FIX: Disambiguate .next() call ---
        match tokio_stream::StreamExt::next(&mut rx).await {
            Some(Ok(Message::Text(token))) => {
                tracing::debug!("Received auth token via WebSocket");
                
                // --- FIX: Corrected syntax error (added let/match) ---
                let auth_result = state.token_validator.validate(&token).await
                    .map_err(|e| e.to_string()); // Convert errors to string

                match auth_result {
                    Ok(claims) => {
                        tracing::info!("Auth successful for sub: {}", claims.sub);
                        tx.send(Message::Text("AUTH_SUCCESS".to_string())).await.ok(); // Send success
                        break (claims.sub, claims.sid); // Return sub and sid
                    }
                    Err(e) => {
                        tracing::warn!("Auth failed: {}", e);
                        tx.send(Message::Text("AUTH_FAILED".to_string())).await.ok();
                        // Close the connection immediately on auth failure
                        return;
                    }
                }
            }
            Some(Ok(Message::Close(_))) => {
                tracing::info!("Client disconnected before auth");
                return;
            }
            Some(Err(e)) => {
                tracing::warn!("WebSocket error during auth: {}", e);
                return;
            }
            None => {
                tracing::info!("WebSocket stream ended during auth");
                return;
            }
            _ => {
                // Ignore other message types (Binary, Ping, Pong) during auth
                tracing::warn!("Received non-text message during auth");
            }
        }
    };
    
    tracing::info!("User {} connected with session {}", user_id, session_id);

    // --- Event Subscription ---
    // --- FIX: Use correct field name 'bus' ---
    let mut event_rx = state.bus.subscribe(&format!("session.{}", session_id)).await;

    // --- Bi-directional Communication Loops ---
    let user_id_clone = user_id.clone();
    let session_id_clone = session_id.clone();
    // --- FIX: Use correct field name 'game_service' ---
    let game_service_clone = state.game_service.clone();

    // Task to handle incoming messages (client -> server)
    let mut rx_task = tokio::spawn(async move {
        // --- FIX: Disambiguate .next() call ---
        while let Some(Ok(message)) = tokio_stream::StreamExt::next(&mut rx).await {
            match message {
                Message::Text(text) => {
                    tracing::warn!("Received unexpected text message post-auth: {}", text);
                }
                Message::Binary(bin) => {
                    handle_binary_message(
                        bin,
                        &user_id_clone,
                        &session_id_clone,
                        game_service_clone.as_ref(),
                    )
                    .await;
                }
                Message::Ping(data) => {
                    if tx.send(Message::Pong(data)).await.is_err() {
                        break; 
                    }
                }
                Message::Close(_) => {
                    tracing::info!("Client initiated disconnect for user {}", user_id_clone);
                    break;
                }
                _ => {}
            }
        }
        tracing::debug!("RX task for user {} ended", user_id_clone);
    });

    // Task to handle outgoing messages (server -> client)
    let mut tx_task = tokio::spawn(async move {
        // --- FIX: Disambiguate .next() call ---
        while let Some(event) = tokio_stream::StreamExt::next(&mut event_rx).await {
            // TODO: Translate `Box<dyn Any + Send>` event into a 
            // Protobuf `web::ServerEnvelope` and send as `Message::Binary`.
            
            tracing::info!("Got an event ({:?}), but translation is not implemented", event);
            let placeholder_msg = "EVENT_RECEIVED_PLACEHOLDER";
            if tx.send(Message::Text(placeholder_msg.to_string())).await.is_err() {
                break;
            }
        }
        tracing::debug!("TX task for user {} ended", user_id);
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut rx_task) => {
            tracing::debug!("RX task finished first. Aborting TX task.");
            tx_task.abort();
        }
        _ = (&mut tx_task) => {
            tracing::debug!("TX task finished first. Aborting RX task.");
            rx_task.abort();
        }
    }

    tracing::info!("WebSocket connection closed for user {}", user_id);
}

/// Handles incoming binary messages from the client
async fn handle_binary_message(
    bytes: Vec<u8>,
    user_id: &PlayerID,
    session_id: &GameSessionID,
    game_service: &GameService,
) {
    use prost::Message as _;
    // --- FIX: Removed unused pb imports ---

    // 1. Decode the outer envelope
    // --- FIX: Use correct struct name (no alias) ---
    match ClientEnvelope::decode(bytes.as_slice()) {
        Ok(envelope) => {
            // 2. Determine message type and dispatch
            match envelope.message {
                // --- FIX: Use correct oneof name ---
                Some(ClientMessage::StartGame(start_cmd)) => {
                    let game_type = match PbGameType::try_from(start_cmd.game_type) {
                        Ok(PbGameType::Puzzle) => crate::domain::game::GameType::Puzzle,
                        Ok(PbGameType::Trivia) => crate::domain::game::GameType::Trivia,
                        _ => {
                            tracing::warn!("Invalid game type received");
                            return;
                        }
                    };
                    
                    let cmd = StartGame {
                        game_type,
                        // --- FIX: Use correct PbPlayerId struct ---
                        player: PbPlayerId {
                            value: user_id.clone(),
                        },
                        session_id: session_id.clone(),
                    };
                    
                    if let Err(e) = game_service.start_game(cmd).await {
                        tracing::error!("Failed to start game: {}", e);
                    }
                }
                // --- FIX: Use correct oneof name ---
                Some(ClientMessage::GameCommand(cmd_envelope)) => {
                    let domain_command: Box<dyn std::any::Any + Send> = {
                        if let Some(inner) = cmd_envelope.command {
                            match inner {
                                // --- Puzzle Commands ---
                                // --- FIX: Use correct oneof name ---
                                GameCommandMessage::PuzzleMove(c) => {
                                    // --- FIX: Use correct struct name & pass session/player ---
                                    Box::new(crate::domain::puzzle::Command::MovePiece(
                                        MovePieceCommand {
                                            session_id: session_id.clone(),
                                            player_id: Some(PbPlayerId { value: user_id.clone() }),
                                            from_x: c.from_x, from_y: c.from_y,
                                            to_x: c.to_x, to_y: c.to_y,
                                        }
                                    ))
                                }
                                // --- FIX: Use correct oneof name ---
                                GameCommandMessage::PuzzleUndo(_) => {
                                    // --- FIX: Use correct struct name & pass session/player ---
                                    Box::new(crate::domain::puzzle::Command::UndoMove(
                                        UndoMoveCommand {
                                            session_id: session_id.clone(),
                                            player_id: Some(PbPlayerId { value: user_id.clone() }),
                                        }
                                    ))
                                }
                                
                                // --- Trivia Commands ---
                                // --- FIX: Use correct oneof name ---
                                GameCommandMessage::TriviaSubmit(c) => {
                                    // --- FIX: Use correct struct name & pass session ---
                                    Box::new(crate::domain::trivia::Command::SubmitAnswer(
                                        SubmitAnswerCommand {
                                            session_id: session_id.clone(),
                                            player_id: Some(PbPlayerId { value: user_id.clone() }),
                                            answer: c.answer,
                                        }
                                    ))
                                }
                                // --- FIX: Use correct oneof name ---
                                GameCommandMessage::TriviaHint(_) => {
                                    // --- FIX: Use correct struct name & pass session ---
                                    Box::new(crate::domain::trivia::Command::RevealHint(
                                        RevealHintCommand {
                                            session_id: session_id.clone(),
                                        }
                                    ))
                                }
                            }
                        } else {
                            tracing::warn!("Received empty GameCommand");
                            return;
                        }
                    };

                    let cmd = GameCommand {
                        session_id: session_id.clone(),
                        player_id: user_id.clone(),
                        command: domain_command,
                    };
                    
                    if let Err(e) = game_service.handle_command(cmd).await {
                        tracing::error!("Failed to handle game command: {}", e);
                    }
                }
                None => {
                    tracing::warn!("Received empty ClientEnvelope");
                }
            }
        }
        Err(e) => {
            tracing::warn!("Failed to decode ClientEnvelope: {}", e);
        }
    }
}