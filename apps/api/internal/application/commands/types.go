package commands

// App-level DTOs that the WS adapter produces from protobuf. Keeps protobuf out of app/domain.

// Trivia
type TriviaSubmitAnswer struct {
	PlayerID string
	Answer   string
}
type TriviaRevealHint struct{}

// Puzzle
type PuzzleMovePiece struct {
	FromX, FromY int
	ToX, ToY     int
}
type PuzzleUndoMove struct{}
