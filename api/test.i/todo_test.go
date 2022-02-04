package testi

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/stretchr/testify/assert"
)

func TestCreateTodo(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false, time.Now().UTC(), NoTags)

	req, err := http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)

	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	var todo model.Todo
	err = json.NewDecoder(res.Body).Decode(&todo)
	assert.NoError(t, err)
	assert.NotEqual(t, int64(0), todo.ID)
}

func TestCreateTodoWithTags(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	now := time.Now()
	tags := []string{"tag1", "tag2"}
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false, now.UTC(), tags)

	req, err := http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)

	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)
	var todo model.Todo
	err = json.NewDecoder(res.Body).Decode(&todo)
	assert.NoError(t, err)

	expected := model.Todo{ID: todo.ID, UserID: loginResponse.UserID, Title: "todo-1", Due: now.UTC(), Effort: 1, Tags: tags}
	assert.Equal(t, expected, todo)
}

func TestGetTodoByID(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false, time.Now().UTC(), NoTags)
	req, err := http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)
	var todo model.Todo
	err = json.NewDecoder(res.Body).Decode(&todo)
	assert.NoError(t, err)
	defer res.Body.Close()

	idStr := strconv.FormatInt(todo.ID, 10)
	req, err = http.NewRequest("GET", getURL("todo")+"/"+idStr, todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, res.StatusCode)
}

func TestPendingTodos(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	client := http.Client{}

	todoData1 := getTestTask(t, loginResponse.UserID, "todo-1", false, time.Now().UTC(), NoTags)
	req, _ := http.NewRequest("POST", getURL("todo"), todoData1)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	todoData2 := getTestTask(t, loginResponse.UserID, "todo-2", true, time.Now().UTC(), NoTags)
	req, _ = http.NewRequest("POST", getURL("todo"), todoData2)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	req, _ = http.NewRequest("GET", getURL("todo")+"?pending=true", nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, res.StatusCode)
}

func TestGetTodosByDateRange(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	client := http.Client{}

	now := time.Now()

	todoData1 := getTestTask(t, loginResponse.UserID, "todo-1", false, now.UTC(), NoTags)
	req, _ := http.NewRequest("POST", getURL("todo"), todoData1)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)
	var todoToday model.Todo
	err = json.NewDecoder(res.Body).Decode(&todoToday)
	assert.NoError(t, err)

	todoData2 := getTestTask(t, loginResponse.UserID, "todo-2", true, now.Add(1*time.Hour*24).UTC(), NoTags)
	req, _ = http.NewRequest("POST", getURL("todo"), todoData2)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	todoData3 := getTestTask(t, loginResponse.UserID, "todo-3", true, now.Add(-1*time.Hour*24).UTC(), NoTags)
	req, _ = http.NewRequest("POST", getURL("todo"), todoData3)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	start := time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, now.Location()).UTC()
	end := time.Date(now.Year(), now.Month(), now.Day(), 23, 59, 59, 999, now.Location()).UTC()
	url := fmt.Sprintf("%s?from=%s&to=%s", getURL("todo"), start.Format(time.RFC3339), end.Format(time.RFC3339))
	req, _ = http.NewRequest("GET", url, nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, res.StatusCode)

	var todos []model.Todo
	err = json.NewDecoder(res.Body).Decode(&todos)
	defer res.Body.Close()
	assert.NoError(t, err)
	assert.Equal(t, 1, len(todos))
	if len(todos) > 0 {
		assert.Equal(t, todoToday.ID, todos[0].ID)
	}

	yesterday := now.Add(-1 * time.Hour * 24)
	start = time.Date(yesterday.Year(), yesterday.Month(), yesterday.Day(), 0, 0, 0, 0, yesterday.Location()).UTC()
	tomorrow := now.Add(1 * time.Hour * 24)
	end = time.Date(tomorrow.Year(), tomorrow.Month(), tomorrow.Day(), 23, 59, 59, 999, tomorrow.Location()).UTC()
	url = fmt.Sprintf("%s?from=%s&to=%s", getURL("todo"), start.Format(time.RFC3339), end.Format(time.RFC3339))
	req, _ = http.NewRequest("GET", url, nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, res.StatusCode)

	err = json.NewDecoder(res.Body).Decode(&todos)
	defer res.Body.Close()
	assert.NoError(t, err)
	assert.Equal(t, 3, len(todos))
	if len(todos) > 0 {
		assert.Equal(t, todoToday.ID, todos[0].ID)
	}
}

func TestGetTodsWithNoFilter(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")

	client := http.Client{}
	req, _ := http.NewRequest("GET", getURL("todo")+"?pending", nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusBadRequest, res.StatusCode)

	var errorObject model.ErrorObject
	err = json.NewDecoder(res.Body).Decode(&errorObject)
	assert.NoError(t, err)
	defer res.Body.Close()
	errorObjectExpected := model.ErrorObject{
		Message: "Get todo request needs a query param (peding or date range)",
	}
	assert.Equal(t, errorObjectExpected, errorObject)
}

func TestToggleDone(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false, time.Now().UTC(), NoTags)

	req, err := http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)

	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	var todoRet model.Todo
	err = json.NewDecoder(res.Body).Decode(&todoRet)
	defer res.Body.Close()
	assert.NoError(t, err)

	idStr := strconv.FormatInt(todoRet.ID, 10)
	todoRet.Done = !todoRet.Done

	var temp bytes.Buffer
	json.NewEncoder(&temp).Encode(todoRet)
	req, err = http.NewRequest("PUT", getURL("todo")+"/"+idStr, bytes.NewReader(temp.Bytes()))
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)

	_, err = client.Do(req)
	assert.NoError(t, err)

	req, err = http.NewRequest("GET", getURL("todo")+"/"+idStr, todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, res.StatusCode)

	err = json.NewDecoder(res.Body).Decode(&todoRet)
	assert.NoError(t, err)
	defer res.Body.Close()
	assert.Equal(t, true, todoRet.Done)
}

func TestDeleteTodo(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false, time.Now().UTC(), NoTags)
	req, err := http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)
	var todoRet model.Todo
	err = json.NewDecoder(res.Body).Decode(&todoRet)
	defer res.Body.Close()
	assert.NoError(t, err)

	idStr := strconv.FormatInt(todoRet.ID, 10)
	req, err = http.NewRequest("DELETE", getURL("todo")+"/"+idStr, nil)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, res.StatusCode)
}
