package ports

import (
	"context"

	"github.com/runecraft-studios/pastello/internal/domain/game"
)

// Repositories (write side)
type GameSessionRepo interface {
	Save(ctx context.Context, s *game.Session, snapshot any) error
	Load(ctx context.Context, id game.GameSessionID) (*game.Session, any, error)
}

// Tip: if you ever split read/write sides (CQRS), you can define:
// type GameSessionReader interface { ByID(ctx context.Context, id game.GameSessionID) (*game.Session, any, error) }
// type GameSessionWriter interface { Save(ctx context.Context, s *game.Session, snapshot any) error }
