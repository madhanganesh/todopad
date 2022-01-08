package repository

import (
	"database/sql"
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/config"
	"github.com/madhanganesh/todopad/api/model"
	"github.com/pressly/goose"
)

type dummylogger struct {
}

func (d *dummylogger) Fatal(...interface{})                   {}
func (d *dummylogger) Fatalf(format string, v ...interface{}) {}
func (d *dummylogger) Print(v ...interface{})                 {}
func (d *dummylogger) Println(v ...interface{})               {}
func (d *dummylogger) Printf(format string, v ...interface{}) {}

func setupdb(t *testing.T) *sql.DB {
	db, err := config.GetSqliteDB(":memory:")
	//os.Remove("testdb.sqlite")
	//db, err := config.GetSqliteDB("file:testdb.sqlite")
	if err != nil {
		panic(err)
	}

	dlog := &dummylogger{}
	goose.SetLogger(dlog)
	goose.SetDialect("sqlite3")
	if err := goose.Up(db, "./../_db/migrations"); err != nil {
		panic(err)
	}

	return db
}

func adjustTodo(id int64, todo model.Todo) model.Todo {
	todo.ID = id
	todo.Due = time.Date(todo.Due.Year(), todo.Due.Month(), todo.Due.Day(), todo.Due.Hour(), todo.Due.Minute(), todo.Due.Second(), 0, todo.Due.Location())
	return todo
}
