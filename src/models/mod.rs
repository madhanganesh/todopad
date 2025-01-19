use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub due: Option<NaiveDate>,
    pub completed: bool,
    pub notes: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct Tag {
    pub user_id: i64,
    pub todo_id: i64,
    pub tag: String,
}
