package repository

import (
	"database/sql"
	"errors"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/madhanganesh/todopad/api/model"
)

type Todo struct {
	db *sql.DB
}

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

func (repo *Todo) GetByDateRange(userid int64, from time.Time, to time.Time) ([]model.Todo, error) {
	query := `
    select id, userid, title, due, done, effort, tags, notes
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

func (repo *Todo) GetReport(userid int64, req model.ReportRequest) (model.ReportResponse, error) {
	if req.From.After(req.To) {
		return model.ReportResponse{}, fmt.Errorf("from date is later than to in request")
	}

	if req.GroupBy != "day" && req.GroupBy != "week" && req.GroupBy != "month" && req.GroupBy != "tags" {
		return model.ReportResponse{}, fmt.Errorf("invalid group-by key in request: %s. Supported are day, week, month, tags", req.GroupBy)
	}

	if req.GroupBy == "tags" && len(req.Tags) <= 0 {
		return model.ReportResponse{}, fmt.Errorf("no tags in request to group by tags")
	}

	if req.GroupBy == "tags" {
		return repo.getGroupedByTags(userid, req)
	}

	query := `
    select strftime('%Y-%m-%d', datetime(due, $1)) as day, sum(effort)
    from todos
    where userid = $2 and (due > $3 and due < $4)
	group by day
  `

	if req.GroupBy == "week" {
		query = `
    select strftime('%W', datetime(due, $1)) as week, sum(effort)
    from todos
    where userid = $2 and (due > $3 and due < $4)
	group by week
  `
	}

	if req.GroupBy == "month" {
		query = `
    select strftime('%m', datetime(due, $1)) as month, sum(effort)
    from todos
    where userid = $2 and (due > $3 and due < $4)
	group by month
  `
	}

	offset := fmt.Sprintf("%d seconds", req.TimeZoneOffsetInSecs)
	rows, err := repo.db.Query(query, offset, userid, req.From, req.To)
	if err != nil {
		return model.ReportResponse{}, err
	}
	defer rows.Close()

	var res model.ReportResponse
	res.ReportRequest = req
	res.EffortsByGroup = map[string]float32{}
	for rows.Next() {
		var day string
		var sum float32
		err = rows.Scan(&day, &sum)
		if err != nil {
			return model.ReportResponse{}, err
		}
		res.EffortsByGroup[day] = sum
	}

	return res, nil
}

func getTodosFromRows(rows *sql.Rows) ([]model.Todo, error) {
	todos := []model.Todo{}
	for rows.Next() {
		var todo model.Todo
		var tags string
		err := rows.Scan(&todo.ID, &todo.UserID, &todo.Title, &todo.Due, &todo.Done, &todo.Effort, &tags, &todo.Notes)
		if err != nil {
			return nil, err
		}
		todo.Tags = []string{}
		if len(tags) > 0 {
			todo.Tags = strings.Split(tags, ";")
		}
		todos = append(todos, todo)
	}

	return todos, nil
}

func (repo *Todo) getGroupedByTags(userid int64, req model.ReportRequest) (model.ReportResponse, error) {
	query := `
    select effort, tags
    from todos
    where userid = $1 and (due > $2 and due < $3)
  `
	rows, err := repo.db.Query(query, userid, req.From, req.To)
	if err != nil {
		return model.ReportResponse{}, err
	}
	defer rows.Close()

	var res model.ReportResponse
	res.ReportRequest = req
	res.EffortsByGroup = map[string]float32{}
	for rows.Next() {
		var effort float32
		var tagss string
		err = rows.Scan(&effort, &tagss)
		if err != nil {
			return model.ReportResponse{}, err
		}

		tags := strings.Split(tagss, ";")
		for _, tag := range tags {
			var found bool = false
			for _, reqTag := range req.Tags {
				if reqTag == tag {
					found = true
					break
				}
			}

			if found {
				res.EffortsByGroup[tag] += effort
			}
		}
	}

	return res, err
}

var ErrInvalidTodo = errors.New("UserID, Effort and Due should not be empty")
var ErrNoTodoFound = errors.New("no todo found")
