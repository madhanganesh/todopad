use axum::response::IntoResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    user: Option<String>,
}

pub async fn about() -> impl IntoResponse {
    let template = AboutTemplate {user: None};
    super::HtmlTemplate(template)
}



