package controller

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/madhanganesh/todopad/api/repository"
)

type Report struct {
	todoRepository *repository.Todo
}

func NewReportContoller(todoRepository *repository.Todo) *Report {
	return &Report{
		todoRepository: todoRepository,
	}
}

func (t *Report) GetAdhoc(w http.ResponseWriter, r *http.Request) {
	userid, err := getUserID(r)
	if err != nil {
		handleError(err, w, http.StatusUnauthorized, "")
		return
	}

	var reportRequest model.ReportRequest
	err = json.NewDecoder(r.Body).Decode(&reportRequest)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "invalid request json")
		return
	}
	defer r.Body.Close()

	reportResponse, err := t.todoRepository.GetReport(userid, reportRequest)
	if err != nil {
		handleError(err, w, http.StatusInternalServerError, "")
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	err = json.NewEncoder(w).Encode(reportResponse)
	if err != nil {
		log.Printf("Error: %v", err)
	}
}
