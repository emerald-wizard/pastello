package services

import (
	"context"

	"github.com/runecraft-studios/pastello/apps/api/internal/domain/game"
)

type CommandRegistry struct{ Svc GameService }

func (r CommandRegistry) Handle(ctx context.Context, sess *game.Session, cmd any) (any, error) {
	return r.Svc.HandleGameCommand(ctx, sess.ID, cmd)
}
