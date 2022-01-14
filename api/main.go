package main

import (
	"log"
	"os"

	"github.com/madhanganesh/todopad/api/config"
	"github.com/madhanganesh/todopad/api/http"
)

func main() {
	appConfig, err := config.NewConfigFromEnvs()
	if err != nil {
		log.Printf("%v", err)
		os.Exit(1)
	}

	db, err := config.InitializeDB(appConfig.DatabaseURL, appConfig.MigrationsDir)
	if err != nil {
		log.Printf("Error in initializing DB: %v", err)
		os.Exit(1)
	}

	httpServer := http.NewServer(appConfig, db)

	log.Fatal(httpServer.ListenAndServe())
}
