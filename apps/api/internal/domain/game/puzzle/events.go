package puzzle

import "github.com/runecraft-studios/pastello/apps/api/internal/domain/game"

type PieceMoved struct {
	game.EventMeta
	SessionID    game.GameSessionID
	FromX, FromY int
	ToX, ToY     int
}

func (PieceMoved) Name() string { return "puzzle.piece_moved" }

type MoveUndone struct {
	game.EventMeta
	SessionID game.GameSessionID
}

func (MoveUndone) Name() string { return "puzzle.move_undone" }
