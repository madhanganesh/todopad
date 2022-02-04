package testi

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"fmt"
	"net/http"
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/madhanganesh/todopad/api/repository"
	"github.com/stretchr/testify/assert"
)

func TestReportAdhocSimple(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")

	todos := createTodos(t, db, loginResponse.UserID, 0)
	reportRequest := createReportRequest(t, -1, 1, "day")
	expected := getExpectedResponse(todos, reportRequest)

	var temp bytes.Buffer
	err := json.NewEncoder(&temp).Encode(reportRequest)
	if err != nil {
		t.Fatal(err)
	}
	requestDatat := bytes.NewReader(temp.Bytes())

	req, err := http.NewRequest("POST", getURL("report"), requestDatat)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, res.StatusCode)

	var actual model.ReportResponse
	err = json.NewDecoder(res.Body).Decode(&actual)
	assert.NoError(t, err)
	defer res.Body.Close()
	assert.Equal(t, expected["day"], actual.EffortsByGroup)
}

func day(days int) time.Time {
	return time.Now().Add(time.Duration(days) * time.Hour * 24)
}

func createTodos(t *testing.T, db *sql.DB, userID int64, ds ...int) []model.Todo {
	t.Helper()

	todos := []model.Todo{}
	todoRepo := repository.NewTodoRepository(db)
	for i, d := range ds {
		title := fmt.Sprintf("test todo - %d", i)

		due := day(d)
		effort := float32(1.5)
		todo := model.Todo{UserID: userID, Title: title, Due: due.UTC(), Effort: effort}
		todo, err := todoRepo.Create(todo)
		assert.NoError(t, err)
		todos = append(todos, todo)
	}
	return todos
}

func createReportRequest(t *testing.T, start int, end int, groupby string) model.ReportRequest {
	now := time.Now()
	startDate := now.Add(time.Duration(start) * time.Hour * 24)
	endDate := now.Add(time.Duration(end) * time.Hour * 24)
	startDate = time.Date(startDate.Year(), startDate.Month(), startDate.Day(), 0, 0, 0, 0, startDate.Location())
	endDate = time.Date(endDate.Year(), endDate.Month(), endDate.Day(), 23, 59, 59, 999, endDate.Location())
	_, offset := now.Zone()
	reportRequest := model.ReportRequest{From: startDate.UTC(), To: endDate.UTC(), GroupBy: groupby, TimeZoneOffsetInSecs: offset}
	return reportRequest
}

func getExpectedResponse(todos []model.Todo, reportRequest model.ReportRequest) map[string]map[string]float32 {
	groups := map[string]map[string]float32{}
	groups["day"] = map[string]float32{}
	groups["week"] = map[string]float32{}
	groups["month"] = map[string]float32{}

	for _, todo := range todos {
		if todo.Due.After(reportRequest.From) && todo.Due.Before(reportRequest.To) {
			due := todo.Due.In(time.Now().Location())
			groups["day"][due.Format("2006-01-02")] += todo.Effort
			_, week := due.ISOWeek()
			groups["week"][fmt.Sprintf("%02d", week)] += todo.Effort
			groups["month"][fmt.Sprintf("%02d", int(due.Month()))] += todo.Effort
		}
	}

	return groups
}
