package trivia

import "github.com/runecraft-studios/pastello/internal/domain/game"

type TriviaRules struct {
	NumQuestions       int
	SecondsPerQuestion int
	NegativeMarking    bool
	Categories         []string
	MaxPlayers         int
}

func (r TriviaRules) Type() game.GameType { return game.GameTypeTrivia }
func (r TriviaRules) Validate() error     { return nil } // add bounds checks later
