use axum::{
    response::Html,
    routing::get,
    Router,
};
use askama::Template;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> Html<String> {
    let template = IndexTemplate;
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate;

async fn about() -> Html<String> {
    let template = AboutTemplate;
    Html(template.render().unwrap())
}
