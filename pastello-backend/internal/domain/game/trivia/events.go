package trivia

import "github.com/runecraft-studios/pastello/internal/domain/game"

// Domain events emitted by the Trivia engine.

type AnswerAccepted struct {
	game.EventMeta
	SessionID game.GameSessionID
	PlayerID  game.PlayerID
	Delta     int
	Total     int
}

func (AnswerAccepted) Name() string { return "trivia.answer_accepted" }

type HintRevealed struct {
	game.EventMeta
	SessionID game.GameSessionID
	Hint      string
}

func (HintRevealed) Name() string { return "trivia.hint_revealed" }
