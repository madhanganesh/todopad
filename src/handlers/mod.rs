pub mod about;
pub mod auth;
pub mod index;
pub mod login;
pub mod todo;

use askama::Template;
use axum::{
    http::{StatusCode, HeaderMap},
    response::{Html, IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use auth::validate_cookie;

const SECRET: &[u8] = b"my_secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
}

#[derive(Clone)]
pub struct CurrentUser {
    user_id: i64,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            ).into_response(),
        }
    }
}

struct NavItem {
    name: String,
    url: String,
}

struct BaseTemplate {
    navs: Vec<NavItem>,
}

impl BaseTemplate {
    async fn new(headers: HeaderMap) -> Self {
        let mut navs =  vec![
            NavItem{name: "todos".to_string(), url: "/".to_string()},
            NavItem{name: "about".to_string(), url: "/about".to_string()},
            NavItem{name: "login".to_string(), url: "/login".to_string()},
        ];

        if validate_cookie(&headers).await.is_ok() {
            navs =  vec![
                NavItem{name: "todos".to_string(), url: "/".to_string()},
                NavItem{name: "about".to_string(), url: "/about".to_string()},
                NavItem{name: "logout".to_string(), url: "/logout".to_string()},
            ];
        }

        BaseTemplate { navs }
    }
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    base: BaseTemplate,
}

use std::env;
use sqlx::SqlitePool;
use crate::utils::tags::get_tags;
use crate::repo::save_tags;

fn spawn_get_tags_and_save(pool: &SqlitePool, user_id: i64, todo_id: i64, title: String) {
    let openai_api_key = env::var("OPENAI_API_KEY");
    match openai_api_key {
        Ok(api_key) => {
            let pool_clone = pool.clone();
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
}
