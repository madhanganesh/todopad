package repository

import (
	"database/sql"
	"math/rand"
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/config"
	"github.com/madhanganesh/todopad/api/model"
)

func setupdb(t *testing.T) *sql.DB {
	db, err := config.InitializeDB(":memory:", "../_scripts/migrations")
	if err != nil {
		t.Fatal(err)
	}

	_, err = db.Exec(`delete from todos`)
	if err != nil {
		t.Fatal(err)
	}

	_, err = db.Exec(`delete from users`)
	if err != nil {
		t.Fatal(err)
	}

	rand.Seed(time.Now().UTC().UnixNano())
	return db
}

func adjustTodoDue(todo model.Todo) model.Todo {
	todo.Due = time.Date(todo.Due.Year(), todo.Due.Month(), todo.Due.Day(), todo.Due.Hour(), todo.Due.Minute(), todo.Due.Second(), todo.Due.Nanosecond(), time.Now().UTC().Location())
	return todo
}
