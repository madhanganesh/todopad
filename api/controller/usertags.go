package controller

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/madhanganesh/todopad/api/repository"
)

type UserTags struct {
	userTagsRepository *repository.UserTags
}

func NewUserTagsContoller(userTagsRepository *repository.UserTags) *UserTags {
	return &UserTags{
		userTagsRepository: userTagsRepository,
	}
}

func (c *UserTags) GetUserTags(w http.ResponseWriter, r *http.Request) {
	userid, err := getUserID(r)
	if err != nil {
		handleError(err, w, http.StatusUnauthorized, "")
		return
	}

	usertags, err := c.userTagsRepository.GetTags(userid)
	if err != nil {
		handleError(err, w, http.StatusInternalServerError, "")
		return
	}

	tags := []string{}
	for _, usertag := range usertags {
		tags = append(tags, usertag.Tag)
	}

	w.WriteHeader(http.StatusOK)
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	err = json.NewEncoder(w).Encode(model.TagsResponse{Tags: tags})
	if err != nil {
		log.Printf("Error: %v", err)
	}
}
