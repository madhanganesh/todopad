package model

import "time"

type Todo struct {
	ID     int64     `json:"id"`
	UserID int64     `json:"userid"`
	Title  string    `json:"title"`
	Due    time.Time `json:"due"`
	Done   bool      `json:"done"`
	Effort float32   `json:"effort"`
	Tags   []string  `json:"tags"`
	Notes  string    `json:"notes"`
}
