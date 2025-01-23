use std::sync::Arc;
use askama::Template;
use axum::{
    extract::State, http::HeaderMap, response::{IntoResponse, Response}
};

use sqlx::SqlitePool;
use tower_sessions::Session;

use super::{AboutTemplate, BaseTemplate, HtmlTemplate};
use super::auth::validate_cookie;
use crate::{handlers::get_todos_and_show_date, models::Todo};


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    base: BaseTemplate,
    todos: Option<Vec<Todo>>,
    show_date: bool,
    filter: String,
}

pub async fn index(
    session: Session,
    headers: HeaderMap, 
    State(pool): State<Arc<SqlitePool>>)
-> Response {
   
    let filter: String = session.get("filter").await.unwrap().unwrap_or("pending".to_string());
    if let Ok(user_id) = validate_cookie(&headers).await {
        let (todos, show_date) = get_todos_and_show_date(&filter, &pool, user_id).await;

        let template = IndexTemplate {
            base: BaseTemplate::new(headers).await,
            todos: Some(todos),
            show_date,
            filter,
        };
        return HtmlTemplate(template).into_response();
    }

    let template = AboutTemplate {
        base: BaseTemplate::new(headers).await,
    };

    HtmlTemplate(template).into_response()
}
