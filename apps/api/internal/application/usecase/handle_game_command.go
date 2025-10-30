package usecase

import (
	"context"
	"errors"

	"github.com/runecraft-studios/pastello/apps/api/internal/application/commands"
	"github.com/runecraft-studios/pastello/apps/api/internal/domain/game"
	"github.com/runecraft-studios/pastello/apps/api/internal/domain/game/puzzle"
	"github.com/runecraft-studios/pastello/apps/api/internal/domain/game/trivia"
	"github.com/runecraft-studios/pastello/apps/api/internal/ports"
)

type HandleDeps struct {
	Repo    ports.GameSessionRepo
	Bus     ports.EventBus
	Engines interface {
		For(t game.GameType) game.Engine
	}
	// Clock ports.Clock // <- remove if truly unused
}

type HandleGameCommandIn struct {
	SessionID game.GameSessionID
	Command   any // one of commands.TriviaSubmitAnswer, commands.PuzzleMovePiece, ...
}
type HandleGameCommandOut struct {
	Payload any // tiny DTO your ws translate.go understands
}

type HandleGameCommand struct{ d HandleDeps }

func NewHandleGameCommand(d HandleDeps) HandleGameCommand { return HandleGameCommand{d: d} }

func (uc HandleGameCommand) Execute(ctx context.Context, in HandleGameCommandIn) (HandleGameCommandOut, error) {
	// 1) Load session
	sess, _, err := uc.d.Repo.Load(ctx, in.SessionID)
	if err != nil {
		return HandleGameCommandOut{}, err
	}
	if sess.Type == game.GameTypeUnspecified {
		return HandleGameCommandOut{}, errors.New("session has no game type")
	}

	// 2) Select engine
	eng := uc.d.Engines.For(sess.Type)
	if eng == nil {
		return HandleGameCommandOut{}, errors.New("no engine for session type")
	}

	// 3) Map app command → engine command
	ecmd := mapToEngineCmd(sess.Type, in.Command)
	if ecmd == nil {
		return HandleGameCommandOut{}, errors.New("unsupported command for session type")
	}

	// 4) Apply
	next, events, err := eng.Apply(ctx, sess, ecmd)
	if err != nil {
		return HandleGameCommandOut{}, err
	}

	// 5) Persist
	var snapshot any
	if err := uc.d.Repo.Save(ctx, next, snapshot); err != nil {
		return HandleGameCommandOut{}, err
	}

	// 6) Publish (simple pass-through)
	for _, e := range events {
		if uc.d.Bus != nil {
			_ = uc.d.Bus.Publish(ctx, e.Name(), e)
		}
	}

	// 7) Return tiny payload for WS layer
	return HandleGameCommandOut{Payload: flatten(events)}, nil
}

// --- helpers ---

func mapToEngineCmd(t game.GameType, cmd any) any {
	switch t {
	case game.GameTypePuzzle:
		switch c := cmd.(type) {
		case commands.PuzzleMovePiece:
			return puzzle.Cmd{MovePiece: &puzzle.MovePiece{FromX: c.FromX, FromY: c.FromY, ToX: c.ToX, ToY: c.ToY}}
		case commands.PuzzleUndoMove:
			return puzzle.Cmd{UndoMove: &puzzle.UndoMove{}}
		}
	case game.GameTypeTrivia:
		switch c := cmd.(type) {
		case commands.TriviaSubmitAnswer:
			return trivia.Cmd{SubmitAnswer: &trivia.SubmitAnswer{PlayerID: c.PlayerID, Answer: c.Answer}}
		case commands.TriviaRevealHint:
			return trivia.Cmd{RevealHint: &trivia.RevealHint{}}
		}
	}
	return nil
}

// flatten: keep returning the first event as before; if you want multiple,
// change return type to []any and adjust ws translate.go accordingly.
func flatten(events []game.DomainEvent) any {
	for _, e := range events {
		switch v := e.(type) {
		case trivia.AnswerAccepted:
			return struct{ Delta, Total int }{Delta: v.Delta, Total: v.Total}
		case trivia.HintRevealed:
			return struct{ Hint string }{Hint: v.Hint}
		case puzzle.PieceMoved:
			return struct{ FromX, FromY, ToX, ToY int }{v.FromX, v.FromY, v.ToX, v.ToY}
		case puzzle.MoveUndone:
			return struct{}{}
		}
	}
	return nil
}
