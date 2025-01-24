use std::{collections::HashMap, sync::Arc};
use askama::Template;
use axum::{
    Extension, Form, Json,
    extract::{Path, Query, State}, 
    http::{HeaderMap, HeaderValue}, 
    response::{Html, IntoResponse, Response}, 
};
use chrono::NaiveDate;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;
use tower_sessions::Session;

use crate::{models::Todo, repo::{self}};
use super::{get_todos_and_show_date, spawn_get_tags_and_save, BaseTemplate, CurrentUser, HtmlTemplate};

#[derive(Deserialize)]
pub struct TodoInputForm {
    pub title: String,
}

#[derive(Template)]
#[template(path = "partials/todo.html")]
pub struct TodoTemplate<'a> {
    pub todo: &'a Todo,
    pub show_date: bool,
}

pub async fn create_todo(
    session: Session,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Form(form): Form<TodoInputForm>) -> Response {

    let filter: String = session.get("filter").await.unwrap().unwrap_or("pending".to_string());
    let (due, show_date) = super::get_date_and_show_date(&filter); 

    match repo::create_todo(&pool, user.user_id, &form.title, &due).await {
        Ok(todo) => {
            let template = TodoTemplate { todo: &todo, show_date };
            spawn_get_tags_and_save(&pool, user.user_id, todo.id, todo.title.clone());
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
    pub show_date: bool,
}

pub async fn get_todos(
    session: Session,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Query(query_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {

    let filter: &str = query_params.get("filter")
        .map(String::as_str)
        .unwrap_or("pending");

    let (todos, show_date) = get_todos_and_show_date(filter, &pool, user.user_id).await;
    session.insert("filter", filter).await.unwrap();
    let template = TodosTemplate { todos, show_date };
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
        Ok(tags) => Ok(Json(tags)), 
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
            Response::builder()
                .status(StatusCode::OK)
                .header("HX-Location", HeaderValue::from_static("/"))
                .body(axum::body::Body::empty())
                .unwrap()
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
            Response::builder()
                .status(StatusCode::OK)
                .header("HX-Location", HeaderValue::from_static("/"))
                .body(axum::body::Body::empty())
                .unwrap()
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
