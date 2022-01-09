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

	_ "github.com/lib/pq"
	"github.com/pressly/goose"

	"github.com/madhanganesh/todopad/api/config"
	todohttp "github.com/madhanganesh/todopad/api/http"
	"github.com/madhanganesh/todopad/api/model"
)

var appConfig *config.App

func init() {
	db, err := sql.Open("postgres", "user=postgres password=zenith sslmode=disable")
	if err != nil {
		panic(err)
	}

	if err := goose.SetDialect("postgres"); err != nil {
		panic(err)
	}

	appConfig = config.NewAppConfigFromParams("3000", "testsecretkey", db)
	httpServer := todohttp.NewServer(appConfig)

	go httpServer.ListenAndServe()
}

func setupDB(t *testing.T) {
	t.Helper()

	_, err := appConfig.Db.Exec(`delete from todos`)
	if err != nil {
		t.Fatal(err)
	}
	_, err = appConfig.Db.Exec(`delete from users`)
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

func getTestTask(t *testing.T, userid int64, title string, done bool) io.Reader {
	var temp bytes.Buffer
	todo := model.Todo{
		UserID: userid,
		Title:  title,
		Due:    time.Now().UTC(),
		Effort: 1.0,
		Done:   false,
	}
	err := json.NewEncoder(&temp).Encode(todo)
	if err != nil {
		t.Fatal(err)
	}

	return bytes.NewReader(temp.Bytes())
}
