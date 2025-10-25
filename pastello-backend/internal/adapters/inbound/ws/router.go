package ws

import (
	"context"

	web "github.com/runecraft-studios/pastello/internal/gen/runecraftstudios/pastello/web/game/v1"
)

type Handler interface {
	Handle(ctx context.Context, env *web.Envelope) (*web.Envelope, error)
}

type Router struct {
	start Handler
	cmd   Handler
	auth  Auth
}

func NewRouter(start Handler, cmd Handler, auth Auth) *Router {
	return &Router{start: start, cmd: cmd, auth: auth}
}

func (r *Router) Auth() Auth { return r.auth } // used by Server

func (r *Router) Route(ctx context.Context, env *web.Envelope) (*web.Envelope, error) {
	switch env.GetBody().(type) {
	case *web.Envelope_StartGameSession:
		return r.start.Handle(ctx, env)
	default:
		if r.cmd != nil {
			return r.cmd.Handle(ctx, env)
		}
		return nil, nil
	}
}
