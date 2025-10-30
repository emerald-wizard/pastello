package ws

import (
	"context"
	"errors"

	"github.com/runecraft-studios/pastello/apps/api/internal/application/services"
	"github.com/runecraft-studios/pastello/apps/api/internal/domain/game"

	web "github.com/runecraft-studios/pastello/packages/contracts/gen/go/runecraftstudios/pastello/web/game/v1"
)

type GameCommandHandler struct {
	Registry services.CommandRegistry
}

// Handle routes trivia/puzzle commands carried inside the Envelope oneof.
// It mirrors the old behavior, but is transport-agnostic (no websocket.Conn,
// no proto marshal/write here).
func (h GameCommandHandler) Handle(ctx context.Context, env *web.Envelope) (*web.Envelope, error) {
	// 1) Ensure we have a supported command and a session id
	if env == nil || env.Body == nil {
		return nil, nil
	}
	sessID := extractSessionID(env)
	if sessID == "" {
		return nil, errors.New("missing session_id")
	}

	// 2) Build a lightweight session context for the registry
	sess := &game.Session{
		ID: game.GameSessionID(sessID),
	}
	// If you keep per-game typing in domain:
	// sess.Type = pickTypeFromCmd(env)

	// 3) Convert the web command into a registry command
	cmd := toAppCmd(env)
	if cmd == nil {
		return nil, nil // not a command we handle here
	}

	// 4) Dispatch to registry; get domain/app payload back
	payload, err := h.Registry.Handle(ctx, sess, cmd)
	if err != nil {
		return nil, err
	}

	// 5) Translate payload → edge/web event Envelope, echo correlationId
	out := toEdgeEvent(env.CorrelationId, payload, sessID)
	return out, nil
}
