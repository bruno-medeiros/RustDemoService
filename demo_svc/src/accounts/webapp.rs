use crate::accounts::api::{AccountsApi, CreateAccountParams, DepositParams, WithdrawParams};
use crate::accounts::service::SqlAccountsService;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use tx_model::AccountId;


pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccountParams>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    let result = accounts.create_account(&payload.description).await;
    match result {
        Ok(response) => {
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn get_balance(
    State(state): State<Arc<AppState>>,
    Json(params): Json<AccountId>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    let result = accounts.get_balance(&params).await;
    match result {
        Ok(response) => {
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}


pub async fn deposit(
    State(state): State<Arc<AppState>>,
    Json(params): Json<DepositParams>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    let result = accounts.deposit(&params.account_id, params.amount).await;

    match result {
        Ok(response) => {
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn withdraw(
    State(state): State<Arc<AppState>>,
    Json(params): Json<WithdrawParams>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    let result = accounts.withdraw(&params.account_id, params.amount).await;

    match result {
        Ok(response) => {
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub fn create_webapp(accounts: SqlAccountsService) -> Router {
    let state = Arc::new(AppState {
        accounts: Arc::new(Mutex::new(accounts)),
    });
    let app = Router::new()
        .route("/accounts/", post(create_account))
        .route("/accounts/get_balance", post(get_balance))
        .route("/accounts/deposit", post(deposit))
        .route("/accounts/withdraw", post(withdraw))
        .with_state(state);
    app
}

pub async fn create_app(conn_url: &String) -> anyhow::Result<Router> {
    info!("Creating connection pool...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;
    let pool = Arc::new(pool);

    let accounts = SqlAccountsService::create(pool.clone()).await?;

    let app = create_webapp(accounts);
    Ok(app)
}