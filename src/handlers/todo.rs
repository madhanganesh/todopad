
use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::{Html, IntoResponse, Response}, Extension, Form};
use serde::Deserialize;
use sqlx::{query, query_as, Pool, Sqlite, SqlitePool};

// Assuming the Todo struct is defined in the same module or needs to be imported
use crate::{models::Todo, repo};

use super::CurrentUser;

#[derive(Deserialize)]
pub struct TodoInputForm {
    pub title: String,
}

#[derive(Template)]
#[template(path = "partials/todo.html")]
pub struct TodoTemplate<'a> {
    pub todo: &'a Todo,
}

pub async fn create_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Form(form): Form<TodoInputForm>) -> Response {
    match repo::create_todo(&pool, &user.user_id, &form.title).await {
        Ok(todo) => {
            let template = TodoTemplate { todo: &todo };
            return super::HtmlTemplate(template).into_response();
        }
        Err(_) => {
            return Html("<p>Error creating todo</p>".to_string()).into_response();
        }
    }
}