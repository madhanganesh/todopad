package testi

import (
	"encoding/json"
	"net/http"
	"strings"
	"testing"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/stretchr/testify/assert"
)

func TestSignup(t *testing.T) {
	setupDB(t)
	signUpURL := "http://localhost:" + appConfig.Port + "/signup"

	reqStr := `{"email":"test@test.com", "name": "Test User", "password": "password"}`
	req, err := http.NewRequest("POST", signUpURL, strings.NewReader(reqStr))
	assert.NoError(t, err)

	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)
}

func TestDuplicateSignup(t *testing.T) {
	setupDB(t)
	signUpURL := "http://localhost:" + appConfig.Port + "/signup"

	reqStr := `{"email":"test@test.com", "name": "Test User", "password": "password"}`
	req, err := http.NewRequest("POST", signUpURL, strings.NewReader(reqStr))
	assert.NoError(t, err)

	client := http.Client{}
	res, err := client.Do(req)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, res.StatusCode)

	req, _ = http.NewRequest("POST", signUpURL, strings.NewReader(reqStr))
	res, _ = client.Do(req)
	assert.Equal(t, http.StatusConflict, res.StatusCode)
}

func TestLogin(t *testing.T) {
	setupDB(t)
	signUpURL := "http://localhost:" + appConfig.Port + "/signup"
	loginURL := "http://localhost:" + appConfig.Port + "/login"
	pingURL := "http://localhost:" + appConfig.Port + "/secureping"

	client := http.Client{}

	reqStr := `{"email":"test@test.com", "name": "Test User", "password": "password"}`
	req, _ := http.NewRequest("POST", signUpURL, strings.NewReader(reqStr))
	client.Do(req)

	reqStr = `{"email": "test@test.com", "password": "password"}`
	req, _ = http.NewRequest("POST", loginURL, strings.NewReader(reqStr))
	res, _ := client.Do(req)
	assert.Equal(t, http.StatusOK, res.StatusCode)

	var loginResponse model.LoginResponse
	err := json.NewDecoder(res.Body).Decode(&loginResponse)
	assert.NoError(t, err)
	defer res.Body.Close()

	req, _ = http.NewRequest("GET", pingURL, nil)
	req.Header.Set("Authorization", "Bearer "+loginResponse.Token)
	res, _ = client.Do(req)
	assert.Equal(t, http.StatusOK, res.StatusCode)

	var user model.User
	err = json.NewDecoder(res.Body).Decode(&user)
	assert.NoError(t, err)
	defer res.Body.Close()

	assert.Equal(t, "test@test.com", user.Email)
	assert.Equal(t, loginResponse.UserID, user.ID)
}
