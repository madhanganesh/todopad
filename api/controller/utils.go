package controller

import (
	"encoding/json"
	"log"
	"net/http"
)

type errorObject struct {
	Message string `json:"error"`
}

func handleError(err error, w http.ResponseWriter, statusCode int, message string) {
	if message == "" {
		log.Printf("Error: %v", err)
		message = err.Error()
	} else {
		log.Printf("%s: %v", message, err)
	}
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(statusCode)
	obj := errorObject{
		Message: message,
	}
	json.NewEncoder(w).Encode(obj)
}
