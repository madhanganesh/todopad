package model

type UserTag struct {
	UserID int64  `json:"userid"`
	Tag    string `json:"tag"`
}

type TagsResponse struct {
	Tags []string `json:"tags"`
}
