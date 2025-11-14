use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use prost::Message as ProstMessage;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tracing::{error, info, warn};

// Import our auth structs
use crate::adapters::inbound::ws::auth::GameTicketClaims;

// Import all the types we need
use crate::{
    main::AppState,
    pb::web::game::v1 as web,
    application::{
        commands::AppCommand,
        usecase::handle_game_command::AppCommandResponse,
    },
    domain::game::{GameSessionID, GameType, PlayerID},
};

// Declare the auth.rs file as a submodule
pub mod auth;

/// Axum entrypoint for handling the WebSocket upgrade request.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}


/// Main function to handle a single WebSocket connection.
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    info!("WebSocket client connecting...");

    // 1. --- AUTHENTICATION STEP ---
    // The first message MUST be a text message containing the JWT "Game Ticket"
    let claims = match socket.recv().await {
        Some(Ok(Message::Text(token))) => {
            
            // --- THIS IS OUR NEW DEV MODE BYPASS ---
            if token.starts_with("DEV::") {
                info!("Using DEV MODE authentication");
                let parts: Vec<&str> = token.split("::").collect();
                if parts.len() == 3 {
                    // Manually create the claims our app needs
                    Ok(GameTicketClaims {
                        sub: parts[1].to_string(), // user_id
                        sid: parts[2].to_string(), // session_id
                        // Fill in dummy data for the rest
                        aud: "dev-mode".to_string(),
                        iss: "dev-mode".to_string(),
                        exp: 0, // Not validated in dev mode
                    })
                } else {
                    Err(anyhow::anyhow!("Invalid DEV:: token format"))
                }
            } else {
                // --- This is the normal, secure production path ---
                info!("Using PRODUCTION authentication (JWKS)");
                state.token_validator.validate_token(&token).await
            }
            // --- END OF DEV MODE BYPASS ---
            
        }
        .map_err(|e| e.to_string()) // Convert errors to string for logging
        {
            Ok(claims) => {
                info!(user_id = %claims.sub, session_id = %claims.sid, "Client authenticated");
                claims
            },
            Err(e) => {
                error!("Authentication failed: {}", e);
                let err_env = build_error_envelope("auth-fail", "AUTH_ERROR", "Invalid game ticket");
                let _ = socket.send(Message::Binary(err_env.encode_to_vec())).await;
                socket.close().await.ok();
                return;
            }
        },
        _ => {
            // (Client failed to send any token)
            error!("Client failed to send auth token");
            let err_env = build_error_envelope("auth-fail", "AUTH_REQUIRED", "First message must be auth token");
            let _ = socket.send(Message::Binary(err_env.encode_to_vec())).await;
            socket.close().await.ok();
            return;
        }
    };
    
    // Auth was successful. Store the authenticated IDs.
    let authenticated_session_id = claims.sid;
    let authenticated_player_id = claims.sub; 

    // 2. --- GAME LOOP ---
    // Now we enter the normal message processing loop.
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(Message::Binary(bin)) => bin,
            Ok(Message::Close(_)) => {
                info!(session_id = %authenticated_session_id, "Client disconnected");
                break;
            }
            Ok(m) => {
                warn!(session_id = %authenticated_session_id, "Received non-binary message: {:?}", m);
                continue;
            }
            Err(e) => {
                error!(session_id = %authenticated_session_id, "WebSocket error: {}", e);
                break;
            }
        };

        // 2a. Decode Envelope
        let env = match web::Envelope::decode(msg.as_slice()) {
            Ok(env) => env,
            Err(e) => {
                error!(session_id = %authenticated_session_id, "Failed to decode envelope: {}", e);
                continue;
            }
        };

        let correlation_id = env.correlation_id.clone();
        
        // 2b. Route
        let response_payload = route_envelope(
            &env,
            &state,
            &authenticated_session_id,
            &authenticated_player_id
        ).await;

        // 2c. Send Reply
        let reply_envelope = match response_payload {
            Ok(Some(payload)) => {
                Some(build_reply_envelope(&correlation_id, payload))
            }
            Ok(None) => None, // No reply needed
            Err(e) => {
                error!(session_id = %authenticated_session_id, "Handler error: {}", e);
                Some(build_error_envelope(&correlation_id, "HANDLER_ERROR", &e.to_string()))
            }
        };
        
        if let Some(reply) = reply_envelope {
            let mut buf = Vec::new();
            if reply.encode(&mut buf).is_ok() {
                if socket.send(Message::Binary(buf)).await.is_err() {
                    error!(session_id = %authenticated_session_id, "Failed to send reply, client disconnected.");
                    break;
                }
            }
        }
    }
    info!(session_id = %authenticated_session_id, "WebSocket connection closed.");
}

/// Routes an incoming envelope to the correct application service.
async fn route_envelope(
    env: &web::Envelope,
    state: &AppState,
    session_id: &GameSessionID,
    player_id: &PlayerID,
) -> anyhow::Result<Option<AppCommandResponse>> {
    
    let app_cmd = to_app_command(env)
        .ok_or_else(|| anyhow::anyhow!("unhandled or invalid command body"))?;

    // Pass the authenticated IDs and the command to the registry
    let payload = state.command_registry.handle(
        session_id.clone(),
        player_id.clone(),
        app_cmd
    ).await?;
    
    Ok(Some(payload))
}

/// Translates a Protobuf envelope body into an internal ApplicationCommand.
fn to_app_command(env: &web::Envelope) -> Option<AppCommand> {
    match env.body.as_ref()? {
        web::envelope::Body::PuzzleMovePiece(c) => Some(AppCommand::PuzzleMovePiece(
            crate::application::commands::PuzzleMovePiece {
                from_x: c.from_x,
                from_y: c.from_y,
                to_x: c.to_x,
                to_y: c.to_y,
            }
        )),
        web::envelope::Body::PuzzleUndoMove(_) => Some(AppCommand::PuzzleUndoMove(
            crate::application::commands::PuzzleUndoMove
        )),
        web::envelope::Body::TriviaSubmitAnswer(c) => Some(AppCommand::TriviaSubmitAnswer(
            crate::application::commands::TriviaSubmitAnswer {
                // The use case will validate this against the token's player_id
                player_id: c.player_id.as_ref().map_or("", |id| &id.value).to_string(),
                answer: c.answer.clone(),
            }
        )),
        web::envelope::Body::TriviaRevealHint(_) => Some(AppCommand::TriviaRevealHint(
            crate::application::commands::TriviaRevealHint
        )),
        // All other message types are ignored
        _ => None,
    }
}

/// Builds a reply envelope from an application response.
fn build_reply_envelope(corr_id: &str, payload: AppCommandResponse) -> web::Envelope {
    use crate::pb::game::{puzzle::v1 as puzzle, trivia::v1 as trivia};

    let body = match payload {
        AppCommandResponse::PuzzlePieceMoved { from_x, from_y, to_x, to_y } => {
            web::envelope::Body::PuzzlePieceMoved(puzzle::PieceMovedEvent {
                session_id: None, // We could pass session_id down if needed
                from_x, from_y, to_x, to_y,
            })
        }
        AppCommandResponse::PuzzleMoveUndone => {
            web::envelope::Body::PuzzleMoveUndone(puzzle::MoveUndoneEvent {
                session_id: None,
            })
        }
        AppCommandResponse::TriviaAnswerAccepted { delta, total } => {
            web::envelope::Body::TriviaAnswerAccepted(trivia::AnswerAcceptedEvent {
                session_id: None,
                player_id: None, // We could pass player_id down if needed
                delta_score: delta,
                total_score: total,
            })
        }
        AppCommandResponse::TriviaHintRevealed { hint } => {
            web::envelope::Body::TriviaHintRevealed(trivia::HintRevealedEvent {
                session_id: None,
                hint_text: hint,
            })
        }
        AppCommandResponse::NoReply => return web::Envelope {
            correlation_id: corr_id.to_string(),
            body: None,
        }
    };
    
    web::Envelope {
        correlation_id: corr_id.to_string(),
        body: Some(body),
    }
}

/// Builds an error envelope.
fn build_error_envelope(corr_id: &str, code: &str, msg: &str) -> web::Envelope {
    web::Envelope {
        correlation_id: corr_id.to_string(),
        body: Some(web::envelope::Body::Error(web::ErrorEvent {
            code: code.to_string(),
            message: msg.to_string(),
        })),
    }
}