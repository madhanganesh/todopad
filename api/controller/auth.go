package controller

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"net/http"
	"strings"

	"golang.org/x/crypto/bcrypt"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/madhanganesh/todopad/api/repository"
)

type Auth struct {
	userRepository *repository.User
	signingKey     []byte
}

func NewAuthController(userRepository *repository.User, secretKey []byte) *Auth {
	return &Auth{
		userRepository: userRepository,
	}
}

func (c *Auth) SignUpUser(w http.ResponseWriter, r *http.Request) {
	var user model.User
	err := json.NewDecoder(r.Body).Decode(&user)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "Invalid JSON input")
		return
	}
	defer r.Body.Close()

	if user.Email == "" || user.Password == "" {
		handleError(err, w, http.StatusBadRequest, "Empty values in Signup request")
		return
	}

	user.Password, err = hashPassword(user.Password)
	if err != nil {
		handleError(err, w, http.StatusInternalServerError, "System error. Retry after some time")
		return
	}

	user, err = c.userRepository.Create(user)
	if err != nil {
		code := http.StatusInternalServerError
		if errors.Is(err, repository.ErrEmailExists) {
			code = http.StatusConflict
		}
		handleError(err, w, code, "")
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	json.NewEncoder(w).Encode(user)
}

func (c *Auth) Login(w http.ResponseWriter, r *http.Request) {
	var credential model.LoginRequest
	err := json.NewDecoder(r.Body).Decode(&credential)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "Invalid JSON input")
		return
	}
	defer r.Body.Close()

	if credential.Email == "" || credential.Password == "" {
		handleError(err, w, http.StatusBadRequest, "Empty values in Signup request")
		return
	}

	user, err := c.userRepository.Get(credential.Email)
	if err != nil {
		code := http.StatusInternalServerError
		if errors.Is(err, repository.ErrNoUserExists) {
			code = http.StatusNotFound
		}
		handleError(err, w, code, "")
		return
	}

	if !checkPasswordHash(credential.Password, user.Password) {
		handleError(errors.New("invalid creditials"), w, http.StatusForbidden, "")
		return
	}

	token, err := getJWT(c.signingKey, user)
	if err != nil {
		handleError(err, w, http.StatusBadRequest, "error in creating security token")
	}

	loginResponse := model.LoginResponse{
		UserID: user.ID,
		Email:  user.Email,
		Token:  token,
	}
	w.WriteHeader(200)
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	err = json.NewEncoder(w).Encode(loginResponse)
	if err != nil {
		log.Printf("Error: %v", err)
	}
}

func (c *Auth) Middleware(handler http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header["Authorization"]
		if authHeader == nil {
			handleError(errors.New("no Authorization header"), w, http.StatusUnauthorized, "")
			return
		}

		authHeaderParts := strings.Split(authHeader[0], " ")
		if len(authHeaderParts) != 2 || strings.ToLower(authHeaderParts[0]) != "bearer" {
			handleError(errors.New("invalid auth token format"), w, http.StatusUnauthorized, "")
			return
		}

		claims, err := validateToken(authHeaderParts[1], c.signingKey)
		if err != nil {
			handleError(err, w, http.StatusUnauthorized, "invalid auth token")
			return
		}

		r.Header.Set("userid", fmt.Sprintf("%d", claims.UserID))
		r.Header.Set("email", claims.Email)

		handler(w, r)
	}
}

func hashPassword(password string) (string, error) {
	bytes, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	return string(bytes), err
}

func checkPasswordHash(password, hash string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hash), []byte(password))
	return err == nil
}
