
use std::{collections::HashMap, env, sync::Arc};

use askama::Template;
use axum::{extract::{Path, Query, State}, http::HeaderMap, response::{Html, IntoResponse, Redirect, Response}, Extension, Form, Json};
use chrono::{NaiveDate, Utc};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;

// Assuming the Todo struct is defined in the same module or needs to be imported
use crate::{models::Todo, repo::{self, get_pending_todos, get_todos_for_date, save_tags}, utils::tags::get_tags};

use super::{BaseTemplate, CurrentUser, HtmlTemplate, spawn_get_tags_and_save};

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
    match repo::create_todo(&pool, user.user_id, &form.title).await {
        Ok(todo) => {
            let template = TodoTemplate { todo: &todo };

            let openai_api_key = env::var("OPENAI_API_KEY");
            match openai_api_key {
                Ok(api_key) => {
                    let todo_id = todo.id;
                    let user_id = user.user_id;
                    let title = todo.title.clone();
                    let pool_clone = Arc::clone(&pool);
                    tokio::spawn(async move {
                        match get_tags(&api_key, &title).await {
                            Ok(tags) => {
                                 _ = save_tags(&pool_clone, user_id, todo_id, tags).await;
                            }
                            Err(err) => eprintln!("Error: {:?}", err),
                        }
                    });
                },
                Err(_) => {
                    println!("OPENAI_API_KEY is not set so tags are not idetified");
                }
            }

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
    let yesterday = today.pred_opt().unwrap();

    let todos = match filter {
        "pending" => get_pending_todos(&pool, user.user_id).await.unwrap(),
        "today" => get_todos_for_date(&pool, user.user_id, &today).await.unwrap(),
        "yesterday" => get_todos_for_date(&pool, user.user_id, &yesterday).await.unwrap(),
        "tomorrow" => get_todos_for_date(&pool, user.user_id, &tomorrow).await.unwrap(),
        _ =>  get_pending_todos(&pool, user.user_id).await.unwrap(),
    };

    let template = TodosTemplate { todos };
    super::HtmlTemplate(template).into_response()
}

pub async fn delete_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> StatusCode {
    match repo::delete_todo(&pool, user.user_id, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}


pub async fn toggle_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> StatusCode {
    match repo::toggle_todo(&pool, user.user_id, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_tags_for_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Path(id): Path<i64>,  
) -> Result<Json<Vec<String>>, axum::response::Response> {
    match repo::get_tags_for_todo(&pool, user.user_id, id).await {
        Ok(tags) => Ok(Json(tags)), // Return tags as JSON
        Err(err) => {
            eprintln!("Error fetching tags: {:?}", err);
            Err(axum::response::Response::builder()
                .status(500)
                .body("Error fetching tags".into())
                .unwrap())
        }
    }
}

#[derive(Template)]
#[template(path = "todo_edit.html")]
struct EditTodoTemplate {
    base: BaseTemplate,
    todo: Todo,
    tags: Vec<String>,
    error: Option<String>,
}

async fn get_todo_response(
    headers: HeaderMap,
    pool: &SqlitePool, 
    user_id: i64, 
    todo_id: i64, 
    err: Option<String>
) -> Response {

    let tags = repo::get_tags_for_todo(pool, user_id, todo_id).await.unwrap_or(vec![]);
    match repo::get_todo(pool, user_id, todo_id).await {
        Ok(todo) => {
            let template = EditTodoTemplate {
                base: BaseTemplate::new(headers).await,
                todo,
                tags, 
                error: err
            };

            HtmlTemplate(template).into_response()
        },
        Err(err) => {
            eprintln!("Error getting a todo: {:?}", err);
            axum::response::Response::builder()
                .status(500)
                .body("Error fetching todo".into())
                .unwrap()
        }
    }
}

pub async fn edit_todo(
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
    Path(todo_id): Path<i64>,
    State(pool): State<Arc<SqlitePool>>,
) -> Response {
    
    get_todo_response(
        headers, 
        &pool, 
        user.user_id, 
        todo_id, 
        None
    ).await
}

#[derive(Debug, Deserialize)]
pub struct TodoEditForm {
    pub title: String,
    pub due_date: Option<String>,
    pub completed: Option<String>,
    pub notes: Option<String>,
    pub tags: String, 
}

pub async fn update_todo(
    headers: HeaderMap,
    Path(todo_id): Path<i64>,
    State(pool): State<Arc<SqlitePool>>,
    Extension(user): Extension<CurrentUser>,
    Form(form): Form<TodoEditForm>,
) -> Response {

    let due = match form.due_date {
        Some(ref date_str) if !date_str.is_empty() => {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
        }
        _ => None,
    };

    let result = repo::update_todo(
        &pool, 
        user.user_id, 
        todo_id, form.title.clone(), 
        due, 
        form.completed.is_some(), 
        form.notes
    ).await;


    match result {
        Ok(_) => {
            update_tags(&pool, user.user_id, todo_id, String::from(&form.title), &form.tags).await;
            Redirect::to("/").into_response()
        }, 
        Err(err) => {
            eprintln!("Error getting a todo: {:?}", err);
            get_todo_response(
                headers, 
                &pool, 
                user.user_id, 
                todo_id, 
                Some("Failed to update the todo item. Please try again.".to_string())
            ).await
        }
    }
}

pub async fn delete_todo_from_edit(
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(todo_id): Path<i64>,
) -> Response {
    let result = repo::delete_todo(&pool, user.user_id, todo_id).await;
    match result {
        Ok(_) => {
            Redirect::to("/").into_response()
        }, 
        Err(err) => {
            eprintln!("Error getting a todo: {:?}", err);
            get_todo_response(
                headers, 
                &pool, 
                user.user_id, 
                todo_id, 
                Some("Failed to update the todo item. Please try again.".to_string())
            ).await
        }
    }
}

async fn update_tags(pool: &SqlitePool, user_id: i64, todo_id: i64, title: String, tags_str: &str)  {
    let tags: Vec<String> = tags_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if let Ok(current_tags) = repo::get_tags_for_todo(pool, user_id, todo_id).await {
        if current_tags == tags {
            spawn_get_tags_and_save(pool, user_id, todo_id, title);
        } else {
            match repo::save_tags(pool, user_id, todo_id, tags).await {
                Ok(_) => (),
                Err(err) => eprintln!("Error while saving tags: {:?}", err)
            }
        }
    }
}

