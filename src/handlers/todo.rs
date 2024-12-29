
use std::{collections::HashMap, sync::Arc};

use askama::Template;
use axum::{extract::{Path, Query, State}, response::{Html, IntoResponse, Response}, Extension, Form};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;

// Assuming the Todo struct is defined in the same module or needs to be imported
use crate::{models::Todo, repo::{self, get_pending_todos, get_todos_for_date}};

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
            super::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            Html("<p>Error creating todo</p>".to_string()).into_response()
        }
    }
}

#[derive(Template)]
#[template(path="partials/todos.html")]
pub struct TodosTemplate {
    pub todos: Vec<Todo>,
}

pub async fn get_todos(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    query_params: Option<Query<HashMap<String, String>>>,
) -> Response {
    let filter = query_params
        .as_ref()
        .and_then(|q| q.get("filter").map(String::as_str))
        .unwrap_or("pending");

    let today = Utc::now().naive_utc().date();
    let tomorrow = today.succ_opt().unwrap();

    let todos = match filter {
        "pending" => get_pending_todos(&pool, &user.user_id).await.unwrap(),
        "today" => get_todos_for_date(&pool, &user.user_id, &today).await.unwrap(),
        "tomorrow" => get_todos_for_date(&pool, &user.user_id, &tomorrow).await.unwrap(),
        _ =>  get_pending_todos(&pool, &user.user_id).await.unwrap(),
    };

    let template = TodosTemplate { todos };
    super::HtmlTemplate(template).into_response()
}

pub async fn delete_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> StatusCode {
    match repo::delete_todo(&pool, &user.user_id, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn toggle_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> StatusCode {
    match repo::toggle_todo(&pool, &user.user_id, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
