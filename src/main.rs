mod utils;
mod models;
mod handlers;
mod repo;

use dotenv::from_filename;
use std::env;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use axum::{
    middleware, routing::{get, post}, Router
};
use tower_http::services::ServeDir;
use sqlx::Executor;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

use handlers::index::index;
use handlers::about::about;
use handlers::login::{login_page, login_handler, logout_handler};
use handlers::todo::create_todo;
use handlers::auth::auth_middleware;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<SqlitePool>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    /*dotenv().ok();
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");*/

    // Determine the environment
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    println!("APP_ENV: {}", environment);

    // Load the appropriate .env file
    match environment.as_str() {
        "release" => from_filename(".env.release").ok(),
        _ => from_filename(".env").ok(),
    };
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);
    println!("SQLX_OFFLINE is set to: {}", env::var("SQLX_OFFLINE").unwrap());

    let pool = match get_db(&environment, &database_url).await {
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
        .nest_service("/static", ServeDir::new("static"));

    let auth_routes = Router::new()
        .route("/todos", post(create_todo))
        .route_layer(middleware::from_fn(auth_middleware));

    let routes = public_routes
        .merge(auth_routes)
        .with_state(pool);
    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}

async fn get_db(environment: &str, database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations applied");

    if environment == "development" {
        seed_dev_data(&pool).await?;
        println!("Development seed data applied");
    }

    Ok(pool)
}

async fn seed_dev_data(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let seed_script = include_str!("../seeds/seed_dev_data.sql");
    pool.execute(seed_script).await?;
    Ok(())
}
