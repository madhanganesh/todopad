package config

import (
	"fmt"
	"os"
)

type App struct {
	Port          string
	SecretKey     []byte
	DatabaseURL   string
	MigrationsDir string
}

func NewConfigFromEnvs() (App, error) {
	environmentVariables := map[string]string{"PORT": "", "SECRET_KEY": "", "DATABASE_URL": "", "MIGRATIONS_DIR": ""}
	for key := range environmentVariables {
		value := os.Getenv(key)
		if value == "" {
			return App{}, fmt.Errorf("environment variable '%s' is not present", key)
		}

		environmentVariables[key] = value
	}

	appConfig := App{
		Port:          environmentVariables["PORT"],
		SecretKey:     []byte(environmentVariables["SECRET_KEY"]),
		DatabaseURL:   environmentVariables["DATABASE_URL"],
		MigrationsDir: environmentVariables["MIGRATIONS_DIR"],
	}

	return appConfig, nil
}

func NewConfig(port, secretKey, databaseURL, migrationsDir string) App {
	return App{Port: port, SecretKey: []byte(secretKey), DatabaseURL: databaseURL, MigrationsDir: migrationsDir}
}
