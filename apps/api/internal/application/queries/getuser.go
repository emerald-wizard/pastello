package queries

//
//import (
//	"context"
//	"errors"
//
//	"pasteltime/internal/adapters/persistence"
//)
//
//// GetUserQuery requests user info by user ID.
//type GetUserQuery struct {
//	UserID string
//}
//
//// GetUserHandler fetches a user from the repository.
//type GetUserHandler struct {
//	userRepo persistence.UserRepositoryPostgres // or outbound.UserRepository
//}
//
//func NewGetUserHandler(repo persistence.UserRepositoryPostgres) *GetUserHandler {
//	return &GetUserHandler{userRepo: repo}
//}
//
//func (h *GetUserHandler) HandleQuery(ctx context.Context, query interface{}) (interface{}, error) {
//	q, ok := query.(GetUserQuery)
//	if !ok {
//		return nil, errors.New("invalid query type, expected GetUserQuery")
//	}
//
//	user, err := h.userRepo.FindUserByID(ctx, q.UserID)
//	if err != nil {
//		return nil, err
//	}
//	if user == nil {
//		return nil, nil // user not found
//	}
//	return user, nil
//}
//
