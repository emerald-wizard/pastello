package ws

import (
	"context"

	"github.com/runecraft-studios/pastello/internal/application/services"
	sessionv1 "github.com/runecraft-studios/pastello/internal/gen/runecraftstudios/pastello/game/session/v1"
	typesv1 "github.com/runecraft-studios/pastello/internal/gen/runecraftstudios/pastello/game/types/v1"
	web "github.com/runecraft-studios/pastello/internal/gen/runecraftstudios/pastello/web/game/v1"
)

type GameStartHandler struct {
	Svc services.GameService
}

func (h GameStartHandler) Handle(ctx context.Context, env *web.Envelope) (*web.Envelope, error) {
	cmd := env.GetStartGameSession()
	if cmd == nil {
		return nil, nil
	}

	host := cmd.GetPlayerIds()
	var hostID string
	if len(host) > 0 {
		hostID = host[0].GetValue() // or pick explicitly from UI; adjust as you like
	}

	sessionID, err := h.Svc.StartGameSession(ctx, hostID)
	if err != nil {
		return nil, err
	}
	// Build the web event, embedding a *sessionv1.GameSession*
	return &web.Envelope{
		CorrelationId: env.CorrelationId,
		Body: &web.Envelope_GameSessionStarted{
			GameSessionStarted: &web.GameSessionStartedEvent{
				Session: &sessionv1.GameSession{
					Id: &typesv1.GameSessionId{Value: string(sessionID)},
					// Fill more fields if you have them at start time:
					// GameType:  cmd.GetGameType(),
					// PlayerIds: slice of string -> []string or []typesv1.PlayerId as your proto expects
					// Status:    "CREATED",
					// CreatedAt: time.Now().UTC().Format(time.RFC3339),
				},
			},
		},
	}, nil
}
