use crate::accounts::api::{AccountsApi, DepositResult, GetBalanceResult, WithdrawResult};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tx_model::AccountId;
use uuid::Uuid;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateAccount {
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAccountResponse {
    // #[serde(with = "uuid::serde::simple")]
    id: Uuid,
}

#[derive(Deserialize)]
pub struct DepositParams {
    account_id: AccountId,
    amount: u32,
}

#[derive(Deserialize)]
pub struct WithdrawParams {
    account_id: AccountId,
    amount: u32,
}

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccount>,
) -> Response {
    let mut accounts = state.accounts.lock().await;

    let result = accounts.create_account(&payload.description).await;
    match result {
        Ok(id) => {
            let response = CreateAccountResponse { id };
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
        Ok(GetBalanceResult::Ok(balance)) => {
            (StatusCode::OK, Json(balance)).into_response()
        }
        Ok(GetBalanceResult::AccountNotFound(account_id)) => {
            (StatusCode::NOT_FOUND, Json(account_id)).into_response()
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
        Ok(DepositResult::Ok(balance)) => {
            (StatusCode::OK, Json(balance)).into_response()
        }
        Ok(DepositResult::AccountNotFound(account_id)) => {
            (StatusCode::NOT_FOUND, Json(account_id)).into_response()
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
        Ok(WithdrawResult::Ok(balance)) => {
            (StatusCode::OK, Json(balance)).into_response()
        }
        Ok(WithdrawResult::AccountNotFound(account_id)) => {
            (StatusCode::NOT_FOUND, Json(account_id)).into_response()
        }
        Ok(WithdrawResult::NotEnoughBalance(balance)) => {
            (StatusCode::BAD_REQUEST, format!("Not enough balance: {balance}")).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}