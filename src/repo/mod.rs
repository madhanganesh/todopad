use chrono::NaiveDate;
use sqlx::{query_as, SqlitePool};

use crate::models::{Todo, Tag};

pub async fn get_pending_todos(pool: &SqlitePool, user_id: &str) -> Result<Vec<Todo>, sqlx::Error> {
    query_as!(Todo, "SELECT * FROM todos WHERE user_id = ? AND completed = false", user_id)
        .fetch_all(pool)
        .await
}

pub async fn get_todos_for_date(pool: &SqlitePool, user_id: &str, date: &NaiveDate) -> Result<Vec<Todo>, sqlx::Error> {
    query_as!(Todo, "SELECT * FROM todos WHERE user_id = ? AND due = ?", user_id, date)
        .fetch_all(pool)
        .await
}

pub async fn create_todo(pool: &SqlitePool, user_id: &str, title: &str) -> Result<Todo, sqlx::Error> {
    let result = query_as!(Todo, "INSERT INTO todos (user_id, title) VALUES (?, ?)", user_id, title)
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

pub async fn delete_todo(pool: &SqlitePool, user_id: &str, todo_id: i64) -> Result<(), sqlx::Error> {
    query_as!(Todo, "DELETE from todos where user_id=? and id=?", user_id, todo_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn toggle_todo(pool: &SqlitePool, user_id: &str, todo_id: i64) -> Result<(), sqlx::Error> {
    query_as!(Todo, "UPDATE todos SET completed = NOT completed where user_id=? and id=?", user_id, todo_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn save_tags(pool: &SqlitePool, user_id: &str, todo_id: i64, tags: Vec<String>) -> Result<(), sqlx::Error> {
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

pub async fn get_tags_for_todo(pool: &SqlitePool, user_id: &str, todo_id: i64) -> Result<Vec<String>, sqlx::Error> {

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
