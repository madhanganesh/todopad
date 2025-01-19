use std::sync::Arc;
use askama::Template;
use axum::{
    extract::State, http::HeaderMap, response::{IntoResponse, Response}
};

use sqlx::SqlitePool;

use super::{AboutTemplate, BaseTemplate, HtmlTemplate};
use super::auth::validate_cookie;
use crate::{models::Todo, repo};


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    base: BaseTemplate,
    todos: Option<Vec<Todo>>,
}

pub async fn index(headers: HeaderMap, State(pool): State<Arc<SqlitePool>>) -> Response {
   if let Ok(user_id) = validate_cookie(&headers).await {
        let todos = repo::get_pending_todos(&pool, user_id).await.unwrap();
        let template = IndexTemplate {
            base: BaseTemplate::new(headers).await,
            todos: Some(todos),
        };
        return HtmlTemplate(template).into_response();
    }

    let template = AboutTemplate {
        base: BaseTemplate::new(headers).await,
    };

    HtmlTemplate(template).into_response()
}
