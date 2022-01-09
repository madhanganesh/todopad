package controller

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"net/http"
	"strconv"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/madhanganesh/todopad/api/repository"
)

type Todo struct {
	todoRepository *repository.Todo
}

func NewTodoContoller(todoRepository *repository.Todo) *Todo {
	return &Todo{
		todoRepository: todoRepository,
	}
}

func (t *Todo) Create(w http.ResponseWriter, r *http.Request) {
	userid, err := getUserID(r)
	if err != nil {
		handleError(err, w, http.StatusUnauthorized, "")
		return
	}

	var todo model.Todo
	err = json.NewDecoder(r.Body).Decode(&todo)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "Unable to parse JSON request")
		return
	}
	defer r.Body.Close()
	todo.UserID = userid

	todo, err = t.todoRepository.Create(todo)
	if err != nil {
		if errors.Is(err, repository.ErrInvalidTodo) {
			handleError(err, w, http.StatusBadRequest, "")
			return
		}
		handleError(err, w, http.StatusInternalServerError, "Internal Error. Try later or check with admin.")
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	err = json.NewEncoder(w).Encode(todo)
	if err != nil {
		log.Printf("Error: %v", err)
	}
	log.Printf("Todo created for user: %d, todo ID: %d", todo.UserID, todo.ID)
}

func (t *Todo) GetByID(w http.ResponseWriter, r *http.Request) {
	id, err := getIDFromURLPath(r)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "")
		return
	}

	userid, err := getUserID(r)
	if err != nil {
		handleError(err, w, http.StatusUnauthorized, "")
		return
	}

	todo, err := t.todoRepository.GetByID(userid, id)
	if err != nil {
		if errors.Is(err, repository.ErrNoTodoFound) {
			handleError(err, w, http.StatusBadRequest, fmt.Sprintf("todo with %d not found", id))
			return
		}
		handleError(err, w, http.StatusInternalServerError, fmt.Sprintf("Error in retrieving todo with ID: %d", id))
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	err = json.NewEncoder(w).Encode(todo)
	if err != nil {
		log.Printf("Error: %v", err)
	}
}

func (t *Todo) Get(w http.ResponseWriter, r *http.Request) {
	userid, err := getUserID(r)
	if err != nil {
		handleError(err, w, http.StatusUnauthorized, "")
		return
	}

	var todos []model.Todo
	pendings := r.URL.Query().Get("pending")
	if pendings == "true" {
		log.Printf("invoking GetPendingTasks for user %d\n", userid)
		todos, err = t.todoRepository.GetPending(userid)
		if err != nil {
			handleError(err, w, http.StatusInternalServerError, "Error reading pending todos")
			return
		}
		w.WriteHeader(http.StatusOK)
		w.Header().Set("Content-Type", "application/json; charset=UTF-8")
		err = json.NewEncoder(w).Encode(todos)
		if err != nil {
			log.Printf("Error: %v", err)
		}

		return
	}

	handleError(fmt.Errorf("Get todo request needs a query param (peding or date range)"), w, http.StatusBadRequest, "")
}

func (t *Todo) Update(w http.ResponseWriter, r *http.Request) {
	userid, err := getUserID(r)
	if err != nil {
		handleError(err, w, http.StatusUnauthorized, "")
		return
	}

	var todo model.Todo
	err = json.NewDecoder(r.Body).Decode(&todo)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "Unable to parse JSON request")
		return
	}
	defer r.Body.Close()
	todo.UserID = userid

	err = t.todoRepository.Update(userid, todo.ID, todo)
	if err != nil {
		if errors.Is(err, repository.ErrNoTodoFound) {
			handleError(err, w, http.StatusBadRequest, "")
			return
		}
		handleError(err, w, http.StatusInternalServerError, "Internal Error. Try later or check with admin.")
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (t *Todo) Delete(w http.ResponseWriter, r *http.Request) {
	userid, err := getUserID(r)
	if err != nil {
		handleError(err, w, http.StatusUnauthorized, "")
		return
	}

	id, err := getIDFromURLPath(r)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "")
		return
	}

	err = t.todoRepository.Delete(userid, id)
	if err != nil {
		if errors.Is(err, repository.ErrNoTodoFound) {
			handleError(err, w, http.StatusBadRequest, "")
			return
		}
		handleError(err, w, http.StatusInternalServerError, "Internal Error. Try later or check with admin.")
		return
	}

	w.WriteHeader(http.StatusOK)
}

func getUserID(r *http.Request) (int64, error) {
	useridstr := r.Header.Get("userid")
	if useridstr == "" {
		return 0, errors.New("UserID not found in request")
	}
	userid, err := strconv.ParseInt(useridstr, 10, 64)
	if err != nil {
		return 0, err
	}

	return userid, nil
}
