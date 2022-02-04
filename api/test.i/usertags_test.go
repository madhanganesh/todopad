package testi

import (
	"encoding/json"
	"net/http"
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/stretchr/testify/assert"
)

func TestUserTags(t *testing.T) {
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

	req, err = http.NewRequest("GET", getURL("usertags"), nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	assert.NoError(t, err)
	res, err = client.Do(req)
	assert.NoError(t, err)

	var tagsRes model.TagsResponse
	err = json.NewDecoder(res.Body).Decode(&tagsRes)
	assert.NoError(t, err)
	defer res.Body.Close()

	assert.Equal(t, []string{"tag1", "tag2"}, tagsRes.Tags)

}

func TestUserTagsNoTagsSet(t *testing.T) {
	setupDB(t)

	loginResponse := setupUser(t, "usr1")
	now := time.Now()
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false, now.UTC(), NoTags)
	req, err := http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	req, err = http.NewRequest("GET", getURL("usertags"), nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	assert.NoError(t, err)
	res, err = client.Do(req)
	assert.NoError(t, err)

	var tagsRes model.TagsResponse
	err = json.NewDecoder(res.Body).Decode(&tagsRes)
	assert.NoError(t, err)
	defer res.Body.Close()

	assert.Equal(t, []string{}, tagsRes.Tags)

}

func TestUserTagsCollectedOverTodos(t *testing.T) {
	setupDB(t)

	loginResponse := setupUser(t, "usr1")
	now := time.Now()
	todoData := getTestTask(t, loginResponse.UserID, "todo-1", false, now.UTC(), []string{"tag1", "tag2"})
	req, err := http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	todoData = getTestTask(t, loginResponse.UserID, "todo-1", false, now.UTC(), []string{"tag2", "tag3"})
	req, err = http.NewRequest("POST", getURL("todo"), todoData)
	assert.NoError(t, err)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	client = http.Client{}
	res, err = client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	req, err = http.NewRequest("GET", getURL("usertags"), nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	assert.NoError(t, err)
	res, err = client.Do(req)
	assert.NoError(t, err)

	var tagsRes model.TagsResponse
	err = json.NewDecoder(res.Body).Decode(&tagsRes)
	assert.NoError(t, err)
	defer res.Body.Close()

	assert.Equal(t, []string{"tag1", "tag2", "tag3"}, tagsRes.Tags)

}
