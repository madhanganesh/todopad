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
    user: Option<String>,
    todos: Option<Vec<Todo>>,
}


#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    user: Option<String>,
}

pub async fn index(headers: HeaderMap, State(pool): State<Arc<SqlitePool>>) -> Response {
   if let Ok(user_id) = validate_cookie(&headers).await {
        let todos = repo::get_pending_todos(&pool, &user_id).await.unwrap();
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

/*pub async fn index1(headers: HeaderMap, State(pool): State<Arc<SqlitePool>>) -> Response {
    if let Some(cookie) = headers.typed_get::<Cookie>() {
        if let Some(token) = cookie.get("auth_token") {
            let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default());
            if let Ok(TokenData { claims, .. }) = token_data {
                let todos = repo::get_pending_todos(&pool, &claims.sub).await.unwrap();
                let template = IndexTemplate {
                    user: Some(claims.sub),
                    todos: Some(todos),
                };
                return super::HtmlTemplate(template).into_response();
            }
        }
    }

    let template = AboutTemplate {
        user: None,
    };
    super::HtmlTemplate(template).into_response()
}*/


