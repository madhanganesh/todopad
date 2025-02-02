pub mod about;
pub mod auth;
pub mod index;
pub mod login;
pub mod todo;
pub mod insights;

use std::env;
use askama::Template;
use axum::{
    http::{StatusCode, HeaderMap},
    response::{Html, IntoResponse, Response},
};
use chrono::{NaiveDate, Utc, DateTime};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use auth::validate_cookie;
use sqlx::SqlitePool;

use crate::{models::Todo, repo::todo::{get_pending_todos, get_todos_for_date}, utils::tags::get_tags};
use crate::repo::todo::save_tags;

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
                NavItem{name: "insights".to_string(), url: "/insights".to_string()},
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

async fn get_todos_and_show_date(
        filter: &str,
        pool: &SqlitePool,
        user_id: i64,
        timezone: &str,
) -> (Vec<Todo>, bool) {

    let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
    let now_in_tz: DateTime<Tz> = Utc::now().with_timezone(&tz);

    let today: NaiveDate = now_in_tz.date_naive();
    let tomorrow = today.succ_opt().unwrap();
    let yesterday = today.pred_opt().unwrap();

    match filter {
        "pending" => (get_pending_todos(pool, user_id).await.unwrap(), true),
        "today" => (get_todos_for_date(pool, user_id, &today).await.unwrap(), false),
        "yesterday" => (get_todos_for_date(pool, user_id, &yesterday).await.unwrap(), false),
        "tomorrow" => (get_todos_for_date(pool, user_id, &tomorrow).await.unwrap(), false),
        _ =>  (get_pending_todos(pool, user_id).await.unwrap(), true),
    }
}

fn get_date_and_show_date(filter: &str, timezone: &str) -> (NaiveDate, bool) {
    let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
    let now_in_tz: DateTime<Tz> = Utc::now().with_timezone(&tz);

    let today: NaiveDate = now_in_tz.date_naive();
    let tomorrow = today.succ_opt().unwrap();
    let yesterday = today.pred_opt().unwrap();

    match filter {
        "pending" => (today, true),
        "today" => (today, false),
        "tomorrow" => (tomorrow, false),
        "yesterday" => (yesterday, false),
        _ => (today, true),
    }
}

/*use axum::{
    extract::Request,
    middleware::Next,
};
use chrono_tz::Tz;
use std::sync::Arc;

/// Key for storing timezone in Axum's request extensions
#[derive(Clone)]
pub struct Timezone(String);

/// Middleware function to extract timezone and store it in request extensions
pub async fn timezone_middleware(
    cookies: Cookies,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get timezone from headers or cookies, fallback to UTC
    let user_timezone = headers.get("X-Timezone")
        .and_then(|tz| tz.to_str().ok())
        .or_else(|| cookies.get("timezone").map(|c| c.value()))
        .unwrap_or("UTC")
        .to_string();

    // Attach timezone to request extensions
    let mut request = request;
    request.extensions_mut().insert(Timezone(user_timezone));

    // Continue to the next middleware/handler
    Ok(next.run(request).await)
}*/

use axum::{
    extract::Request,
    middleware::Next,
};


pub async fn timezone_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check headers first (HTMX requests)
    let tz = request.headers()
        .get("X-Timezone")
        .and_then(|h| h.to_str().ok())
        .or_else(|| {
            // Check cookies
            request.headers()
                .get_all("Cookie")
                .iter()
                .find_map(|c| {
                    let cookie = c.to_str().ok()?;
                    cookie.split(';')
                        .find(|s| s.trim().starts_with("timezone="))
                        .and_then(|s| s.split('=').nth(1))
                })
        })
        .map(|s| s.to_string())
        .unwrap_or_else(|| "UTC".to_string()); // Final fallback

    request.extensions_mut().insert(tz);
    Ok(next.run(request).await)
}
