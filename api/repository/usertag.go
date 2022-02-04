package repository

import (
	"database/sql"
	"log"

	"github.com/madhanganesh/todopad/api/model"
)

type UserTags struct {
	db *sql.DB
}

func NewUserTagsRepository(db *sql.DB) *UserTags {
	return &UserTags{
		db: db,
	}
}

func (r *UserTags) GetTags(userid int64) ([]model.UserTag, error) {
	query := `select userid, tag from usertags where userid = $1`

	rows, err := r.db.Query(query, userid)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return getTagsFromRows(rows)
}

func (r *UserTags) SetUserTags(userid int64, tags []string) {
	statement := `
    insert into usertags (userid, tag)
    values ($1, $2)
    on conflict(userid, tag)
    do nothing
  `
	for _, tag := range tags {
		_, err := r.db.Exec(statement, userid, tag)
		if err != nil {
			log.Printf("Error in setting user tags: %d %+v\n", userid, err)
		}
	}
}

func getTagsFromRows(rows *sql.Rows) ([]model.UserTag, error) {
	usertags := []model.UserTag{}
	for rows.Next() {
		var usertag model.UserTag
		err := rows.Scan(&usertag.UserID, &usertag.Tag)
		if err != nil {
			return nil, err
		}

		usertags = append(usertags, usertag)
	}

	return usertags, nil
}
