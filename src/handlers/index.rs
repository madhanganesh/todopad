use std::sync::Arc;
use askama::Template;
use axum::{
    extract::State, http::HeaderMap, response::{IntoResponse, Response}
};

use sqlx::SqlitePool;

use crate::{models::Todo, repo};
use super::auth::validate_cookie;


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    user: Option<i64>,
    todos: Option<Vec<Todo>>,
}


#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    user: Option<i64>,
}

pub async fn index(headers: HeaderMap, State(pool): State<Arc<SqlitePool>>) -> Response {
   if let Ok(user_id) = validate_cookie(&headers).await {
        let todos = repo::get_pending_todos(&pool, user_id).await.unwrap();
        let template = IndexTemplate {
            user: Some(user_id),
            todos: Some(todos),
        };
        return super::HtmlTemplate(template).into_response();
    }

    let template = AboutTemplate {
        user: None,
    };
    super::HtmlTemplate(template).into_response()
}
