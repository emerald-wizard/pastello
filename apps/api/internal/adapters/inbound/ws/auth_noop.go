package ws

import (
	"context"
	"net/http"

	web "github.com/runecraft-studios/pastello/packages/contracts/gen/go/runecraftstudios/pastello/web/game/v1"
)

type NoopAuth struct{}

func (NoopAuth) OnUpgrade(r *http.Request) (context.Context, error) { return r.Context(), nil }
func (NoopAuth) OnEnvelope(ctx context.Context, _ *web.Envelope) (context.Context, error) {
	return ctx, nil
}
