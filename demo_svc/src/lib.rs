pub mod accounts;

use crate::accounts::service::SqlAccountsService;
use crate::accounts::web::CreateAccount;
use axum::extract::State;
use axum::routing::post;
use axum::{routing::get, Json, Router};
use axum_macros::debug_handler;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::types::ToSql;

#[tokio::main]
pub async fn svc_main() -> Result<(), Box<dyn Error>> {

    let conn_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:example@localhost:5432/postgres".to_owned());

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;
    let pool = Arc::new(pool);
    let accounts = SqlAccountsService::create(pool.clone()).await?;
    let state = Arc::new(AppState { accounts : Arc::new(Mutex::new(accounts))});

    setup_routes(state).await
}

async fn setup_routes(state: Arc<AppState>) -> Result<(), Box<dyn Error>> {

    let app = Router::new()
        .route("/", get(hello_endpoint))
        .route("/foo", get(handler2))
        .route("/accounts", post(accounts::web::create_account))
        .with_state(state)
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    accounts : Arc<Mutex<SqlAccountsService>>,
}

async fn hello_endpoint() -> &'static str {
    "Hello, World!\n"
}


#[debug_handler]
async fn handler2(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccount>,
) -> &'static str {
    "Hello, World!\n"
}
