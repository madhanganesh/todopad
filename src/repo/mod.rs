use sqlx::{query_as, SqlitePool};

use crate::models::Todo;


pub async fn get_pending_todos(pool: &SqlitePool, user_id: &str) -> Result<Vec<Todo>, sqlx::Error> {
    query_as!(Todo, "SELECT * FROM todos WHERE user_id = ? AND completed = false", user_id)
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
