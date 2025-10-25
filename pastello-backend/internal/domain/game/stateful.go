// internal/domain/game/stateful.go
package game

type StatefulEngine interface {
	Engine
	Snapshot() any
	Restore(snap any) error
}
