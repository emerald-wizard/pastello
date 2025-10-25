package usecase

import (
	"context"

	"github.com/runecraft-studios/pastello/internal/domain/game"
	"github.com/runecraft-studios/pastello/internal/ports"
)

type StartDeps struct {
	Repo  ports.GameSessionRepo
	Clock ports.Clock
	IDGen ports.IDGen
	Bus   ports.EventBus
}

type StartGameSessionIn struct {
	HostID   game.PlayerID
	GameType game.GameType // NEW: set the engine type at creation
	// If you snapshot rules on the session, include them here:
	// TriviaRules *trivia.TriviaRules
	// PuzzleRules *puzzle.PuzzleRules
	// Or just a RulesetID string if you reference pre-defined rules
	RulesetID string
}

type StartGameSessionOut struct {
	SessionID game.GameSessionID
}

type StartGameSession struct{ d StartDeps }

func NewStartGameSession(d StartDeps) StartGameSession { return StartGameSession{d: d} }

func (uc StartGameSession) Execute(ctx context.Context, in StartGameSessionIn) (StartGameSessionOut, error) {
	id := game.GameSessionID(uc.d.IDGen.New())
	now := uc.d.Clock.Now()

	// Ensure we set the session type so HandleGameCommand can find an engine
	gt := in.GameType
	if gt == game.GameTypeUnspecified {
		gt = game.GameTypeUnspecified // or default if you want
	}

	sess := &game.Session{
		ID:        id,
		Type:      gt,
		PlayerIDs: []game.PlayerID{in.HostID},
		Status:    game.SessionStatusCreated,
		CreatedAt: now,
		RulesetID: in.RulesetID,
		// If you snapshot rules directly in Session, set them here.
	}

	var snapshot any
	if err := uc.d.Repo.Save(ctx, sess, snapshot); err != nil {
		return StartGameSessionOut{}, err
	}

	if uc.d.Bus != nil {
		_ = uc.d.Bus.Publish(ctx, "game.session.started", map[string]any{
			"session_id":  string(id),
			"host_id":     string(in.HostID),
			"game_type":   int(gt),
			"ruleset_id":  in.RulesetID,
			"occurred_at": now,
		})
	}
	return StartGameSessionOut{SessionID: id}, nil
}
