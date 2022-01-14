package repository

import (
	"database/sql"
	"errors"
	"fmt"
	"strings"

	"github.com/madhanganesh/todopad/api/model"
)

type User struct {
	db *sql.DB
}

func NewUserRepository(db *sql.DB) *User {
	return &User{
		db: db,
	}
}

func (r *User) Create(user model.User) (model.User, error) {
	query := `insert into users (name, email, hpassword) values ($1, $2, $3) returning id`

	var id int64
	err := r.db.QueryRow(query, user.Name, user.Email, user.Password).Scan(&id)
	if err != nil {
		if strings.Contains(err.Error(), "UNIQUE constraint failed") {
			return model.User{}, ErrEmailExists
		}
		return model.User{}, fmt.Errorf("error in repository.User::Create when inserting a user: %w", err)
	}
	user.ID = id
	return user, nil
}

func (r *User) Get(email string) (model.User, error) {
	query := `select id, name, email, hpassword from users where email=$1`
	row := r.db.QueryRow(query, email)

	var user model.User
	err := row.Scan(&user.ID, &user.Name, &user.Email, &user.Password)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return model.User{}, ErrNoUserExists
		}
		return model.User{}, fmt.Errorf("error in repository.User::Get when selecting a user for email %s: %w", email, err)
	}

	return user, nil
}

func (r *User) GetByID(id int64) (model.User, error) {
	query := `select id, name, email, hpassword from users where id=$1`
	row := r.db.QueryRow(query, id)

	var user model.User
	err := row.Scan(&user.ID, &user.Name, &user.Email, &user.Password)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return model.User{}, ErrNoUserExists
		}
		return model.User{}, fmt.Errorf("error in repository.User::Get when selecting a user for ID %d: %w", id, err)
	}

	return user, nil
}

var ErrEmailExists = errors.New("email already exists")
var ErrNoUserExists = errors.New("user is not registered")
