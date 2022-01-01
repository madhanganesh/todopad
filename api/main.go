package main

import (
	"log"

	"github.com/madhanganesh/todopad/api/config"
	"github.com/madhanganesh/todopad/api/http"
	_ "github.com/mattn/go-sqlite3"
)

func main() {
	appConfig, cleanup := config.NewAppConfigFromEnv()
	defer cleanup()

	httpServer := http.NewServer(appConfig)
	log.Fatal(httpServer.ListenAndServe())
}
