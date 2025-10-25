package game

import "time"

// DomainEvent is a pure domain event (no proto, no transport types).
type DomainEvent interface {
	Name() string
	OccurredAt() time.Time
}

// Common metadata you can embed in concrete events.
type EventMeta struct {
	at time.Time
}

func (m EventMeta) OccurredAt() time.Time { return m.at }

// Helpers to create metadata (use the Clock port in UCs/engines)
func NewMeta(at time.Time) EventMeta { return EventMeta{at: at} }
