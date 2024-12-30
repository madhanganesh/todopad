use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    id: String,
    hashed_password: String,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub user_id: String,
    pub title: String,
    pub due: Option<NaiveDate>,
    pub completed: bool,
    pub notes: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct Tag {
    pub user_id: String,
    pub todo_id: i64,
    pub tag: String,
}
