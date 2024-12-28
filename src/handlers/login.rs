use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Response},
    http::{header, HeaderValue, StatusCode}
};
use jsonwebtoken::{encode, EncodingKey, Header};


use serde::{Deserialize, Serialize};
use sqlx::{query, SqlitePool};

use crate::utils::{self, verify_password};

const SECRET: &[u8] = b"my_secret_key";

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
    user: Option<String>,
}

pub async fn login_page() -> impl IntoResponse {
    let template = LoginTemplate { error: None, user: None };
    super::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // username
    exp: usize,  // expiration timestamp
}

pub async fn login_handler(State(pool): State<Arc<SqlitePool>>, Form(form): Form<LoginForm>) -> Response {

    let result = query!("select hashed_password from users where id=?", form.username)
                    .fetch_one(&*pool)
                    .await;
    if let Ok(row) = result {
        if verify_password(&row.hashed_password, &form.password) {
            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(365))
                .unwrap()
                .timestamp() as usize;

            let claims = Claims {
                sub: form.username.clone(),
                exp: expiration,
            };

            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
                .expect("Failed to encode JWT");

            return Response::builder()
                .status(StatusCode::FOUND)
                .header(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&format!("auth_token={}; HttpOnly; Path=/", token)).unwrap(),
                )
                .header(header::LOCATION, HeaderValue::from_static("/"))
                .body(axum::body::Body::empty())
                .unwrap();
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
