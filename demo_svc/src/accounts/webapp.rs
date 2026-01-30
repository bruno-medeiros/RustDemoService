use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::post;
use axum::{Json, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;
use tracing::info;

use crate::accounts::api::{
    AccountId, AccountsApi, CreateAccountParams, DepositParams, WithdrawParams,
};
use crate::accounts::service::SqlAccountsService;
use crate::app_util::{to_response, to_response_with_ok_status};
use crate::AppState;

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccountParams>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    #[cfg(feature = "test-utils")]
    if payload.description.contains("!!!RETURN_INTERNAL_ERROR") {
        return to_response::<()>(Err(anyhow::anyhow!("Instrumented Internal Server Error")));
    }

    to_response_with_ok_status(
        StatusCode::CREATED,
        accounts.create_account(&payload.description).await,
    )
}

pub async fn get_balance(
    State(state): State<Arc<AppState>>,
    Json(params): Json<AccountId>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    to_response(accounts.get_balance(&params).await)
}

pub async fn deposit(
    State(state): State<Arc<AppState>>,
    Json(params): Json<DepositParams>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    to_response(accounts.deposit(&params.account_id, params.amount).await)
}

pub async fn withdraw(
    State(state): State<Arc<AppState>>,
    Json(params): Json<WithdrawParams>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    to_response(accounts.withdraw(&params.account_id, params.amount).await)
}

pub fn create_webapp(accounts: SqlAccountsService) -> Router {
    let state = Arc::new(AppState {
        accounts: Arc::new(Mutex::new(accounts)),
    });
    Router::new()
        .route("/accounts/", post(create_account))
        .route("/accounts/get_balance", post(get_balance))
        .route("/accounts/deposit", post(deposit))
        .route("/accounts/withdraw", post(withdraw))
        .with_state(state)
}

pub async fn create_app(conn_url: &str) -> anyhow::Result<Router> {
    info!("Creating connection pool...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(conn_url)
        .await?;
    let pool = Arc::new(pool);

    let accounts = SqlAccountsService::create(pool.clone()).await?;

    let app = create_webapp(accounts);
    Ok(app)
}
