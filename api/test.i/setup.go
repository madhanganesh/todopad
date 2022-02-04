package testi

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/config"
	todohttp "github.com/madhanganesh/todopad/api/http"
	"github.com/madhanganesh/todopad/api/model"
)

var appConfig config.App
var db *sql.DB

var NoTags = []string{}

func init() {
	var err error
	port := "3000"
	secretKey := "testsecretkey"
	databaseURL := ":memory:"
	migrationsDir := "../_scripts/migrations"

	appConfig = config.NewConfig(port, secretKey, databaseURL, migrationsDir)
	db, err = config.InitializeDB(appConfig.DatabaseURL, appConfig.MigrationsDir)
	if err != nil {
		panic(err)
	}

	httpServer := todohttp.NewServer(appConfig, db)
	go httpServer.ListenAndServe()
}

func setupDB(t *testing.T) {
	t.Helper()

	_, err := db.Exec(`delete from todos`)
	if err != nil {
		t.Fatal(err)
	}
	_, err = db.Exec(`delete from users`)
	if err != nil {
		t.Fatal(err)
	}
}

func setupUser(t *testing.T, name string) model.LoginResponse {
	t.Helper()

	reqStr := fmt.Sprintf(`{"email":"%s@test.com", "name":"%s", "password": "password"}`, name, name)
	res, err := http.Post(getURL("signup"), "application/json", strings.NewReader(reqStr))
	if err != nil {
		t.Fatal(err)
	}

	var loginResponse model.LoginResponse
	err = json.NewDecoder(res.Body).Decode(&loginResponse)
	if err != nil {
		t.Fatal(err)
	}
	defer res.Body.Close()

	return loginResponse
}

func getURL(resource string) string {
	return "http://localhost:" + appConfig.Port + "/" + resource
}

func getTestTask(t *testing.T, userid int64, title string, done bool, due time.Time, tags []string) io.Reader {
	todo := model.Todo{
		UserID: userid,
		Title:  title,
		Due:    due,
		Effort: 1.0,
		Tags:   tags,
		Done:   false,
	}
	var temp bytes.Buffer
	err := json.NewEncoder(&temp).Encode(todo)
	if err != nil {
		t.Fatal(err)
	}

	return bytes.NewReader(temp.Bytes())
}
