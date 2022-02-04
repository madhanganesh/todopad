package repository

import (
	"database/sql"
	"fmt"
	"math/rand"
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/stretchr/testify/assert"
)

func TestGroupByDaySimple(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, 0)
	reportRequest := createReportRequest(t, -1, 1, "day")
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["day"], reportResponse.EffortsByGroup)
}

func TestGroupByDay1xN(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, 0, 0, 0)
	reportRequest := createReportRequest(t, -1, 1, "day")
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["day"], reportResponse.EffortsByGroup)
}

func TestGroupByDayNx1(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, -1, 0, 1)
	reportRequest := createReportRequest(t, -1, 1, "day")
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["day"], reportResponse.EffortsByGroup)
}

func TestGroupByDayNxN(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, -1, -1, 0, 0, 1, 1)
	reportRequest := createReportRequest(t, -1, 1, "day")
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["day"], reportResponse.EffortsByGroup)
}

func TestGroupByDay(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7)
	reportRequest := createReportRequest(t, -1, 1, "day")
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["day"], reportResponse.EffortsByGroup)
}

func TestGroupByWeek(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7)
	reportRequest := createReportRequest(t, -7, 7, "week")
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["week"], reportResponse.EffortsByGroup)
}

func TestGroupByMonth(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7)
	reportRequest := createReportRequest(t, -20, 20, "month")
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["month"], reportResponse.EffortsByGroup)
}

func TestGroupByTagsSimple(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, 0)
	reportRequest := createReportRequest(t, -1, 1, "tags")
	reportRequest.Tags = getAllTags()
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["tags"], reportResponse.EffortsByGroup)
}

func TestGroupByTagsManyTodos(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7)
	reportRequest := createReportRequest(t, -1, 1, "tags")
	reportRequest.Tags = getAllTags()
	expected := getExpectedResponse(todos, reportRequest)

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["tags"], reportResponse.EffortsByGroup)
}

func TestGroupByTagsNonExistent(t *testing.T) {
	db := setupdb(t)
	todos := createTodos(t, db, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7)
	reportRequest := createReportRequest(t, -1, 1, "tags")
	reportRequest.Tags = []string{"tag1", "tag2"}
	expected := map[string]map[string]float32{
		"tags": {},
	}

	todoRepo := NewTodoRepository(db)
	reportResponse, err := todoRepo.GetReport(todos[0].UserID, reportRequest)
	assert.NoError(t, err)

	assert.Equal(t, expected["tags"], reportResponse.EffortsByGroup)
}

func day(days int) time.Time {
	return time.Now().Add(time.Duration(days) * time.Hour * 24)
}

func createTodos(t *testing.T, db *sql.DB, ds ...int) []model.Todo {
	t.Helper()

	var todos []model.Todo
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)

	todoRepo := NewTodoRepository(db)
	for i, d := range ds {
		title := fmt.Sprintf("test todo - %d", i)

		due := day(d)
		effort := float32(1.5)
		tags := getRandomTags()
		todo := model.Todo{UserID: user.ID, Title: title, Due: due.UTC(), Effort: effort, Tags: tags}
		todo, err = todoRepo.Create(todo)
		assert.NoError(t, err)
		todos = append(todos, todo)
	}
	return todos
}

func getRandomTags() []string {
	verbTags := []string{"architecture", "coordination", "coding", "meeting", "interview"}
	nounTags := []string{"module1", "proj2", "yodlee"}

	noun := nounTags[rand.Intn(len(nounTags))]
	verb := verbTags[rand.Intn(len(verbTags))]

	return []string{noun, verb}

}

func getAllTags() []string {
	verbTags := []string{"architecture", "coordination", "coding", "meeting", "interview"}
	nounTags := []string{"module1", "proj2", "yodlee"}

	return append(verbTags, nounTags...)
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
	groups["tags"] = map[string]float32{}

	for _, todo := range todos {
		if todo.Due.After(reportRequest.From) && todo.Due.Before(reportRequest.To) {
			due := todo.Due.In(time.Now().Location())
			groups["day"][due.Format("2006-01-02")] += todo.Effort
			_, week := due.ISOWeek()
			groups["week"][fmt.Sprintf("%02d", week)] += todo.Effort
			groups["month"][fmt.Sprintf("%02d", int(due.Month()))] += todo.Effort

			for _, tag := range todo.Tags {
				groups["tags"][tag] += todo.Effort
			}
		}
	}

	return groups
}
