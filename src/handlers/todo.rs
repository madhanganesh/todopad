use std::{collections::HashMap, sync::Arc};
use askama::Template;
use axum::{
    Extension, Form, Json,
    extract::{Path, Query, State}, 
    http::HeaderMap, 
    response::{Html, IntoResponse, Response}, 
};
use chrono::{DateTime, Days, Datelike, NaiveDate, Utc, Weekday};
use chrono_tz::Tz;
use hyper::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use tower_sessions::Session;

use crate::{models::Todo, repo::todo::delete_todo_tag};
use crate::repo;
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
    pub timezone: String,
}

pub async fn create_todo(
    session: Session,
    Extension(user): Extension<CurrentUser>,
    Extension(timezone): Extension<String>,
    State(pool): State<Arc<SqlitePool>>, 
    Form(form): Form<TodoInputForm>,
) -> Response {

    let filter: String = session
        .get("filter")
        .await
        .unwrap()
        .unwrap_or("pending".to_string());

    let (due, show_date) = super::get_date_and_show_date(&filter, &timezone); 

    match repo::todo::create_todo(&pool, user.user_id, &form.title, &due).await {
        Ok(todo) => {
            let template = TodoTemplate { todo: &todo, show_date, timezone: timezone.clone() };
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
    pub timezone: String,
}

pub async fn get_todos(
    session: Session,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Query(query_params): Query<HashMap<String, String>>,
    Extension(timezone): Extension<String>,
) -> impl IntoResponse {

    let filter: &str = query_params.get("filter")
        .map(String::as_str)
        .unwrap_or("pending");

    let (todos, show_date) = get_todos_and_show_date(
        filter, &pool, user.user_id, &timezone).await;
    session.insert("filter", filter).await.unwrap();
    let template = TodosTemplate { 
        todos, 
        show_date, 
        timezone,
    };
    super::HtmlTemplate(template).into_response()
}

pub async fn delete_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> StatusCode {
    match repo::todo::delete_todo(&pool, user.user_id, id).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("Error: in delete todo {}. {:?}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn toggle_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> StatusCode {
    match repo::todo::toggle_todo(&pool, user.user_id, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_tags_for_todo(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Path(id): Path<i64>,  
) -> Result<Json<Vec<String>>, axum::response::Response> {

    match repo::todo::get_tags_for_todo(&pool, user.user_id, id).await {
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

    let tags = repo::todo::get_tags_for_todo(pool, user_id, todo_id).await.unwrap_or(vec![]);
    match repo::todo::get_todo(pool, user_id, todo_id).await {
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
    pub effort: f64,
    pub notes: Option<String>,
    pub tags: String, 
}

pub async fn update_todo(
    headers: HeaderMap,
    Path(todo_id): Path<i64>,
    State(pool): State<Arc<SqlitePool>>,
    Extension(user): Extension<CurrentUser>,
    Extension(timezone): Extension<String>,
    Form(form): Form<TodoEditForm>,
) -> Response {

    let due = match form.due_date {
        Some(ref date_str) if !date_str.is_empty() => {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
        }
        _ => {
            let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
            let now_in_tz: DateTime<Tz> = Utc::now().with_timezone(&tz);
            Some(now_in_tz.date_naive())
        }
    };

    let todo = Todo {
        id: todo_id,
        user_id: user.user_id,
        title: form.title.clone(),
        due,
        effort: form.effort,
        completed: form.completed.is_some(),
        notes: form.notes.clone(),
    };

    let result = repo::todo::update_todo( &pool, &todo).await;

    match result {
        Ok(_) => {
            update_tags(&pool, user.user_id, todo_id, String::from(&form.title), &form.tags).await;
            /*Response::builder()
                .status(StatusCode::OK)
                .header("HX-Location", HeaderValue::from_static("/"))
                .body(axum::body::Body::empty())
                .unwrap()*/

            let path = "<script>window.location.href='/';</script>";
            Html(path).into_response()

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
    let result = repo::todo::delete_todo(&pool, user.user_id, todo_id).await;
    match result {
        Ok(_) => {
            /*Response::builder()
                .status(StatusCode::OK)
                .header("HX-Location", HeaderValue::from_static("/"))
                .body(axum::body::Body::empty())
                .unwrap()*/
            let path = "<script>window.location.href='/';</script>";
            Html(path).into_response()
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

    if let Ok(current_tags) = repo::todo::get_tags_for_todo(pool, user_id, todo_id).await {
        if current_tags == tags {
            spawn_get_tags_and_save(pool, user_id, todo_id, title);
        } else {
            match repo::todo::save_tags(pool, user_id, todo_id, tags).await {
                Ok(_) => (),
                Err(err) => eprintln!("Error while saving tags: {:?}", err)
            }
        }
    }
}

#[derive(Serialize)]
struct TagResponse {
    tags: Vec<String>,
}

pub async fn get_tags(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>
) -> impl IntoResponse {
    let tags = repo::todo::get_tags(&pool, user.user_id).await.unwrap();

    Json(TagResponse { tags} )
}

#[derive(Serialize)]
struct TodoTrends {
    hours: f64,
}

pub async fn get_todos_trends(
    session: Session,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Extension(timezone): Extension<String>,
) -> impl IntoResponse {

    let filter: String = session
        .get("filter")
        .await
        .unwrap()
        .unwrap_or("pending".to_string());


    let (todos, _) = get_todos_and_show_date(
        &filter, 
        &pool, 
        user.user_id, 
        &timezone).await;

    Json(TodoTrends {
        hours: todos.iter().map(|t| t.effort).sum(),
    })
}

#[derive(Deserialize)]
pub struct UpdateEffortRequest {
    pub change: f64,  
}

pub async fn update_effort(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Path(todo_id): Path<i64>,
    Form(form): Form<UpdateEffortRequest>,
) -> impl IntoResponse {
    
    let mut todo = repo::todo::get_todo(&pool, user.user_id, todo_id).await.unwrap();
    let new_effort = (todo.effort + form.change).max(0.5); 
    todo.effort = new_effort;
    repo::todo::update_todo(&pool, &todo).await.unwrap();

    let response_html = format!(
        r#"<span id="effort-{}">{:.1}</span>"#,
        todo_id, new_effort
    );

    Html(response_html)
}

#[derive(Deserialize)]
pub struct UpdateDueRequest {
    pub change: i64,  
}

pub async fn update_due(
    Extension(user): Extension<CurrentUser>,
    Extension(timezone): Extension<String>,
    State(pool): State<Arc<SqlitePool>>, 
    Path(todo_id): Path<i64>,
    Form(form): Form<UpdateDueRequest>,
) -> impl IntoResponse {
    
    let mut todo = repo::todo::get_todo(&pool, user.user_id, todo_id).await.unwrap();
    let new_due = if form.change >= 0 {
        next_weekday(todo.due.unwrap())
    } else {
        previous_weekday(todo.due.unwrap())
    };
    todo.due = Some(new_due);
    repo::todo::update_todo(&pool, &todo).await.unwrap();

    let response_html = format!(
        r#"<span id="due-{}">{}</span>"#,
        todo_id, todo.relative_due(&timezone)
    );

    Html(response_html)
}

fn next_weekday(mut date: NaiveDate) -> NaiveDate {
    date = date.checked_add_days(Days::new(1)).unwrap(); // Move to next day

    // If it's Saturday (Sat -> 6), move to Monday
    // If it's Sunday (Sun -> 7), move to Monday
    match date.weekday() {
        Weekday::Sat => date = date.checked_add_days(Days::new(2)).unwrap(),
        Weekday::Sun => date = date.checked_add_days(Days::new(1)).unwrap(),
        _ => (),
    }

    date
}

fn previous_weekday(mut date: NaiveDate) -> NaiveDate {
    date = date.checked_sub_days(Days::new(1)).unwrap(); // Move to previous day

    // If it's Sunday, move to Friday (-2 days)
    // If it's Saturday, move to Friday (-1 day)
    match date.weekday() {
        Weekday::Sun => date = date.checked_sub_days(Days::new(2)).unwrap(),
        Weekday::Sat => date = date.checked_sub_days(Days::new(1)).unwrap(),
        _ => (),
    }

    date
}

pub async fn delete_tag(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>, 
    Path((todo_id, tag)): Path<(i64, String)>,
) -> StatusCode {

    match delete_todo_tag(&pool, user.user_id, todo_id, &tag).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("Error: in delete tag {}. {:?}", &tag, err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
