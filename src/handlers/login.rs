use std::{fmt::Debug, sync::Arc};
use askama::Template;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Response},
    http::{header, HeaderMap, HeaderValue, StatusCode}
};
use serde::Deserialize;
use sqlx::SqlitePool;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{repo::{self, todo::get_user_from_email}, utils::{self, verify_password}};
use super::{BaseTemplate, Claims};

const SECRET: &[u8] = b"my_secret_key";

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
    base: BaseTemplate,
}

pub async fn login_page(headers: HeaderMap) -> impl IntoResponse {
    let template = LoginTemplate { error: None, base: BaseTemplate::new(headers).await};
    super::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

pub async fn login_handler(headers: HeaderMap, 
                            pool: State<Arc<SqlitePool>>, 
                            Form(form): Form<LoginForm>) -> Response {
    let user = get_user_from_email(&pool, &form.email).await;
    if let Ok(user) = user {
        if verify_password(&user.password_hash, &form.password) {
            return set_cookie_and_redirect(user.id);
        }
    }

    let template = LoginTemplate {
        base: BaseTemplate::new(headers).await,
        error: Some("Invalid username or password".to_string()),
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
    base: BaseTemplate,
    error: Option<String>,
}

pub async fn register_page(headers: HeaderMap) -> impl IntoResponse {
    let template = RegisterTemplate{error: None, base: BaseTemplate::new(headers).await};
    super::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    password: String,
}

pub async fn register_handler(
    headers: HeaderMap,
    State(pool): State<Arc<SqlitePool>>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let password_hash = match utils::hash_password(&form.password) {
        Ok(hash) => hash,
        Err(err) => { 
            return handle_registration_error(headers, err, "System error. Please contact administrator").await;
        }
    };

    match repo::todo::register_user(&pool, &form.email, &password_hash).await {
        Ok(user_id) => {
            set_cookie_and_redirect(user_id)
        }
        Err(err) => {
            let s = &err.to_string();
            handle_registration_error(headers, err, s).await
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

async fn handle_registration_error(headers: HeaderMap, err: impl Debug, msg: &str) -> Response {
    println!("Error in register user: {:?}", err);
    let template = RegisterTemplate {
        base: BaseTemplate::new(headers).await,
        error: Some(msg.to_string()),
    };
    super::HtmlTemplate(template).into_response()
}
