package controller

import (
	"errors"
	"fmt"
	"time"

	"github.com/golang-jwt/jwt"
	"github.com/madhanganesh/todopad/api/model"
)

type todoPadClaims struct {
	jwt.StandardClaims
	UserID int64  `json:"userid"`
	Email  string `json:"email"`
}

func getJWT(secretKey []byte, user model.User) (string, error) {
	claims := todoPadClaims{
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: time.Now().Add(24 * time.Hour).Unix(),
			Issuer:    "todopad",
		},
		UserID: user.ID,
		Email:  user.Email,
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
		return nil, err
	}

	if token.Valid {
		if claims, ok := token.Claims.(*todoPadClaims); ok {
			return claims, nil
		}
	}

	return nil, errors.New("invalid token")
}
