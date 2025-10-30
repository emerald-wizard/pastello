package ports

import "time"

// Clock is a small seam for time to make logic testable/deterministic.
type Clock interface {
	Now() time.Time
}
