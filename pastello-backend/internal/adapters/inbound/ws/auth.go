package ws

import (
	"context"
	"net/http"

	web "github.com/runecraft-studios/pastello/internal/gen/runecraftstudios/pastello/web/game/v1"
)

// Auth is a small middleware-style contract:
//   - OnUpgrade: runs BEFORE WS upgrade; can inspect headers/cookies and
//     return a context carrying auth claims (or an error to reject).
//   - OnEnvelope: runs PER MESSAGE; can re-check or enrich context per frame.
type Auth interface {
	OnUpgrade(r *http.Request) (context.Context, error)
	OnEnvelope(ctx context.Context, env *web.Envelope) (context.Context, error)
}
