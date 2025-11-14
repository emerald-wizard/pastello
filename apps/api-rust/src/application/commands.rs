// These are the DTOs that flow from the adapter to the use case.
// Based on internal/application/commands/types.go

// Puzzle
#[derive(Debug, Clone)]
pub struct PuzzleMovePiece {
    pub from_x: i32,
    pub from_y: i32,
    pub to_x: i32,
    pub to_y: i32,
}
#[derive(Debug, Clone)]
pub struct PuzzleUndoMove;

// Trivia
#[derive(Debug, Clone)]
pub struct TriviaSubmitAnswer {
    pub player_id: String,
    pub answer: String,
}
#[derive(Debug, Clone)]
pub struct TriviaRevealHint;

/// A simple enum to represent any possible application command
#[derive(Debug, Clone)]
pub enum AppCommand {
    PuzzleMovePiece(PuzzleMovePiece),
    PuzzleUndoMove(PuzzleUndoMove),
    TriviaSubmitAnswer(TriviaSubmitAnswer),
    TriviaRevealHint(TriviaRevealHint),
}