package queries

//import (
//	"context"
//	"errors"
//
//	"pasteltime/internal/application/ports/outbound"
//)
//
//// GetUserSessionStatusQuery asks for the status of a session.
//type GetUserSessionStatusQuery struct {
//	SessionID string
//}
//
//// GetUserSessionStatusHandler returns session status (e.g., isExpired).
//type GetUserSessionStatusHandler struct {
//	sessionRepo outbound.SessionRepository
//}
//
//func NewGetUserSessionStatusHandler(repo outbound.SessionRepository) *GetUserSessionStatusHandler {
//	return &GetUserSessionStatusHandler{sessionRepo: repo}
//}
//
//func (h *GetUserSessionStatusHandler) HandleQuery(ctx context.Context, query interface{}) (interface{}, error) {
//	q, ok := query.(GetUserSessionStatusQuery)
//	if !ok {
//		return nil, errors.New("invalid query type, expected GetUserSessionStatusQuery")
//	}
//
//	session, err := h.sessionRepo.FindSessionByID(ctx, q.SessionID)
//	if err != nil {
//		return nil, err
//	}
//	if session == nil {
//		return map[string]bool{"exists": false}, nil
//	}
//	return map[string]interface{}{
//		"exists":    true,
//		"isExpired": session.IsExpired(),
//	}, nil
//}
//
