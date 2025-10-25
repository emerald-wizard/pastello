package puzzle

import (
	"context"
	"errors"
	"fmt"

	"github.com/runecraft-studios/pastello/internal/domain/game"
	"github.com/runecraft-studios/pastello/internal/ports"
)

// Deps are injected ports to keep the engine pure enough.
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
		state: NewState(4, 4), // example default; adjust as needed
	}
}

func (e *Engine) Type() game.GameType { return game.GameTypePuzzle }

// Optional: expose a snapshot if you store puzzle substate here.
func (e *Engine) Snapshot() State { return e.state }

func (e *Engine) Restore(snap any) error {
	s, ok := snap.(State)
	if !ok {
		return fmt.Errorf("bad snapshot type")
	}
	e.state = s
	return nil
}

// Apply is the command dispatcher. It does not mutate the input session;
// it returns a copy with any changes (if you project puzzle state into Session).
func (e *Engine) Apply(ctx context.Context, sess *game.Session, cmd any) (*game.Session, []game.DomainEvent, error) {
	_ = ctx

	in, ok := cmd.(Cmd)
	if !ok {
		return nil, nil, game.ErrWrongEngine
	}

	switch {
	case in.MovePiece != nil:
		evts, err := e.movePiece(sess.ID, in.MovePiece.FromX, in.MovePiece.FromY, in.MovePiece.ToX, in.MovePiece.ToY)
		if err != nil {
			return nil, nil, err
		}
		next := *sess // copy if you project state; else keep as-is
		return &next, evts, nil

	case in.UndoMove != nil:
		evts, err := e.undoMove(sess.ID)
		if err != nil {
			return nil, nil, err
		}
		next := *sess
		return &next, evts, nil
	}

	return nil, nil, game.ErrUnsupportedCommand
}

// ---- internal helpers called by Apply ----

func (e *Engine) movePiece(sessionID game.GameSessionID, fromX, fromY, toX, toY int) ([]game.DomainEvent, error) {
	// Minimal legality checks; replace with your real rules.
	if !e.inBounds(fromX, fromY) || !e.inBounds(toX, toY) {
		return nil, errors.New("move out of bounds")
	}
	// Record move (replace with real tile/board updates).
	e.state.History = append(e.state.History, Move{From: Pos{fromX, fromY}, To: Pos{toX, toY}})

	evt := PieceMoved{
		EventMeta: game.NewMeta(e.deps.Clock.Now()),
		SessionID: sessionID,
		FromX:     fromX, FromY: fromY, ToX: toX, ToY: toY,
	}
	return []game.DomainEvent{evt}, nil
}

func (e *Engine) undoMove(sessionID game.GameSessionID) ([]game.DomainEvent, error) {
	if len(e.state.History) == 0 {
		return nil, errors.New("nothing to undo")
	}
	// Pop last move; update state accordingly (replace with real logic).
	e.state.History = e.state.History[:len(e.state.History)-1]

	evt := MoveUndone{
		EventMeta: game.NewMeta(e.deps.Clock.Now()),
		SessionID: sessionID,
	}
	return []game.DomainEvent{evt}, nil
}

func (e *Engine) inBounds(x, y int) bool {
	return x >= 0 && x < e.state.Width && y >= 0 && y < e.state.Height
}
