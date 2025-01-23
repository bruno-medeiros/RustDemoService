pub mod accounts;
pub mod axum_example;

use crate::accounts::service::SqlAccountsService;
use axum::routing::post;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[tokio::main]
pub async fn svc_main(port: u32) -> Result<(), Box<dyn Error>> {
    let conn_url = std::env::var("DATABASE_URL")?;

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;
    let pool = Arc::new(pool);
    let accounts = SqlAccountsService::create(pool.clone()).await?;
    let state = Arc::new(AppState {
        accounts: Arc::new(Mutex::new(accounts)),
    });

    setup_routes(port, state).await?;
    Ok(())
}

async fn setup_routes(port: u32, state: Arc<AppState>) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/accounts", post(accounts::web::create_account))
        .route("/accounts/get_balance", post(accounts::web::get_balance))
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");
    info!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    accounts: Arc<Mutex<SqlAccountsService>>,
}
