package clockfixed

import (
	"time"

	"github.com/runecraft-studios/pastello/internal/ports"
)

type Fixed struct{ T time.Time }

var _ ports.Clock = Fixed{}

func (f Fixed) Now() time.Time { return f.T }
