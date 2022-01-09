package repository

import (
	"database/sql"
	"errors"
	"fmt"
	"log"
	"strings"

	"github.com/madhanganesh/todopad/api/model"
)

// TodoRepository struct
type Todo struct {
	db *sql.DB
}

// Init method
func NewTodoRepository(db *sql.DB) *Todo {
	return &Todo{
		db: db,
	}
}

func (r *Todo) Create(todo model.Todo) (model.Todo, error) {
	if todo.UserID == 0 || todo.Due.IsZero() || todo.Effort == 0 {
		return model.Todo{}, ErrInvalidTodo
	}

	tags := strings.Join(todo.Tags, ";")
	query := `
    insert into todos (userid, title, due, done, effort, tags, notes)
    values ($1, $2, $3, $4, $5, $6, $7) returning id`

	row := r.db.QueryRow(query, todo.UserID, todo.Title, todo.Due, todo.Done, todo.Effort, tags, todo.Notes)
	var id int64
	if err := row.Scan(&id); err != nil {
		return model.Todo{}, fmt.Errorf("error in repository.Todo::Create when inserting a todo: %w", err)
	}
	todo.ID = id
	return todo, nil
}

func (repo *Todo) GetByID(userid int64, id int64) (model.Todo, error) {
	query := `select id, userid, title, due, done, effort, tags, notes
    			from todos
    			where userid = $1 and id = $2`

	log.Printf("TodoRepository::GetByID with UserID: %d, TodoID:%d", userid, id)
	row := repo.db.QueryRow(query, userid, id)
	var tags string
	var todo model.Todo
	err := row.Scan(&todo.ID, &todo.UserID, &todo.Title, &todo.Due, &todo.Done, &todo.Effort, &tags, &todo.Notes)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return model.Todo{}, ErrNoTodoFound
		}
		return model.Todo{}, err
	}

	todo.Tags = []string{}
	if len(tags) > 0 {
		todo.Tags = strings.Split(tags, ";")
	}
	return todo, nil
}

func (repo *Todo) GetPending(userid int64) ([]model.Todo, error) {
	query := `
    select id, userid, title, due, done, effort, tags, notes
    from todos where userid = $1 and done = $2`
	rows, err := repo.db.Query(query, userid, false)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return getTodosFromRows(rows)
}

// GetTodosByDateRange method
func (repo *Todo) GetTodosByDateRange(userid string, from string, to string) ([]model.Todo, error) {
	query := `
    select id, userid, title, due, completed, effort, tags, notes
    from todos
    where userid = $1 and (due > $2 and due < $3)
  `
	rows, err := repo.db.Query(query, userid, from, to)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return getTodosFromRows(rows)
}

func (repo *Todo) Update(userid int64, id int64, todo model.Todo) error {
	query := `
    update todos
    set title=$1, due=$2, done=$3, effort=$4, tags=$5, notes=$6
    where userid=$7 and id=$8
  `

	tags := strings.Join(todo.Tags, ";")
	res, err := repo.db.Exec(query, todo.Title, todo.Due, todo.Done, todo.Effort, tags, todo.Notes, userid, id)
	if err != nil {
		return err
	}

	count, err := res.RowsAffected()
	if err != nil {
		return err
	}
	if count == 0 {
		return ErrNoTodoFound
	}
	if count != 1 {
		return fmt.Errorf("more than 1 record got updated for %d", id)
	}

	return nil
}

func (repo *Todo) Delete(userid int64, id int64) error {
	query := `delete from todos where userid=$1 and id=$2`
	res, err := repo.db.Exec(query, userid, id)
	if err != nil {
		return err
	}

	count, err := res.RowsAffected()
	if err != nil {
		return err
	}
	if count == 0 {
		return ErrNoTodoFound
	}
	if count != 1 {
		return fmt.Errorf("exactly 1 row is not impacted for %d", id)
	}

	return nil
}

func getTodosFromRows(rows *sql.Rows) ([]model.Todo, error) {
	todos := []model.Todo{}
	for rows.Next() {
		var todo model.Todo
		var tags string
		//var duestr string
		err := rows.Scan(&todo.ID, &todo.UserID, &todo.Title, &todo.Due, &todo.Done, &todo.Effort, &tags, &todo.Notes)
		if err != nil {
			return nil, err
		}
		/*todo.Due, err = time.Parse("2006-01-02 15:04:05", duestr)
		if err != nil {
			return nil, err
		}*/

		todo.Tags = []string{}
		if len(tags) > 0 {
			todo.Tags = strings.Split(tags, ";")
		}
		todos = append(todos, todo)
	}

	return todos, nil
}

var ErrInvalidTodo = errors.New("UserID, Effort and Due should not be empty")
var ErrNoTodoFound = errors.New("no todo found")
