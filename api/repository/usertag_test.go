package repository

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/madhanganesh/todopad/api/model"
)

func TestCreateUserTags(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Testusr1", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)

	userTagsRepo := NewUserTagsRepository(db)
	userTagsRepo.SetUserTags(user.ID, []string{"tag1", "tag2"})

	usertags, err := userTagsRepo.GetTags(user.ID)
	assert.NoError(t, err)
	expected := []model.UserTag{{UserID: user.ID, Tag: "tag1"}, {UserID: user.ID, Tag: "tag2"}}
	assert.Equal(t, expected, usertags)
}

func TestSetEmptyUserTags(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Testusr1", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)

	userTagsRepo := NewUserTagsRepository(db)
	userTagsRepo.SetUserTags(user.ID, []string{})

	usertags, err := userTagsRepo.GetTags(user.ID)
	assert.NoError(t, err)
	expected := []model.UserTag{}
	assert.Equal(t, expected, usertags)
}

func TestSetDuplicateUserTags(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Testusr1", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)

	userTagsRepo := NewUserTagsRepository(db)
	userTagsRepo.SetUserTags(user.ID, []string{"tag1", "tag1"})

	usertags, err := userTagsRepo.GetTags(user.ID)
	assert.NoError(t, err)
	expected := []model.UserTag{{UserID: user.ID, Tag: "tag1"}}
	assert.Equal(t, expected, usertags)
}

func TestSettingExistingTags(t *testing.T) {
	db := setupdb(t)

	userRepo := NewUserRepository(db)
	user := model.User{Name: "Testusr1", Email: "test@test.com", Password: "password"}
	user, err := userRepo.Create(user)
	assert.NoError(t, err)

	userTagsRepo := NewUserTagsRepository(db)
	userTagsRepo.SetUserTags(user.ID, []string{"tag1", "tag2"})
	userTagsRepo.SetUserTags(user.ID, []string{"tag2", "tag3"})

	usertags, err := userTagsRepo.GetTags(user.ID)
	assert.NoError(t, err)
	expected := []model.UserTag{{UserID: user.ID, Tag: "tag1"}, {UserID: user.ID, Tag: "tag2"}, {UserID: user.ID, Tag: "tag3"}}
	assert.Equal(t, expected, usertags)
}

func TestNoUser(t *testing.T) {
	db := setupdb(t)

	userTagsRepo := NewUserTagsRepository(db)
	userTagsRepo.SetUserTags(int64(100), []string{"tag1", "tag2"})
}
