package ws

import (
	"context"
	"errors"
	"net/http"
	"strings"

	web "github.com/runecraft-studios/pastello/packages/contracts/gen/go/runecraftstudios/pastello/web/game/v1"
)

type ctxKey string

const ctxKeySub ctxKey = "sub"

type BearerAuth struct {
	// For real use, inject a verifier/JWKs. Here we just compare a shared secret.
	AcceptToken string
	Subject     string // optional fixed subject to attach
}

// Authenticate implements Auth.Authenticate.
// It validates the Authorization header and returns a per-connection context.
func (a BearerAuth) OnUpgrade(r *http.Request) (context.Context, error) {
	h := r.Header.Get("Authorization")
	if !strings.HasPrefix(h, "Bearer ") {
		return nil, errors.New("missing bearer token")
	}
	token := strings.TrimPrefix(h, "Bearer ")
	if token != a.AcceptToken {
		return nil, errors.New("invalid token")
	}
	// Attach subject (or claims) to the connection context.
	ctx := context.WithValue(r.Context(), ctxKeySub, a.Subject)
	return ctx, nil
}

// OnEnvelope implements Auth.OnEnvelope.
// Optional per-message checks (e.g., check claims vs. envelope fields).
func (a BearerAuth) OnEnvelope(ctx context.Context, _ *web.Envelope) (context.Context, error) {
	return ctx, nil
}
