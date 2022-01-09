package config

import (
	"database/sql"
	"fmt"
	"os"
)

type App struct {
	Port          string
	Db            *sql.DB
	SecretKey     []byte
	MigrationsDir string
}

func NewAppConfigFromEnv() (*App, func()) {
	migrationsDir := os.Getenv("MIGRATIONS_DIR")
	if migrationsDir == "" {
		fmt.Println("Env MIGRATIONS_DIR is not set. Check README.md for more details")
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

	postgresDSN := os.Getenv("PG_CONNECTION_STRING")
	if postgresDSN == "" {
		fmt.Println("Env PG_CONNECTION_STRING is not set. Check README.md for more details")
		os.Exit(1)
	}

	db, err := sql.Open("postgres", postgresDSN)
	if err != nil {
		panic(fmt.Sprintf("DB: %v", err))
	}

	return &App{
			Port:          port,
			SecretKey:     []byte(secretKey),
			Db:            db,
			MigrationsDir: migrationsDir,
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
