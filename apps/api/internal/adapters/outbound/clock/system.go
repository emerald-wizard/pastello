package clock

import (
	"time"

	"github.com/runecraft-studios/pastello/apps/api/internal/ports"
)

// System is the production clock (wrapping time.Now).
type System struct{}

var _ ports.Clock = System{}

func (System) Now() time.Time { return time.Now() }
