use std::{fmt::Debug, sync::Arc};
use askama::Template;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Response},
    http::{header, HeaderValue, StatusCode}
};
use serde::Deserialize;
use sqlx::SqlitePool;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{repo::{self, get_user_from_email}, utils::{self, verify_password}};
use super::Claims;

const SECRET: &[u8] = b"my_secret_key";

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
    user: Option<i64>,
}

pub async fn login_page() -> impl IntoResponse {
    let template = LoginTemplate { error: None, user: None };
    super::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

pub async fn login_handler(State(pool): State<Arc<SqlitePool>>, Form(form): Form<LoginForm>) -> Response {
    let user = get_user_from_email(&pool, &form.email).await;
    if let Ok(user) = user {
        if verify_password(&user.password_hash, &form.password) {
            return set_cookie_and_redirect(user.id);
        }
    }

    let template = LoginTemplate {
        error: Some("Invalid username or password".to_string()),
        user: None,
    };
    super::HtmlTemplate(template).into_response()
}

pub async fn logout_handler() -> impl IntoResponse {
     Response::builder()
        .status(StatusCode::FOUND)
        .header(
            header::SET_COOKIE,
            HeaderValue::from_str(&format!("auth_token={}; HttpOnly; Path=/", "")).unwrap(),
        )
        .header(header::LOCATION, HeaderValue::from_static("/login"))
        .body(axum::body::Body::empty())
        .unwrap()
}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {
    user: Option<i64>,
    error: Option<String>,
}

pub async fn register_page() -> impl IntoResponse {
    let template = RegisterTemplate{user: None, error: None};
    super::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    password: String,
}

pub async fn register_handler(
    State(pool): State<Arc<SqlitePool>>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let password_hash = match utils::hash_password(&form.password) {
        Ok(hash) => hash,
        Err(err) => { 
            return handle_registration_error(err, "System error. Please contact administrator");
        }
    };

    match repo::register_user(&pool, &form.email, &password_hash).await {
        Ok(user_id) => {
            set_cookie_and_redirect(user_id)
        }
        Err(err) => {
            let s = &err.to_string();
            handle_registration_error(err, s)
        }
    }
}

fn set_cookie_and_redirect(user_id: i64) -> Response {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(365))
        .unwrap();

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
        .expect("Failed to encode JWT");

    let cookie = format!(
        "auth_token={}; HttpOnly; Path=/; Expires={}",
        token,
        expiration.to_rfc2822(),
    );

    Response::builder()
        .status(StatusCode::FOUND)
        .header(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie).unwrap(),
        )
        .header(header::LOCATION, HeaderValue::from_static("/"))
        .body(axum::body::Body::empty())
        .unwrap()
}

fn handle_registration_error(err: impl Debug, msg: &str) -> Response {
    println!("Error in register user: {:?}", err);
    let template = RegisterTemplate {
        error: Some(msg.to_string()),
        user: None,
    };
    super::HtmlTemplate(template).into_response()
}
