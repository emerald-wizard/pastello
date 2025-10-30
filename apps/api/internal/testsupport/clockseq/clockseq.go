package clockseq

import (
	"sync"
	"time"

	"github.com/runecraft-studios/pastello/apps/api/internal/ports"
)

type Seq struct {
	mu sync.Mutex
	V  []time.Time
	i  int
}

var _ ports.Clock = (*Seq)(nil)

func (c *Seq) Now() time.Time {
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.i >= len(c.V) {
		return time.Unix(0, 0)
	}
	t := c.V[c.i]
	c.i++
	return t
}
