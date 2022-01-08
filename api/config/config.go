package config

import (
	"database/sql"
	"fmt"
	"log"
	"os"
)

type App struct {
	Port      string
	Db        *sql.DB
	SecretKey []byte
	SQLiteDir string
}

func NewAppConfigFromEnv() (*App, func()) {
	sqliteDir := os.Getenv("SQLITE_DIR")
	if sqliteDir == "" {
		fmt.Println("Env SQLITE_DIR is not set. Check README.md for more details")
		os.Exit(1)
	}

	secretKey := os.Getenv("SECRET_KEY")
	if secretKey == "" {
		fmt.Println("Env SECRET_KEY is not set. Check README.md for more details")
		os.Exit(1)
	}

	port := os.Getenv("PORT")
	if port == "" {
		fmt.Println("Env PORT is not set. Check README.md for more details")
		os.Exit(1)
	}

	db, err := GetSqliteDB(sqliteDir + "/todopad.sqlite")
	if err != nil {
		log.Fatal(err)
	}

	return &App{
			Port:      port,
			SecretKey: []byte(secretKey),
			Db:        db,
			SQLiteDir: sqliteDir,
		}, func() {
			db.Close()
		}
}

func NewAppConfigFromParams(port string, secretKey string, db *sql.DB) *App {
	return &App{
		Port:      port,
		SecretKey: []byte(secretKey),
		Db:        db,
	}
}
