package repository

import (
	"database/sql"
	"testing"
	"time"

	_ "github.com/lib/pq"
	"github.com/madhanganesh/todopad/api/model"
	"github.com/pressly/goose"
)

var db *sql.DB

func init() {
	var err error
	db, err = sql.Open("postgres", "user=postgres password=zenith sslmode=disable")
	if err != nil {
		panic(err)
	}

	err = goose.SetDialect("postgres")
	if err != nil {
		panic(err)
	}
}

func setupdb(t *testing.T) *sql.DB {
	_, err := db.Exec(`delete from todos`)
	if err != nil {
		t.Fatal(err)
	}

	_, err = db.Exec(`delete from users`)
	if err != nil {
		t.Fatal(err)
	}

	return db
}

func adjustTodoDue(todo model.Todo) model.Todo {
	todo.Due = time.Date(todo.Due.Year(), todo.Due.Month(), todo.Due.Day(), todo.Due.Hour(), todo.Due.Minute(), todo.Due.Second(), todo.Due.Nanosecond(), time.Now().UTC().Location())
	return todo
}
