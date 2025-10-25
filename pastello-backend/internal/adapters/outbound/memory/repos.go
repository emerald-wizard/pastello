package memory

import (
	"context"
	"sync"

	"github.com/runecraft-studios/pastello/internal/domain/game"
)

// SessionRepo is a threadsafe in-memory repo for demo/dev use.
// Not persistent; suitable for local runs and tests.
type SessionRepo struct {
	mu sync.RWMutex
	m  map[game.GameSessionID]struct {
		S    *game.Session
		Snap any // optional engine snapshot or projection
	}
}

func NewSessionRepo() *SessionRepo {
	return &SessionRepo{
		m: make(map[game.GameSessionID]struct {
			S    *game.Session
			Snap any
		}),
	}
}

func (r *SessionRepo) Save(ctx context.Context, s *game.Session, snapshot any) error {
	_ = ctx
	r.mu.Lock()
	r.m[s.ID] = struct {
		S    *game.Session
		Snap any
	}{S: s, Snap: snapshot}
	r.mu.Unlock()
	return nil
}

func (r *SessionRepo) Load(ctx context.Context, id game.GameSessionID) (*game.Session, any, error) {
	_ = ctx
	r.mu.RLock()
	v, ok := r.m[id]
	r.mu.RUnlock()
	if !ok {
		return nil, nil, nil // not found: return (nil,nil,nil) so callers can branch
	}
	return v.S, v.Snap, nil
}

//var _ out.GameSessionRepo = (*SessionRepo)(nil)
