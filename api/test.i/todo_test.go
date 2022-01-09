package testi

import (
	"bytes"
	"encoding/json"
	"net/http"
	"strconv"
	"testing"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/stretchr/testify/assert"
)

func TestCreateTodo(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false)

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

func TestGetTodoByID(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false)
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

	/*var todoRet model.Todo
	err = json.NewDecoder(res.Body).Decode(&todoRet)
	assert.NoError(t, err)
	defer res.Body.Close()
	assert.Equal(t, int64(1), todoRet.ID)*/
}

func TestPendingTodos(t *testing.T) {
	setupDB(t)
	loginResponse := setupUser(t, "usr1")
	client := http.Client{}

	todoData1 := getTestTask(t, loginResponse.UserID, "todo-1", false)
	req, _ := http.NewRequest("POST", getURL("todo"), todoData1)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	todoData2 := getTestTask(t, loginResponse.UserID, "todo-2", true)
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
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false)

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
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false)
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
