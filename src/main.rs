mod utils;
mod models;
mod handlers;
mod repo;

use std::env;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use axum::{
    middleware, routing::{delete, get, post}, Router
};
use dotenv::dotenv;
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

use handlers::index::index;
use handlers::about::about;
use handlers::login::{login_page, login_handler, logout_handler, register_page, register_handler};
use handlers::todo::{create_todo, delete_todo, toggle_todo, get_todos, get_tags_for_todo, edit_todo, update_todo, delete_todo_from_edit, get_tags};
use handlers::insights::{insights_page, get_insight_data, new_insight, save_insight};
use handlers::auth::auth_middleware;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<SqlitePool>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    dotenv().ok();
    let environment = env::var("ENV").expect("ENV must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("ENV: {}", environment);
    println!("DATABASE_URL: {}", database_url);
    println!("SQLX_OFFLINE is set to: {}", env::var("SQLX_OFFLINE").unwrap());

    let store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    let pool = match get_db(&database_url).await {
        Ok(pool) => pool,
        Err(err) => {
            println!("Error connection to DB: {:?}", err);
            exit(1);
        }
    };

    let pool = Arc::new(pool);
    let public_routes = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/login", get(login_page))
        .route("/login", post(login_handler))
        .route("/logout", get(logout_handler))
        .route("/register", get(register_page))
        .route("/register", post(register_handler))
        .nest_service("/static", ServeDir::new("static"));

    let auth_routes = Router::new()
        .route("/todos/{id}", get(edit_todo))
        .route("/todos/{id}", post(update_todo))
        .route("/todos", post(create_todo))
        .route("/todos", get(get_todos))
        .route("/todos/{id}", delete(delete_todo))
        .route("/todos/_edit/{id}", delete(delete_todo_from_edit))
        .route("/todos/{id}/toggle", post(toggle_todo))
        .route("/todos/{id}/tags", get(get_tags_for_todo))
        .route("/tags", get(get_tags))
        .route("/insights", get(insights_page))
        .route("/insights/save", post(save_insight))
        .route("/insights/new", get(new_insight))
        .route("/insights/{id}/data", get(get_insight_data))
        .route_layer(middleware::from_fn(auth_middleware));

    let routes = public_routes
        .merge(auth_routes)
        .layer(session_layer)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}

async fn get_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations applied");

    Ok(pool)
}
