package game

// IDs and enums (trim to what you use)
type GameSessionID string
type PlayerID string

type GameType int

const (
	GameTypeUnspecified GameType = iota
	GameTypeTrivia
	GameTypePuzzle
)

type SessionStatus int

const (
	SessionStatusUnspecified SessionStatus = iota
	SessionStatusCreated
	SessionStatusActive
	SessionStatusEnded
	SessionStatusCancelled
)

type RulesConfig interface {
	Type() GameType
	Validate() error
}
