package services

import (
	"context"

	"github.com/runecraft-studios/pastello/internal/application/usecase"
	"github.com/runecraft-studios/pastello/internal/domain/game"
	"github.com/runecraft-studios/pastello/internal/ports"
)

// Deps are the application-level ports/services required to build the use cases.
type Deps struct {
	Repo  ports.GameSessionRepo // load/save sessions
	Bus   ports.EventBus        // publish domain events
	Clock ports.Clock           // deterministic time
	IDGen ports.IDGen           // deterministic IDs
	Engs  EngineFactory         // selects the concrete engine by GameType
}

// GameService is a thin façade that exposes app operations to adapters.
// It stitches together the individual use cases; it does not contain rules.
type GameService struct {
	startUC  usecase.StartGameSession
	handleUC usecase.HandleGameCommand
}

// NewGameService composes the UCs from shared dependencies.
func NewGameService(d Deps) GameService {
	return GameService{
		startUC: usecase.NewStartGameSession(usecase.StartDeps{
			Repo:  d.Repo,
			Clock: d.Clock,
			IDGen: d.IDGen,
			Bus:   d.Bus,
		}),
		handleUC: usecase.NewHandleGameCommand(usecase.HandleDeps{
			Repo:    d.Repo,
			Bus:     d.Bus,
			Engines: d.Engs, // services.EngineFactory
			//Clock:   d.Clock,
		}),
	}
}

// StartGameSession creates a new session and emits a "started" event.
// Adapters map their transport payloads (protobuf) into these simple args.
func (s GameService) StartGameSession(ctx context.Context, hostID string) (game.GameSessionID, error) {
	out, err := s.startUC.Execute(ctx, usecase.StartGameSessionIn{
		HostID: game.PlayerID(hostID),
	})
	if err != nil {
		return "", err
	}
	return out.SessionID, nil
}

// HandleGameCommand runs an app command against a session via the proper engine.
// `cmd` is an application command DTO (from internal/application/commands), not protobuf.
func (s GameService) HandleGameCommand(ctx context.Context, sessionID game.GameSessionID, cmd any) (any, error) {
	out, err := s.handleUC.Execute(ctx, usecase.HandleGameCommandIn{
		SessionID: sessionID,
		Command:   cmd,
	})
	if err != nil {
		return nil, err
	}
	return out.Payload, nil
}
