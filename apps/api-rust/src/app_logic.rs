use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use tracing::{info, error};
// 1. ADDED: Time dependency for clock port
use std::time::{SystemTime, UNIX_EPOCH};

// --- New Domain Models (from puzzle/models.go) ---

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
}

#[derive(Debug, Clone, Default)]
pub struct PuzzleState {
    pub width: i32,
    pub height: i32,
    pub board: Vec<i32>, // The actual board state
    pub history: Vec<Move>, // For undo
}
impl PuzzleState {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            // TODO: Initialize board properly
            board: (0..(width * height)).collect(),
            history: Vec::new(),
        }
    }
}

// TODO: Define TriviaState
#[derive(Debug, Clone, Default)]
pub struct TriviaState {
    // ... fields for trivia
}

// This enum makes our Session type-safe
#[derive(Debug, Clone)]
pub enum GameState {
    None,
    Puzzle(PuzzleState),
    Trivia(TriviaState),
}

// --- Domain Types (Modified) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    Unspecified,
    Puzzle,
    Trivia,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub game_type: GameType,
    pub host_id: String,
    pub version: u64,
    // 2. MODIFIED: `board` is now part of a state enum
    pub game_state: GameState,
}
impl Session {
    pub fn new(id: String, host_id: String, game_type: GameType) -> Self {
        // 3. MODIFIED: Initialize game-specific state
        let game_state = match game_type {
            GameType::Puzzle => GameState::Puzzle(PuzzleState::new(4, 4)), // Default 4x4
            GameType::Trivia => GameState::Trivia(TriviaState::default()),
            _ => GameState::None,
        };

        Session {
            id,
            host_id,
            game_type,
            version: 1, // Start at version 1
            game_state,
        }
    }
}

// ... (AppError is the same) ...
#[derive(Debug)]
pub enum AppError {
    SessionNotFound,
    RepoError(String),
    EngineError(String),
    CommandNotSupported,
    NoEngine,
    StartGameError(String),
    // 4. ADDED: More specific error
    InvalidMove(String),
}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for AppError {}


// --- Domain Events (Modified) ---
#[async_trait::async_trait]
pub trait DomainEvent: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;
}

#[derive(Debug)]
pub struct PieceMoved {
    pub from_x: i32,
    pub from_y: i32,
    pub to_x: i32,
    pub to_y: i32,
}
#[async_trait::async_trait]
impl DomainEvent for PieceMoved {
    fn name(&self) -> &'static str { "puzzle.PieceMoved" }
}

// 5. ADDED: New event from engine.go
#[derive(Debug)]
pub struct MoveUndone;
#[async_trait::async_trait]
impl DomainEvent for MoveUndone {
    fn name(&self) -> &'static str { "puzzle.MoveUndone" }
}
// ... (Define other events like AnswerAccepted, etc.)

// --- Ports (Go Interfaces -> Rust Traits) ---

// 6. ADDED: New ports required by the engine
#[async_trait::async_trait]
pub trait Clock: Send + Sync {
    async fn now(&self) -> SystemTime;
}
#[async_trait::async_trait]
pub trait RNG: Send + Sync {
    async fn shuffle(&self, board: &mut [i32]);
}
#[async_trait::async_trait]
pub trait IDGen: Send + Sync {
    async fn new_id(&self) -> String;
}

// ... (Engine, EngineFactory, GameSessionRepo, EventBus traits are the same) ...
#[async_trait::async_trait]
pub trait Engine: Send + Sync {
    async fn apply(
        &self,
        session: Session,
        cmd: EngineCommand,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), AppError>;
}
pub trait EngineFactory: Send + Sync {
    fn for_type(&self, game_type: GameType) -> Option<Arc<dyn Engine>>;
}
#[async_trait::async_trait]
pub trait GameSessionRepo: Send + Sync {
    async fn load(&self, session_id: &str) -> Result<Session, AppError>;
    async fn save(&self, session: &Session) -> Result<(), AppError>;
}
#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), AppError>;
}


// --- `StartGameSession` Use Case (Unchanged) ---
// ... (StartGameDeps, StartGameSessionIn, StartGameSessionOut, StartGameSession struct) ...
pub struct StartGameDeps {
    pub repo: Arc<dyn GameSessionRepo>,
}
pub struct StartGameSessionIn {
    pub host_id: String,
    pub game_type: GameType,
}
pub struct StartGameSessionOut {
    pub session: Session,
}
pub struct StartGameSession {
    deps: Arc<StartGameDeps>,
}
impl StartGameSession {
    pub fn new(deps: Arc<StartGameDeps>) -> Self {
        Self { deps }
    }
    pub async fn execute(
        &self,
        _ctx: (), 
        input: StartGameSessionIn,
    ) -> Result<StartGameSessionOut, AppError> {
        let session_id = Uuid::new_v4().to_string();
        info!("Creating new session {} for host {}", session_id, input.host_id);
        let session = Session::new(session_id, input.host_id, input.game_type);
        self.deps.repo.save(&session).await?;
        Ok(StartGameSessionOut { session })
    }
}


// --- `HandleGameCommand` Use Case (Unchanged) ---
// ... (ApplicationCommand, ReplyPayload, EngineCommand, etc.) ...
#[derive(Debug)]
pub enum ApplicationCommand {
    PuzzleMovePiece(PuzzleMovePieceCommand),
    PuzzleUndoMove,
    TriviaSubmitAnswer(TriviaSubmitAnswerCommand),
    TriviaRevealHint,
}
#[derive(Debug)]
pub struct PuzzleMovePieceCommand {
    pub from_x: i32,
    pub from_y: i32,
    pub to_x: i32,
    pub to_y: i32,
}
#[derive(Debug)]
pub struct TriviaSubmitAnswerCommand {
    pub player_id: String,
    pub answer: String,
}
#[derive(Debug)]
pub enum ReplyPayload {
    PuzzlePieceMoved {
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
    },
    // 7. ADDED: Reply for undo
    PuzzleMoveUndone,
}
#[derive(Debug)]
pub enum EngineCommand {
    Puzzle(PuzzleEngineCommand),
    Trivia(TriviaEngineCommand),
}
#[derive(Debug)]
pub enum PuzzleEngineCommand {
    MovePiece {
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
    },
    UndoMove,
}
#[derive(Debug)]
pub enum TriviaEngineCommand {
    SubmitAnswer { player_id: String, answer: String },
    RevealHint,
}
// ... (HandleGameCommandIn, HandleGameCommandOut, HandleDeps, HandleGameCommand struct) ...
pub struct HandleGameCommandIn {
    pub session_id: String,
    pub command: ApplicationCommand,
}
pub struct HandleGameCommandOut {
    pub payload: Option<ReplyPayload>,
}
pub struct HandleDeps {
    pub repo: Arc<dyn GameSessionRepo>,
    pub bus: Arc<dyn EventBus>,
    pub engines: Arc<dyn EngineFactory>,
}
pub struct HandleGameCommand {
    deps: Arc<HandleDeps>,
}
impl HandleGameCommand {
    pub fn new(deps: Arc<HandleDeps>) -> Self {
        Self { deps }
    }

    pub async fn execute(
        &self,
        _ctx: (),
        input: HandleGameCommandIn,
    ) -> Result<HandleGameCommandOut, AppError> {
        // 1) Load session
        let session = self.deps.repo.load(&input.session_id).await?;
        if session.game_type == GameType::Unspecified {
            return Err(AppError::EngineError("session has no game type".to_string()));
        }
        // 2) Select engine
        let engine = self.deps.engines.for_type(session.game_type)
            .ok_or(AppError::NoEngine)?;
        // 3) Map app command -> engine command
        let engine_cmd = Self::map_to_engine_cmd(session.game_type, input.command)?;
        // 4) Apply
        let (next_session, events) = engine.apply(session, engine_cmd).await?;
        // 5) Persist
        self.deps.repo.save(&next_session).await?;
        // 6) Publish
        for e in events {
            self.deps.bus.publish(e).await.unwrap_or_else(|e| {
                error!("Failed to publish event: {:?}", e);
            });
        }
        // 7) Return DTO payload
        Ok(HandleGameCommandOut {
            payload: Self::flatten(events),
        })
    }

    fn map_to_engine_cmd(
        game_type: GameType,
        cmd: ApplicationCommand,
    ) -> Result<EngineCommand, AppError> {
        match game_type {
            GameType::Puzzle => match cmd {
                ApplicationCommand::PuzzleMovePiece(c) => Ok(EngineCommand::Puzzle(
                    PuzzleEngineCommand::MovePiece {
                        from_x: c.from_x,
                        from_y: c.from_y,
                        to_x: c.to_x,
                        to_y: c.to_y,
                    },
                )),
                // 8. ADDED: Handle undo
                ApplicationCommand::PuzzleUndoMove => {
                    Ok(EngineCommand::Puzzle(PuzzleEngineCommand::UndoMove))
                }
                _ => Err(AppError::CommandNotSupported),
            },
            GameType::Trivia => match cmd {
                // ... (Trivia cases are the same) ...
                ApplicationCommand::TriviaSubmitAnswer(c) => Ok(EngineCommand::Trivia(
                    TriviaEngineCommand::SubmitAnswer {
                        player_id: c.player_id,
                        answer: c.answer,
                    },
                )),
                ApplicationCommand::TriviaRevealHint => {
                    Ok(EngineCommand::Trivia(TriviaEngineCommand::RevealHint))
                }
                _ => Err(AppError::CommandNotSupported),
            },
            _ => Err(AppError::CommandNotSupported),
        }
    }

    fn flatten(events: Vec<Box<dyn DomainEvent>>) -> Option<ReplyPayload> {
        for e in events {
            if let Some(moved) = e.downcast_ref::<PieceMoved>() {
                return Some(ReplyPayload::PuzzlePieceMoved {
                    from_x: moved.from_x,
                    from_y: moved.from_y,
                    to_x: moved.to_x,
                    to_y: moved.to_y,
                });
            }
            // 9. ADDED: Handle MoveUndone event
            if e.downcast_ref::<MoveUndone>().is_some() {
                return Some(ReplyPayload::PuzzleMoveUndone);
            }
        }
        None
    }
}

// --- 10. REPLACED: Stubs with Real Engine ---

// Stub Repo (Unchanged)
pub struct MemRepo {
    pub sessions: RwLock<HashMap<String, Session>>,
}
#[async_trait::async_trait]
impl GameSessionRepo for MemRepo {
    async fn load(&self, session_id: &str) -> Result<Session, AppError> {
        self.sessions.read().await.get(session_id).cloned().ok_or(AppError::SessionNotFound)
    }
    async fn save(&self, session: &Session) -> Result<(), AppError> {
        self.sessions.write().await.insert(session.id.clone(), session.clone());
        Ok(())
    }
}

// Stub Event Bus (Unchanged)
pub struct LogBus;
#[async_trait::async_trait]
impl EventBus for LogBus {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), AppError> {
        info!("[EventBus] Published: {:?}", event);
        Ok(())
    }
}

// 11. ADDED: Stubs for new ports
pub struct StubClock;
#[async_trait::async_trait]
impl Clock for StubClock {
    async fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}
pub struct StubRNG;
#[async_trait::async_trait]
impl RNG for StubRNG {
    async fn shuffle(&self, _board: &mut [i32]) {
        // TODO: Implement real shuffle
        info!("[RNG] Shuffling board (stub)");
    }
}
pub struct StubIDGen;
#[async_trait::async_trait]
impl IDGen for StubIDGen {
    async fn new_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}


// --- Real Puzzle Engine (Translated from Go) ---

// Deps required by the puzzle engine
pub struct PuzzleDeps {
    pub clock: Arc<dyn Clock>,
    pub rng: Arc<dyn RNG>,
    pub idgen: Arc<dyn IDGen>,
}

// The stateless puzzle engine
pub struct PuzzleEngine {
    deps: Arc<PuzzleDeps>,
}

impl PuzzleEngine {
    pub fn new(deps: Arc<PuzzleDeps>) -> Self {
        Self { deps }
    }

    // --- Internal Engine Logic (Helpers) ---

    // `movePiece` helper from Go, adapted for Rust
    async fn move_piece(
        &self,
        session: &mut Session,
        from_x: i32, from_y: i32,
        to_x: i32, to_y: i32
    ) -> Result<Box<dyn DomainEvent>, AppError> {
        
        // Extract the mutable puzzle state
        let puzzle_state = match &mut session.game_state {
            GameState::Puzzle(state) => Ok(state),
            _ => Err(AppError::EngineError("Session is not a puzzle game".to_string())),
        }?;

        // 1. Validation (from `inBounds`)
        if !self.in_bounds(puzzle_state, from_x, from_y) || !self.in_bounds(puzzle_state, to_x, to_y) {
            return Err(AppError::InvalidMove("Move out of bounds".to_string()));
        }

        // 2. TODO: Implement real puzzle logic
        // (e.g., check if move is valid, slide tile, update `puzzle_state.board`)
        info!("[PuzzleEngine] Applying move ({},{}) -> ({},{})", from_x, from_y, to_x, to_y);


        // 3. Record move (from Go logic)
        puzzle_state.history.push(Move {
            from: Pos { x: from_x, y: from_y },
            to: Pos { x: to_x, y: to_y },
        });

        // 4. Create event
        // We use the deps.clock here if we add metadata to the event
        // let _now = self.deps.clock.now().await; 
        let event = PieceMoved {
            from_x, from_y, to_x, to_y,
        };
        Ok(Box::new(event))
    }

    // `undoMove` helper from Go, adapted for Rust
    async fn undo_move(
        &self,
        session: &mut Session,
    ) -> Result<Box<dyn DomainEvent>, AppError> {

        let puzzle_state = match &mut session.game_state {
            GameState::Puzzle(state) => Ok(state),
            _ => Err(AppError::EngineError("Session is not a puzzle game".to_string())),
        }?;

        // 1. Check history
        if puzzle_state.history.is_empty() {
            return Err(AppError::InvalidMove("Nothing to undo".to_string()));
        }

        // 2. Pop last move
        let _last_move = puzzle_state.history.pop();
        
        // 3. TODO: Update `puzzle_state.board` based on the undo
        info!("[PuzzleEngine] Undoing last move");

        // 4. Create event
        let event = MoveUndone;
        Ok(Box::new(event))
    }

    // `inBounds` helper from Go
    fn in_bounds(&self, state: &PuzzleState, x: i32, y: i32) -> bool {
        x >= 0 && x < state.width && y >= 0 && y < state.height
    }
}

#[async_trait::async_trait]
impl Engine for PuzzleEngine {
    async fn apply(
        &self,
        mut session: Session, // Take ownership, make mutable
        cmd: EngineCommand,
    ) -> Result<(Session, Vec<Box<dyn DomainEvent>>), AppError> {
        
        // 1. Check if this engine can handle this command
        let puzzle_cmd = match cmd {
            EngineCommand::Puzzle(c) => c,
            _ => return Err(AppError::EngineError("Wrong engine for command".to_string())),
        };

        // 2. Dispatch to internal helpers
        let event = match puzzle_cmd {
            PuzzleEngineCommand::MovePiece { from_x, from_y, to_x, to_y } => {
                self.move_piece(&mut session, from_x, from_y, to_x, to_y).await?
            }
            PuzzleEngineCommand::UndoMove => {
                self.undo_move(&mut session).await?
            }
        };

        // 3. Update session version and return
        session.version += 1;
        Ok((session, vec![event]))
    }
}


// --- Real Engine Factory ---

// 12. REPLACED: StubEngineFactory with RealEngineFactory
pub struct RealEngineFactory {
    puzzle: Arc<dyn Engine>,
    // trivia: Arc<dyn Engine>, // TODO: Add trivia engine
}
impl RealEngineFactory {
    // 13. MODIFIED: `new` now takes engine-specific deps
    pub fn new(puzzle_deps: Arc<PuzzleDeps>) -> Self {
        Self {
            puzzle: Arc::new(PuzzleEngine::new(puzzle_deps)),
            // TODO: trivia: Arc::new(TriviaEngine::new(...))
        }
    }
}
impl EngineFactory for RealEngineFactory {
    fn for_type(&self, game_type: GameType) -> Option<Arc<dyn Engine>> {
        match game_type {
            GameType::Puzzle => Some(self.puzzle.clone()),
            _ => None, // TODO: Return trivia engine
        }
    }
}