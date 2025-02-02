use std::sync::Arc;
use askama::Template;
use axum::{
    extract::State, http::HeaderMap, response::{IntoResponse, Response}, Extension
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
    timezone: String,
}

pub async fn index(
    session: Session,
    headers: HeaderMap, 
    State(pool): State<Arc<SqlitePool>>,
    Extension(timezone): Extension<String>,
) -> Response {

    let filter: String = session.get("filter").await.unwrap().unwrap_or("pending".to_string());
    if let Ok(user_id) = validate_cookie(&headers).await {
        let (todos, show_date) = get_todos_and_show_date(
            &filter, 
            &pool, 
            user_id, 
            &timezone).await;

        let template = IndexTemplate {
            base: BaseTemplate::new(headers).await,
            todos: Some(todos),
            show_date,
            filter,
            timezone: timezone.clone(),
        };
        return HtmlTemplate(template).into_response();
    }

    let template = AboutTemplate {
        base: BaseTemplate::new(headers).await,
    };

    HtmlTemplate(template).into_response()
}
