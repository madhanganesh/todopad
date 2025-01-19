use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};

use super::{AboutTemplate, BaseTemplate, HtmlTemplate};

pub async fn about(headers: HeaderMap) -> Response {
    let template = AboutTemplate {
        base: BaseTemplate::new(headers).await,
    };
   HtmlTemplate(template).into_response()
}

