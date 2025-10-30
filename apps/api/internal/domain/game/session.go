package game

import "time"

type Session struct {
	ID        GameSessionID
	Type      GameType
	PlayerIDs []PlayerID
	Status    SessionStatus
	CreatedAt time.Time
	RulesetID string
	// Optional: embed per-game substates, if you keep them in Session.
	// Puzzle *puzzle.State // <- requires importing puzzle; keep decoupled for now.
}

func NewSession(id string, t GameType, players []PlayerID, at time.Time) *Session {
	return &Session{ID: GameSessionID(id), Type: t, PlayerIDs: players, Status: 0, CreatedAt: at}
}
