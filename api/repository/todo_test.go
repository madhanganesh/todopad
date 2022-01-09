package repository

import (
	"testing"
	"time"

	"github.com/madhanganesh/todopad/api/model"
	"github.com/stretchr/testify/assert"
)

func TestCreateTodo(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	if err != nil {
		t.Fatal(err)
	}

	todoRepo := NewTodoRepository(db)
	todo := model.Todo{UserID: user.ID, Title: "test task 1", Due: time.Now(), Effort: 1}
	todo, err = todoRepo.Create(todo)

	assert.NoError(t, err)
	assert.NotEqual(t, 0, todo.ID, "User ID after creation should not be 0")
}

func TestGetTodoByID(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)
	todoRepo := NewTodoRepository(db)
	todo := model.Todo{UserID: user.ID, Title: "test task 1", Due: time.Now().UTC(), Effort: 1, Tags: []string{}}
	todo, err = todoRepo.Create(todo)
	assert.NoError(t, err)

	todoRet, err := todoRepo.GetByID(user.ID, todo.ID)
	assert.NoError(t, err)
	lname, _ := todoRet.Due.Zone()
	if lname == "UTC" {
		todoRet.Due = time.Date(todo.Due.Year(), todo.Due.Month(), todo.Due.Day(), todo.Due.Hour(), todo.Due.Minute(), todo.Due.Second(), todo.Due.Nanosecond(), time.Now().UTC().Location())
	}
	assert.Equal(t, todo, todoRet)
}

func TestGetNonFoundTodoByID(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)

	todoRepo := NewTodoRepository(db)
	_, err = todoRepo.GetByID(user.ID, int64(99))
	assert.Equal(t, ErrNoTodoFound, err)
}

func TestGetPendingTodos(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	if err != nil {
		t.Fatal(err)
	}

	todoRepo := NewTodoRepository(db)
	todo := model.Todo{UserID: user.ID, Title: "test task 1", Done: false, Due: time.Now().UTC(), Effort: 1}
	todoRepo.Create(todo)
	todo = model.Todo{UserID: user.ID, Title: "test task 2", Done: true, Due: time.Now().UTC(), Effort: 1}
	todoRepo.Create(todo)

	pendingTodos, err := todoRepo.GetPending(user.ID)

	assert.NoError(t, err)
	assert.Equal(t, 1, len(pendingTodos))
}

func TestMultiUser(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user1 := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user1, _ = userRepo.Create(user1)
	user2 := model.User{Name: "Test User - 2", Email: "test2@test.com", Password: "password"}
	user2, _ = userRepo.Create(user2)

	todoRepo := NewTodoRepository(db)
	user1Todo := model.Todo{UserID: user1.ID, Title: "test task 1", Due: time.Now(), Effort: 1}
	todoRepo.Create(user1Todo)
	user2Todo := model.Todo{UserID: user2.ID, Title: "test task 2", Due: time.Now(), Effort: 2}
	todoRepo.Create(user2Todo)

	user1Todos, _ := todoRepo.GetPending(user1Todo.UserID)
	assert.Equal(t, 1, len(user1Todos))
	assert.Equal(t, user1Todo.UserID, user1Todos[0].UserID)
}

func TestTodoWithoutUserID(t *testing.T) {
	db := setupdb(t)

	todoRepo := NewTodoRepository(db)
	todo := model.Todo{UserID: 1, Title: "test task 1", Due: time.Now(), Effort: 1}
	todo, err := todoRepo.Create(todo)

	assert.Error(t, err)
}

func TestUpdateTodoForDone(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	if err != nil {
		t.Fatal(err)
	}

	todoRepo := NewTodoRepository(db)
	todo := model.Todo{UserID: user.ID, Title: "test task 1", Due: time.Now(), Effort: 1, Done: false}
	todo, err = todoRepo.Create(todo)
	if err != nil {
		t.Fatal(err)
	}

	updatedTodo := todo
	updatedTodo.Done = true
	err = todoRepo.Update(todo.UserID, todo.ID, updatedTodo)
	assert.NoError(t, err)

	todo, err = todoRepo.GetByID(todo.UserID, todo.ID)
	assert.NoError(t, err)
	assert.Equal(t, true, todo.Done)
}

func TestUpdateTodoForWrongID(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	if err != nil {
		t.Fatal(err)
	}

	todoRepo := NewTodoRepository(db)
	todo := model.Todo{UserID: user.ID, Title: "test task 1", Due: time.Now(), Effort: 1, Done: false}
	todo, err = todoRepo.Create(todo)
	if err != nil {
		t.Fatal(err)
	}

	updatedTodo := todo
	updatedTodo.Done = true
	err = todoRepo.Update(todo.UserID, todo.ID, updatedTodo)
	assert.NoError(t, err)

	todo, err = todoRepo.GetByID(todo.UserID, int64(100))
	assert.Equal(t, ErrNoTodoFound, err)
}

func TestDeleteTodo(t *testing.T) {
	db := setupdb(t)
	userRepo := NewUserRepository(db)
	user := model.User{Name: "Test User", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	if err != nil {
		t.Fatal(err)
	}

	todoRepo := NewTodoRepository(db)
	todo := model.Todo{UserID: user.ID, Title: "test task 1", Due: time.Now(), Effort: 1, Done: false}
	todo, err = todoRepo.Create(todo)
	if err != nil {
		t.Fatal(err)
	}

	err = todoRepo.Delete(todo.UserID, todo.ID)
	assert.NoError(t, err)

	_, err = todoRepo.GetByID(todo.UserID, todo.ID)
	assert.EqualError(t, err, "no todo found")
}
