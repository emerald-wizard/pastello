package queries

//import (
//	"context"
//
//	"pasteltime/internal/application/ports/outbound"
//)
//
//// ListActiveSessionsQuery requests a user's active sessions.
//type ListActiveSessionsQuery struct {
//	UserID string
//}
//
//// ListActiveSessionsHandler finds all active sessions for a user.
//type ListActiveSessionsHandler struct {
//	sessionRepo outbound.SessionRepository
//}
//
//func NewListActiveSessionsHandler(repo outbound.SessionRepository) *ListActiveSessionsHandler {
//	return &ListActiveSessionsHandler{sessionRepo: repo}
//}
//
//func (l *ListActiveSessionsHandler) HandleQuery(ctx context.Context, query interface{}) (interface{}, error) {
//	_, ok := query.(ListActiveSessionsQuery)
//	if !ok {
//		return nil, nil
//	}
//
//	return true, nil
//}
//
