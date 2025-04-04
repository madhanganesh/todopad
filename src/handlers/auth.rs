use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use headers::{Cookie, HeaderMapExt};
use hyper::HeaderMap;
use jsonwebtoken::{DecodingKey, TokenData, Validation, decode};

use super::{Claims, SECRET};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    /*if let Ok(user_id) = validate_cookie(&req).await {
        req.extensions_mut().insert(CurrentUser { user_id });
        return Ok(next.run(req).await);
    }*/

    let headers: &HeaderMap = req.headers();
    if let Some(cookie) = headers.typed_get::<Cookie>() {
        if let Some(token) = cookie.get("auth_token") {
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(SECRET),
                &Validation::default(),
            );
            if let Ok(TokenData { claims, .. }) = token_data {
                req.extensions_mut().insert(super::CurrentUser {
                    user_id: claims.sub,
                });
                return Ok(next.run(req).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn validate_cookie(headers: &HeaderMap) -> Result<i64, StatusCode> {
    //pub async fn validate_cookie(req: &Request) -> Result<String, StatusCode> {
    //let headers: &HeaderMap = req.headers();

    if let Some(cookie) = headers.typed_get::<Cookie>() {
        if let Some(token) = cookie.get("auth_token") {
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(SECRET),
                &Validation::default(),
            );
            if let Ok(TokenData { claims, .. }) = token_data {
                return Ok(claims.sub);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
