pub mod about;
pub mod auth;
pub mod index;
pub mod login;
pub mod todo;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

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
            )
                .into_response(),
        }
    }
}
