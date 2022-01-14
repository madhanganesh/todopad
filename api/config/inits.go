package config

import (
	"database/sql"

	_ "github.com/mattn/go-sqlite3"
	"github.com/pressly/goose"
)

func InitializeDB(databaseURL, migrationsDir string) (*sql.DB, error) {
	db, err := sql.Open("sqlite3", databaseURL+"?_foreign_keys=on")
	if err != nil {
		return nil, err
	}

	if err := goose.SetDialect("sqlite3"); err != nil {
		return nil, err
	}
	if err := goose.Up(db, migrationsDir); err != nil {
		return nil, err
	}

	return db, nil
}
