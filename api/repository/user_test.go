package repository

import (
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"

	"github.com/madhanganesh/todopad/api/model"
)

func TestCreateUser(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)
	assert.NotEqual(t, int64(0), user.ID, "UserID after creation should not be 0")
}

func TestAddingExistingEmail(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	user, _ = userRepo.Create(user)

	userB := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	_, err := userRepo.Create(userB)
	assert.Equal(t, ErrEmailExists, err)
}

func TestGetUser(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	user, _ = userRepo.Create(user)

	userRet, err := userRepo.Get("test@test.com")
	assert.NoError(t, err)
	assert.Equal(t, user, userRet, "Want: %v, Got: %v", user, userRet)
}

func TestGetUserForNotExists(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	_, err := userRepo.Get("test@test.com")
	assert.Equal(t, ErrNoUserExists, err)
}
