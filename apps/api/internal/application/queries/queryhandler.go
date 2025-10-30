package queries

import "context"

// QueryHandler is a generic interface for query handlers (read-side operations).
type QueryHandler interface {
	HandleQuery(ctx context.Context, query interface{}) (interface{}, error)
}
