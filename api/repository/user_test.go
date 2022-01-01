package repository

import (
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/pressly/goose"
	"github.com/stretchr/testify/assert"

	"github.com/madhanganesh/todopad/api/model"
)

var db *sql.DB

func init() {
	var err error
	db, err = sql.Open("sqlite3", ":memory:")
	if err != nil {
		panic(err)
	}

	goose.SetDialect("sqlite3")
	if err := goose.Up(db, "./../_db/migrations"); err != nil {
		panic(err)
	}
}

func setupDB(t *testing.T) {
	t.Helper()

	_, err := db.Exec(`delete from users`)
	if err != nil {
		t.Fatal(err)
	}
}

func TestCreateUser(t *testing.T) {
	setupDB(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)
	assert.Equal(t, int64(1), user.ID, "UserID after creation should be 1")
}

func TestAddingExistingEmail(t *testing.T) {
	setupDB(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	user, _ = userRepo.Create(user)

	userB := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	_, err := userRepo.Create(userB)
	assert.Equal(t, ErrEmailExists, err)
}

func TestGetUser(t *testing.T) {
	setupDB(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Madhan Ganesh", Email: "test@test.com", Password: "password"}
	user, _ = userRepo.Create(user)

	userRet, err := userRepo.Get("test@test.com")
	assert.NoError(t, err)
	assert.Equal(t, user, userRet, "Want: %v, Got: %v", user, userRet)
}

func TestGetUserForNotExists(t *testing.T) {
	setupDB(t)

	userRepo := NewUserRepository(db)
	_, err := userRepo.Get("test@test.com")
	assert.Equal(t, ErrNoUserExists, err)
}
