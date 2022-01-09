package main

import (
	"log"

	"github.com/madhanganesh/todopad/api/config"
	"github.com/madhanganesh/todopad/api/http"
	_ "github.com/mattn/go-sqlite3"
	"github.com/pressly/goose"
)

func main() {
	appConfig, cleanup := config.NewAppConfigFromEnv()
	defer cleanup()

	err := runDbMigrations(appConfig)
	if err != nil {
		log.Fatal(err)
	}

	httpServer := http.NewServer(appConfig)
	log.Fatal(httpServer.ListenAndServe())
}

func runDbMigrations(appConfig *config.App) error {
	db := appConfig.Db
	migrationDir := appConfig.MigrationsDir

	if err := goose.SetDialect("postgres"); err != nil {
		return err
	}
	if err := goose.Up(db, migrationDir); err != nil {
		return err
	}

	return nil
}
