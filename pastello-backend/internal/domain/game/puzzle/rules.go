package puzzle

import "github.com/runecraft-studios/pastello/internal/domain/game"

type PuzzleRules struct {
	Difficulty       string
	AllowHints       bool
	TimeLimitSeconds int
	MaxPlayers       int
}

func (r PuzzleRules) Type() game.GameType { return game.GameTypePuzzle }
func (r PuzzleRules) Validate() error     { return nil }
