package game

import "context"

// Common shape for all engines.
type Engine interface {
	Type() GameType
	// Pure(ish): given session + command, decide next state & events.
	Apply(ctx context.Context, sess *Session, cmd any) (next *Session, events []DomainEvent, err error)
}
