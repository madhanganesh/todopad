use chrono::NaiveDate;
use sqlx::{query_as, SqlitePool, Error as SqlxError};
use thiserror::Error;

use crate::models::{Tag, Todo, User};

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
}  

pub async fn register_user(pool: &SqlitePool, email: &String, password_hash: &String) -> Result<i64, RegisterError> {
    let result = query_as!(User, "INSERT INTO users (email, password_hash) VALUES (?, ?)",
        email,
        password_hash
    )
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    let last_insert_id = result.last_insert_rowid();
    Ok(last_insert_id)
}

pub async fn get_user_from_email(pool: &SqlitePool, email: &String) -> Result<User, sqlx::Error> {
    query_as!(User, "SELECT id, email, password_hash from users WHERE email=?", email)
        .fetch_one(pool)
        .await
}

pub async fn get_pending_todos(pool: &SqlitePool, user_id: i64) -> Result<Vec<Todo>, sqlx::Error> {
    query_as!(Todo, "SELECT * FROM todos WHERE user_id = ? AND completed = false", user_id)
        .fetch_all(pool)
        .await
}

pub async fn get_todos_for_date(pool: &SqlitePool, user_id: i64, date: &NaiveDate) -> Result<Vec<Todo>, sqlx::Error> {
    query_as!(Todo, "SELECT * FROM todos WHERE user_id = ? AND due = ?", user_id, date)
        .fetch_all(pool)
        .await
}

pub async fn get_todo(pool: &SqlitePool, user_id: i64, todo_id: i64) 
    -> Result<Todo, sqlx::Error> {
    let todo = query_as!(
        Todo, 
        "SELECT * from todos where user_id=? and id=?", 
        user_id, todo_id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn create_todo(pool: &SqlitePool, user_id: i64, title: &str, due: &NaiveDate) -> Result<Todo, sqlx::Error> {
    let result = query_as!(Todo, "INSERT INTO todos (user_id, title, due) VALUES (?, ?, ?)", user_id, title, due)
        .execute(pool)
        .await?;
    let last_insert_id = result.last_insert_rowid();

    let todo = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE id = ?",
        last_insert_id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn update_todo(pool: &SqlitePool, user_id: i64, todo_id: i64, title: String, due: Option<NaiveDate>, completed: bool, notes: Option<String>) -> Result<(), sqlx::Error> {
    query_as!(Todo, 
        r#"
        UPDATE todos SET title=?, due=?, completed=?, notes=?
        WHERE user_id=? AND id=?
        "#,
        title,
        due, 
        completed,
        notes,
        user_id,
        todo_id
        )
        .execute(pool)
        .await?;
        
    Ok(())
}

pub async fn delete_todo(pool: &SqlitePool, user_id: i64, todo_id: i64) -> Result<(), sqlx::Error> {
    query_as!(Todo, "DELETE from todos where user_id=? and id=?", user_id, todo_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn toggle_todo(pool: &SqlitePool, user_id: i64, todo_id: i64) -> Result<(), sqlx::Error> {
    query_as!(Todo, "UPDATE todos SET completed = NOT completed where user_id=? and id=?", user_id, todo_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn save_tags(pool: &SqlitePool, user_id: i64, todo_id: i64, tags: Vec<String>) -> Result<(), sqlx::Error> {

    query_as!(Tag,"DELETE FROM tags WHERE user_id=? and todo_id=?", user_id, todo_id)
        .execute(pool)
        .await?;

    for tag in &tags {
        query_as!(Tag,
            r#"
            INSERT INTO tags (user_id, todo_id, tag)
            VALUES (?, ?, ?)
            ON CONFLICT(user_id, todo_id, tag) DO NOTHING
            "#,
            user_id,
            todo_id,
            tag
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_tags_for_todo(pool: &SqlitePool, user_id: i64, todo_id: i64) -> Result<Vec<String>, sqlx::Error> {

    let tags = query_as!(Tag, 
        r#"select user_id, todo_id, tag 
        from tags 
        where user_id = ? and todo_id = ?
        "#, 
        user_id, 
        todo_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| r.tag)
    .collect();

    Ok(tags)
}

fn map_sqlx_error(err: SqlxError) -> RegisterError {
    if let SqlxError::Database(db_err) = &err {
        if db_err.message().contains("UNIQUE constraint failed") {
            return RegisterError::UserAlreadyExists;
        }
    }
    RegisterError::DatabaseError(err)
}
