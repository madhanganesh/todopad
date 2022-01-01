package testi

import (
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/pressly/goose"

	"github.com/madhanganesh/todopad/api/config"
	"github.com/madhanganesh/todopad/api/http"
)

var appConfig *config.App

func init() {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		panic(err)
	}

	goose.SetDialect("sqlite3")
	if err := goose.Up(db, "./../_db/migrations"); err != nil {
		panic(err)
	}

	appConfig = config.NewAppConfigFromParams("3000", "testsecretkey", db)
	httpServer := http.NewServer(appConfig)

	go httpServer.ListenAndServe()
}

func setupDB(t *testing.T) {
	t.Helper()

	_, err := appConfig.Db.Exec(`delete from users`)
	if err != nil {
		t.Fatal(err)
	}
}
