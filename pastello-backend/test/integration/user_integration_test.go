package integration

//
//import (
//	"context"
//	"testing"
//
//	"github.com/yourname/goproject_with_files/internal/adapters/persistence"
//	"github.com/yourname/goproject_with_files/internal/domain"
//)
//
//// Example integration test
//func TestUserIntegration(t *testing.T) {
//	userRepo := persistence.NewUserRepositoryPostgres(nil) // typically pass a test DB
//	ctx := context.Background()
//
//	user := domain.User{ID: "u999", Username: "Tester"}
//	if err := userRepo.SaveUser(ctx, user); err != nil {
//		t.Fatalf("failed to save user: %v", err)
//	}
//	found, err := userRepo.FindUserByID(ctx, "u999")
//	if err != nil {
//		t.Fatalf("failed to find user: %v", err)
//	}
//	if found == nil || found.Username != "Tester" {
//		t.Errorf("expected user 'Tester' but got %#v", found)
//	}
//}
//
