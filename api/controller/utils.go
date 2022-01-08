package controller

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strconv"

	"github.com/go-chi/chi"
	"github.com/madhanganesh/todopad/api/model"
)

func handleError(err error, w http.ResponseWriter, statusCode int, message string) {
	if message == "" {
		log.Printf("Error: %v", err)
		message = err.Error()
	} else {
		log.Printf("%s: %v", message, err)
	}
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(statusCode)
	obj := model.ErrorObject{
		Message: message,
	}
	err1 := json.NewEncoder(w).Encode(obj)
	if err1 != nil {
		log.Printf("%v", err1)
	}
}

func getIDFromURLPath(r *http.Request) (int64, error) {
	idStr := chi.URLParam(r, "id")
	if idStr == "" {
		return int64(0), fmt.Errorf("missing ID in URL Path")
	}
	id, err := strconv.ParseInt(idStr, 10, 64)
	if err != nil {
		return int64(0), fmt.Errorf("invalid ID in URL Path")
	}

	return id, err
}
