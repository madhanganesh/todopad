package model

type LoginRequest struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

type LoginResponse struct {
	UserID int64  `json:"userid"`
	Email  string `json:"email"`
	Name   string `json:"name"`
	Token  string `json:"token"`
}
