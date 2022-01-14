package controller

import (
	"errors"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/golang-jwt/jwt"
	"github.com/madhanganesh/todopad/api/model"
)

type todoPadClaims struct {
	jwt.StandardClaims
	UserID int64  `json:"userid"`
	Email  string `json:"email"`
	Name   string `json:"name"`
}

func getJWT(secretKey []byte, user model.User) (string, error) {
	claims := todoPadClaims{
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: time.Now().Add(24 * time.Hour).Unix(),
			//ExpiresAt: time.Now().Add(1 * time.Minute).Unix(),
			Issuer: "todopad",
		},
		UserID: user.ID,
		Email:  user.Email,
		Name:   user.Name,
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(secretKey)
}

func validateToken(accessToken string, signingKey []byte) (*todoPadClaims, error) {
	token, err := jwt.ParseWithClaims(accessToken, &todoPadClaims{}, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return signingKey, nil
	})

	if err != nil {
		if strings.Contains(err.Error(), "token is expired") {
			log.Printf("%v", err)
			return nil, ErrTokenExpired
		}
		return nil, err
	}

	if token.Valid {
		if claims, ok := token.Claims.(*todoPadClaims); ok {
			return claims, nil
		}
	}

	return nil, errors.New("invalid token")
}

var ErrTokenExpired = errors.New("token expired")
