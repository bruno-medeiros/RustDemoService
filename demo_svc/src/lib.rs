pub mod accounts;
pub mod axum_example;

use crate::accounts::service::SqlAccountsService;
use axum::routing::post;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub async fn svc_main(port: u32, conn_url: String) -> anyhow::Result<()> {

    info!("Creating connection pool...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;
    let pool = Arc::new(pool);

    let accounts = SqlAccountsService::create(pool.clone()).await?;

    // Setup webapp:
    let state = Arc::new(AppState {
        accounts: Arc::new(Mutex::new(accounts)),
    });
    let app = Router::new()
        .route("/accounts", post(accounts::web::create_account))
        .route("/accounts/get_balance", post(accounts::web::get_balance))
        .route("/accounts/deposit", post(accounts::web::deposit))
        .route("/accounts/withdraw", post(accounts::web::withdraw))
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
