package trivia

import (
	"context"
	"fmt"

	"github.com/runecraft-studios/pastello/internal/domain/game"
	"github.com/runecraft-studios/pastello/internal/ports"
)

// Deps keep the engine pure-enough (no globals).
type Deps struct {
	Clock ports.Clock
	RNG   ports.RNG
	IDGen ports.IDGen
}

type Engine struct {
	deps  Deps
	state State
}

func NewEngine(d Deps) *Engine {
	return &Engine{
		deps:  d,
		state: NewState(),
	}
}

func (e *Engine) Type() game.GameType { return game.GameTypeTrivia }

// Optional: expose a snapshot for read-side projection/tests.
func (e *Engine) Snapshot() State { return e.state }

func (e *Engine) Restore(snap any) error {
	s, ok := snap.(State)
	if !ok {
		return fmt.Errorf("bad snapshot type")
	}
	e.state = s
	return nil
}

// Apply: single entrypoint for all trivia commands.
// Does not mutate the input session; returns a (possibly) updated copy.
func (e *Engine) Apply(ctx context.Context, sess *game.Session, cmd any) (*game.Session, []game.DomainEvent, error) {
	_ = ctx

	in, ok := cmd.(Cmd)
	if !ok {
		return nil, nil, game.ErrWrongEngine
	}

	switch {
	case in.SubmitAnswer != nil:
		evts := e.submitAnswer(sess.ID, in.SubmitAnswer.PlayerID, in.SubmitAnswer.Answer)
		next := *sess
		return &next, evts, nil

	case in.RevealHint != nil:
		evts := e.revealHint(sess.ID)
		next := *sess
		return &next, evts, nil
	}

	return nil, nil, game.ErrUnsupportedCommand
}

// ---- internal helpers ----

// For now: always +10; replace with real correctness/scoring logic.
func (e *Engine) submitAnswer(sessionID game.GameSessionID, playerID, answer string) []game.DomainEvent {
	_ = answer // TODO: validate correctness against question in state/rules
	const delta = 10

	cur := e.state.Scores[playerID]
	total := cur + delta
	e.state.Scores[playerID] = total

	evt := AnswerAccepted{
		EventMeta: game.NewMeta(e.deps.Clock.Now()),
		SessionID: sessionID,
		PlayerID:  game.PlayerID(playerID),
		Delta:     delta,
		Total:     total,
	}
	return []game.DomainEvent{evt}
}

func (e *Engine) revealHint(sessionID game.GameSessionID) []game.DomainEvent {
	h := "This is a hint." // TODO: compute from state/rules/questions
	e.state.Hints = append(e.state.Hints, h)

	evt := HintRevealed{
		EventMeta: game.NewMeta(e.deps.Clock.Now()),
		SessionID: sessionID,
		Hint:      h,
	}
	return []game.DomainEvent{evt}
}
